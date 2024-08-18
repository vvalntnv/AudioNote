use actix_web::web;
use bson::{doc, oid::ObjectId};
use crate::{database::database_models::{book::Book, stream_data::StreamData}, AppState};

use super::{errors::StreamError, external_models::StreamGeneratedResponse};

pub fn stream_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/generate/{book_id}")
            .route(web::post().to(generate_stream_link))
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
