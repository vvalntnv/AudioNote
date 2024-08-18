use serde::Serialize;

#[derive(Serialize)]
pub struct StreamGeneratedResponse {
    stream_url: String,
    stream_refresh_token: String
}
