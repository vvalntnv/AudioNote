use serde::{Serialize, Deserialize};

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
