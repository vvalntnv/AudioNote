use actix_web::web;
use bson::{doc, oid::ObjectId};
use chrono::Utc;
use mongodb::Collection;
use crate::{database::database_models::{book::Book, stream_data::StreamData}, AppState};

use super::{errors::StreamError, external_models::{RefreshStreamRequest, RefreshStreamResponse, StreamGeneratedResponse}};

pub fn stream_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/generate/{book_id}")
            .route(web::post().to(generate_stream_link))
    );

    cfg.service(
        web::resource("/refresh/{stream_id}")
            .route(web::post().to(refresh_stream))
    );
}
type StreamResult<T> = Result<T, StreamError>;


pub async fn generate_stream_link(
    book_id: web::Path<String>,
    app_data: web::Data<AppState>
) -> StreamResult<web::Json<StreamGeneratedResponse>> 
{  
    let book_id = book_id.into_inner();

    let stream_collection = app_data.db.get_collection::<StreamData>();
    let book_collection = app_data.db.get_collection::<Book>();
    let book_object_id = ObjectId::parse_str(&book_id)
        .map_err(|err| {
            StreamError::GenericStreamError { details: err.to_string() }
        })?;
        
    match book_collection.find_one(doc! {"_id": book_object_id}).await {
        Ok(_) => (),
        Err(err) => return Err(StreamError::GenericStreamError { details: err.to_string() })
    } 

    let stream_data = StreamData::from(book_id);

    match stream_collection.insert_one(&stream_data).await {
        Ok(_) => Ok(web::Json(stream_data.into())),
        Err(err) => return Err(StreamError::GenericStreamError { details: err.to_string() })
    }
}

pub async fn refresh_stream(
    stream_id: web::Path<String>,
    refresh_stream_request: web::Json<RefreshStreamRequest>,
    app_data: web::Data<AppState>
) -> StreamResult<web::Json<RefreshStreamResponse>>
{
    let stream_id = stream_id.into_inner();
    let refresh_stream_request = refresh_stream_request.into_inner();

    let stream_collection = app_data.db.get_collection::<StreamData>();
    let mut stream = get_stream(&stream_collection, &stream_id).await?; 

    let refresh_token = refresh_stream_request.refresh_token;

    if let Err(err) = stream.refresh_stream(refresh_token) {
        Err(StreamError::GenericStreamError { details: err.to_string() })
    } else {
        let update_doc = stream.create_update_doc();
        let query = doc! {"stream_id": stream_id};

        match stream_collection.update_one(query, update_doc).await {
            Ok(_) => Ok(
                web::Json(
                    RefreshStreamResponse {
                        message: "Stream refreshed successfully".to_string(),
                        refresh_token:  stream.get_refresh_token().to_string(),
                        valid_until: stream.get_refresh_token_validity()
                    }
                )
            ),
            Err(err) => Err(StreamError::GenericStreamError { details: err.to_string() })
        }
    } 
}

pub async fn get_stream(collection: &Collection<StreamData>, stream_id: &str) -> StreamResult<StreamData> { 
    let now_mongo = bson::DateTime::now();

    let filter = doc! {
        "stream_id": stream_id,
        "$or": [
            { "valid_until": {"$gte": now_mongo} },
            { "refresh_token_valid_until": {"$gte": now_mongo} }
        ]
    };

    match collection.find_one(filter).await {
        Ok(stream_option) => {
            if let Some(stream) = stream_option {
                Ok(stream)
            } else {
                Err(StreamError::GenericStreamError { details: "blah blah blah".to_string() })
            }
        }
        Err(err) => Err(StreamError::GenericStreamError { details: err.to_string() })
    }
}
