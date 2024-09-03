use std::path::Path;

use serde::{de::DeserializeOwned, Serialize};

use crate::database::database_models::{book::Book, stream_data::StreamData};

struct M3U8Parser<'a> {
    file_directory: &'a Path
}


impl<'a> M3U8Parser<'a>  {
    fn new<P: AsRef<Path>>(path: &'a P) -> Self {
        M3U8Parser {
            file_directory: path.as_ref()
        }
    } 

    fn generate_new_file<S: AsRef<str>>(&self, url: S) {
        todo!()
    }
}
