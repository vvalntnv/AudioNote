use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use crate::database::savable::Savable;

#[derive(Serialize, Deserialize)]
pub struct Book {
    _id: ObjectId,
    name: String,
    author_name: String,
    description: String,
    directory_number: usize
}

impl Book {
    pub fn new(
        name: String, 
        author_name: String,
        book_description: String,
        directory_number: usize
    ) -> Self {
        Book {
            _id: ObjectId::new(),
            name,
            author_name,
            description: book_description,
            directory_number
        }
    }
}

impl Savable for Book {
    const COLLECTION_NAME: &'static str = "books";
    const DATABASE_NAME: &'static str = "audio_note";
}
