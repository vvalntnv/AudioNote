use actix_web::{web::{self, Json}, Responder};
use crate::{database::database_models::book::Book, AppState};

use super::external_models::{BookError, UploadBookMetaData, UploadBookMetaDataResponse};

type BookResult<R: Responder> = Result<R, BookError>;

pub fn users_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/meta")
            .route(web::post().to(upload_book_metadata))
    );
}

pub async fn upload_book_metadata(book_data: Json<UploadBookMetaData>, app_data: web::Data<AppState>)  
-> BookResult<web::Json<UploadBookMetaDataResponse>> {
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
                book_id: result.inserted_id.as_str()
                            .unwrap().to_string()
            };  

            Ok(Json(response))
        }
        Err(_) => Err(BookError::UnsuccessfulInsertion)
    }
}

pub async fn upload_book() {        
}
