use std::path::{Path, PathBuf};

use super::errors::StreamError;

pub struct StreamHandler<'a>{
    base_path: &'a Path
}

impl<'a> StreamHandler <'a> {
    pub fn new(base_path: &'a str) -> Result<Self, StreamError> {
        let base_path = Path::new(base_path);

        if !base_path.exists() {
            return Err(StreamError::GenericStreamError { 
                details: "There is no such dir".to_string()
            })
        }

        Ok(Self { base_path })
    }

    pub fn get_playlist_data(&self) -> Option<PathBuf> {
        let mut file_path = PathBuf::from(self.base_path);

        file_path.push("playlist.m3u8"); 

        if file_path.exists() {
            Some(file_path)
        } else {
            None
        }
    }

    // TODO: finish this method brother :)
    pub fn get_chunk(&self, chunk_number: usize) ->  Option<Box<Path>>{
        let mut chunks_path = PathBuf::from(self.base_path);
        chunks_path.push("chunks");

        todo!()
    }
}
