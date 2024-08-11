use actix_web::ResponseError;
use serde::{Serialize, Deserialize};
use derive_more::{Display, Error};

#[derive(Deserialize)]
pub struct UploadBookMetaData {
    pub book_name: String,
    pub author_name: String,
    pub book_description: String
}

#[derive(Serialize)]
pub struct UploadBookMetaDataResponse {
    pub book_id: String
}

#[derive(Debug, Display, Error)]
pub enum BookError {
    UnsuccessfulInsertion
}

impl ResponseError for BookError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let mut res = actix_web::HttpResponse::new(self.status_code());

        let mut buf = actix_web::web::BytesMut::new();
        let _ = std::write!(helpers::MutWriter(&mut buf), "{}", self);

        let mime = mime::TEXT_PLAIN_UTF_8.try_into_value().unwrap();
        res.headers_mut().insert(actix_web::http::header::CONTENT_TYPE, mime);

        res.set_body(actix_web::body::BoxBody::new(buf))
    }
}
