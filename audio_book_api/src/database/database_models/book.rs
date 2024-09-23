use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use crate::database::savable::Savable;

#[derive(Serialize, Deserialize)]
pub struct Book {
    _id: ObjectId,
    name: String,
    author_name: String,
    description: String,
    directory_number: usize,
    is_uploaded: bool
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
            directory_number,
            is_uploaded: false
        }
    }

    pub fn get_dir_number(&self) -> usize {
        self.directory_number
    }

    pub fn set_as_uploaded(&mut self) {
        self.is_uploaded = true
    }
}

impl Savable for Book {
    const COLLECTION_NAME: &'static str = "books";
    const DATABASE_NAME: &'static str = "audio_note";
}
