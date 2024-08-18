use serde::Serialize;

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
