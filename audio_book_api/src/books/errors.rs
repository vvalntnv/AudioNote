use actix_web::{http::StatusCode, HttpResponse, ResponseError};

use super::super::generics::errors::ErrorContent;
use derive_more::{Error, Display};

#[derive(Debug, Display, Error)]
pub enum BookError {
    #[display("unsuccessful_insertion")]
    UnsuccessfulInsertion{ details: String }
}

impl ResponseError for BookError {
    fn error_response(&self) -> actix_web::HttpResponse {
        let status_code = self.status_code(); 
        let message = self.to_string();
        let details = match self {
            BookError::UnsuccessfulInsertion { details } => details.to_owned()
        };

        let error_content = ErrorContent::new(message, details, status_code.as_u16());

        HttpResponse::build(status_code).json(error_content)
    }


    fn status_code(&self) -> StatusCode {
        match self {
            BookError::UnsuccessfulInsertion{ .. } => StatusCode::INTERNAL_SERVER_ERROR
        } 
    }
}
