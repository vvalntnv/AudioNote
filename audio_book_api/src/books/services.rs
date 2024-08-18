use std::fmt::format;

use actix_multipart::Multipart;
use actix_web::{HttpMessage, HttpRequest};
use futures_util::StreamExt as _;
use actix_web::web::{self, Json};
use mongodb::bson::{doc, Uuid};
use mongodb::bson::oid::ObjectId;
use crate::{database::database_models::book::Book,  AppState};

use super::book_keeper::BookKeeper;
use super::external_models::{UploadBookContentResult, UploadBookMetaData, UploadBookMetaDataResponse};
use super::errors::BookError;


type BookResult<T> = Result<T, BookError>;


pub fn users_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/meta")
            .route(web::post().to(upload_book_metadata))
    );

    cfg.service(
        web::resource("/{book_id}/content")
            .route(web::post().to(upload_book_content))
    );
}

pub async fn upload_book_metadata(
    book_data: Json<UploadBookMetaData>, 
    app_data: web::Data<AppState>
) -> BookResult<Json<UploadBookMetaDataResponse>>
{
    let collection = app_data.db.get_collection::<Book>();
    let book_data = book_data.into_inner();
    
    let new_book = Book::new(
        book_data.book_name, 
        book_data.author_name, 
        book_data.book_description,
        0 as usize
    ); 

    match collection.insert_one(new_book).await {
        Ok(result) => {  
            let response = UploadBookMetaDataResponse {
                book_id: result.inserted_id.as_object_id()
                            .unwrap().to_hex()
            };  

            Ok(Json(response))
        }
        Err(err) => Err(BookError::UnsuccessfulInsertion{
            details: err.to_string()
        })
    }
}

pub async fn upload_book_content(
    book_id: web::Path<String>,
    mut payload: Multipart
) -> BookResult<Json<UploadBookContentResult>> {
    let base_path = format!("./books/0/{book_id}");
    let book_keeper = BookKeeper::new(&base_path)?;

    while let Some(item) = payload.next().await {
        let field = match item {
            Ok(value) => value,
            Err(err) => return Err(err.into()),
        }; 

        if let Err(err) = book_keeper.insert_file_from_multipart_field(field).await {
            return Err(err)
        } else {
            let book_id = ObjectId::parse_str(book_id.into_inner())
                            .map_err(|err| BookError::InvalidIdFormat { details: err.to_string() })?;
            let book_filter = doc! {"_id": book_id};
        };
    }

    Ok(Json(UploadBookContentResult { status_code: 200 } ))
}
