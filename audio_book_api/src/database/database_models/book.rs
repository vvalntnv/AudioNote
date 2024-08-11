use serde::{Serialize, Deserialize};

use crate::database::savable::Savable;

#[derive(Serialize, Deserialize)]
pub struct Book {
    _id: Option<String>,
    book_name: String,
    author_name: String,
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
            _id: None,
            book_name: name,
            author_name,
            directory_number
        }
    }
}

impl Savable for Book {
    const COLLECTION_NAME: &'static str = "books";
    const DATABASE_NAME: &'static str = "books";

}
