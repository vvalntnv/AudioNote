use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::{Display, Error};
use super::super::generics::errors::ErrorContent;

#[derive(Display, Error, Debug)]
pub enum StreamError {
    #[display("generic_stream_error")]
    GenericStreamError { details: String }
}

impl ResponseError for StreamError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let status_code = self.status_code();
        let message = self.to_string();
        let details = match self {
            Self::GenericStreamError { details } => details.to_owned()
        };

        let error_content = ErrorContent::new(message, details, status_code.as_u16());

        HttpResponse::build(status_code).json(error_content)
    }
    
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::GenericStreamError { .. } => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

impl From<String> for StreamError {
    fn from(value: String) -> Self {
        return StreamError::GenericStreamError { details: value }
    }
}
