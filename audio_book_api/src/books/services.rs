use actix_web::web::{self, Json};
use crate::{database::database_models::book::Book,  AppState};

use super::external_models::{UploadBookContentResult, UploadBookMetaData, UploadBookMetaDataResponse};
use super::errors::BookError;


type BookResult<T> = Result<T, BookError>;


pub fn users_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/meta")
            .route(web::post().to(upload_book_metadata))
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
    app_data: web::Data<AppState>
) -> BookResult<UploadBookContentResult> {
    todo!()
}
