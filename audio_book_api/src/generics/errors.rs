use derive_more::derive::Debug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorContent {
    error_message: String,
    details: String,
    status_code: u16
}

impl ErrorContent {
    pub fn new(message: String, details: String, status_code: u16) -> Self {
        ErrorContent {
            error_message: message, details, status_code
        }
    }
}
