use actix_multipart::Multipart;
use futures_util::StreamExt as _;
use actix_web::web::{self, Json};
use mongodb::bson::{doc, oid::ObjectId};
use crate::audio::ffmpeg_controller::FFMpegController;
use crate::{database::database_models::book::Book,  AppState};

use super::book_keeper::BookKeeper;
use super::directory_handler;
use super::external_models::{UploadBookContentResult, UploadBookMetaData, UploadBookMetaDataResponse};
use super::errors::BookError;


type BookResult<T> = Result<T, BookError>;


pub fn books_scope(cfg: &mut web::ServiceConfig) {
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
    mut payload: Multipart,
    app_data: web::Data<AppState>
) -> BookResult<Json<UploadBookContentResult>> {
    println!("Started to upload");
    let base_path = format!("./books/");
    let dir_number = if let Some(number) = directory_handler::find_free_directory(&base_path) {
        number
    } else {
        return Err(BookError::InvalidDirectoryNumber { details: "Could not create directory".to_string() })
    };

    let base_path = format!("{base_path}/{dir_number}/{book_id}");
    
    println!("{}", base_path);
    let book_keeper = BookKeeper::new(&base_path)?;

    let book_id = ObjectId::parse_str(book_id.into_inner())
                    .map_err(|err| {
                        BookError::InvalidIdFormat { details: err.to_string() }
                    })?;

    let book_collection = app_data.db.get_collection::<Book>();

    println!("{:?}", book_id);
    match book_collection.find_one(doc! {"_id": book_id}).await {
        Ok(None) => return Err(BookError::InvalidIdFormat { details: "This book does not exist".to_string() }),
        Ok(Some(_)) => ..,
        Err(err) => return Err(BookError::InvalidIdFormat { details: err.to_string() })
    };

    while let Some(item) = payload.next().await {
        let field = match item {
            Ok(value) => value,
            Err(err) => return Err(err.into()),
        }; 

        if let Err(err) = book_keeper.insert_file_from_multipart_field(field).await {
            return Err(err)
        };
        
        println!("Eto kakwo podawame: {}", &base_path);
    }

    let ffmpeg = FFMpegController::new(&base_path);

    if let Err(err) = ffmpeg.merge_many_audio_files() {
        return Err(BookError::FileError { details: err.to_string() })
    };

    let book_filter = doc! {"_id": book_id};
    let update_book = doc! {"$set": doc! { "directory_number": dir_number as i64 }};

    if let Err(err) = book_collection.update_one(book_filter, update_book).await {
        return Err(BookError::InvalidDirectoryNumber { details: err.to_string() })
    } 
    Ok(Json(UploadBookContentResult { message: "We guud".to_string() } ))
}
