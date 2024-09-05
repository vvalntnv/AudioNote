use actix_web::{http::StatusCode, HttpResponse, ResponseError};

use super::super::generics::errors::ErrorContent;
use derive_more::{Error, Display};
use actix_multipart::MultipartError;

#[derive(Debug, Display, Error)]
pub enum BookError {
    #[display("unsuccessful_insertion")]
    UnsuccessfulInsertion{ details: String },

    #[display("payload_error")]
    PayloadError { details: String },

    #[display("file_error")]
    FileError { details: String },

    #[display("invalid_id_format")]
    InvalidIdFormat { details: String },

    #[display("invalid_dir_numner")]
    InvalidDirectoryNumber { details: String}
}


impl ResponseError for BookError {
    fn error_response(&self) -> actix_web::HttpResponse {
        let status_code = self.status_code(); 
        let message = self.to_string();
        let details = match self {
            BookError::UnsuccessfulInsertion { details } => details.to_owned(),
            BookError::PayloadError { details } => details.to_owned(),
            BookError::FileError { details } => details.to_owned(),
            BookError::InvalidIdFormat { details } => details.to_owned(),
            BookError::InvalidDirectoryNumber { details } => details.to_owned()
        };

        let error_content = ErrorContent::new(message, details, status_code.as_u16());

        HttpResponse::build(status_code).json(error_content)
    }


    fn status_code(&self) -> StatusCode {
        match self {
            BookError::UnsuccessfulInsertion{ .. } => StatusCode::INTERNAL_SERVER_ERROR,
            BookError::PayloadError { .. } => StatusCode::BAD_REQUEST,
            BookError::FileError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            BookError::InvalidIdFormat { .. } => StatusCode::BAD_REQUEST,
            BookError::InvalidDirectoryNumber{ .. } => StatusCode::INTERNAL_SERVER_ERROR
        } 
    }
}

impl From<MultipartError> for BookError {
    fn from(value: MultipartError) -> Self {
        let details = value.to_string();
        BookError::PayloadError { details }
    }
}
