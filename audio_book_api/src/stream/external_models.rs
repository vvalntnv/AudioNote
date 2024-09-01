use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::database::database_models::stream_data::StreamData;

#[derive(Serialize)]
pub struct StreamGeneratedResponse {
    stream_id: String,
    stream_refresh_token: String
}

impl From<StreamData> for StreamGeneratedResponse {
    fn from(stream: StreamData) -> Self {
        StreamGeneratedResponse {
            stream_id: stream.get_stream_id().to_string(),
            stream_refresh_token: stream.get_refresh_token().to_string()
        }
    }
}

#[derive(Deserialize)]
pub struct RefreshStreamRequest {
    pub refresh_token: String 
}

#[derive(Serialize)]
pub struct RefreshStreamResponse {
    pub message: String,
    pub refresh_token: String,
    pub valid_until: DateTime<Local>
}

#[derive(Deserialize)]
pub struct StreamChunkRequest {
    pub dir_number: String,
    pub book_id: String,
    pub chunk_name: String
}
