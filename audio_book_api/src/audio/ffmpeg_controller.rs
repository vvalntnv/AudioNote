use std::env;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;

use regex::Regex;

use crate::audio::audio_converter::AudioConverterTask;

pub struct FFMpegController<'a> {
    book_directory: &'a Path,
    book_id: String
}

impl<'a> FFMpegController<'a> {
    pub fn new<P: AsRef<Path>>(book_path: &'a P, book_id: String) -> Self {
        FFMpegController {
            book_directory: book_path.as_ref(),
            book_id
        } 
    }


    /// This method will spawn a separate thread that will encode a file
    /// and write to a log file. To follow the progress of the encoding,
    /// the external user should connect to one of the `/progress` websocket endpoints
    pub fn create_hls_stream(&self) -> Result<(), String> {
        let file = match self.get_book_file() {
            Ok(Some(file)) => file,
            Ok(None) => return Err("Book file couldn't be found".to_string()),
            Err(err) => return Err(err.to_string())
        }; 

        let chunks_dir = self.book_directory.join("chunks");

        if !chunks_dir.exists() {
            fs::create_dir(&chunks_dir)
                .map_err(|e| e.to_string())?;
        }
        let job = AudioConverterTask::new(&self.book_id);

        // This will run the command in a separate thread
        job.spawn(file, &chunks_dir)?;

        Ok(())
    }

    fn get_book_file(&self) -> Result<Option<PathBuf>, String> {
        let file_regex = Regex::new(r"book\.[a-z0-9]+").unwrap(); 

        let dir_entries = match fs::read_dir(self.book_directory) {
            Ok(entries) => entries,
            Err(err) => return Err(err.to_string())
        };

        for entry in dir_entries {
            let entry = if let Err(err) = entry {
                return Err(err.to_string());
            } else {
                entry.unwrap()
            };

            let path = entry.path();

            if !path.is_file() {
               continue 
            }

            let file_name = path.file_name().unwrap().to_string_lossy();
            if file_regex.is_match(&file_name) {
                return Ok(Some(path));
            }
        }

        Ok(None)
    }


    /// Megres many audio files in the books dircetory into one
    /// If the file is only one, then it renames it and places it in the root of the book directory
    /// Note that this file will be deleted sooner or later
    pub fn merge_many_audio_files(&self) -> Result<(), String> {
        let mut path_buf = self.book_directory.to_path_buf();        
        
        path_buf.push("original");
        
        if path_buf.components().count() <= 2 {
            return self.move_file_to_root(path_buf); 
        }

        let filelist_file = path_buf.join("filelist.txt");

        let filelist_extension = if let Some(ext) = self.get_single_file_extension(&filelist_file){
            ext
        } else {
            "".to_string()
        };

        let output_file_name = format!("book.{ext}", ext=filelist_extension);
        let output_file = path_buf.parent().unwrap().join(&output_file_name);

        let command = Command::new("ffmpeg")
            .args([
                "-f",
                "concat",

                "-safe",
                "0",

                "-i",
                filelist_file.to_str().unwrap(),

                "-vn",

                "-c",
                "copy",

                "-threads",
                "4",

                &output_file.to_str().unwrap()
            ])
            .stdout(Stdio::piped())
            .spawn();

        match command {
            Ok(mut handle) => {
                if let Err(err) = handle.wait(){
                    Err(err.to_string())
                } else {
                    if let Err(err) = fs::remove_dir_all(path_buf) { return Err(err.to_string()) };
                    Ok(())
                }
            }
            Err(err) => Err(format!("Maikata si eba nesho, {}", err.to_string()))
        }

    }

    fn get_single_file_extension(&self, filelist_file: &Path) -> Option<String> {
        let file_result = OpenOptions::new()
            .read(true)
            .open(filelist_file);
        
        let file = match file_result {
            Ok(content) => content,
            Err(_) => return None
        };
        let mut reader = BufReader::new(file);
        let mut line = String::new();

        let file_regex = Regex::new(r"file '.+\.([a-z0-9]+)'\n").unwrap();

        match reader.read_line(&mut line) {
            Ok(_) => {
                let file_ext = file_regex.captures(&line);
                if let Some(captures) = file_ext {
                    let extension = captures.get(1)?.as_str().to_owned();
                    Some(extension)
                } else {
                    None
                }
            },
            Err(_) => return None
        }
    }


    fn move_file_to_root<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let destination_dir = self.book_directory;
        let curr_path = path.as_ref();
        
        if !curr_path.is_file() {
            return Err("The path given is not a file!".to_string()) 
        }

        let file_name = curr_path.file_name().unwrap();
        let dest_path = destination_dir.join(file_name);

        match fs::rename(curr_path, dest_path) {
            Ok(()) => Ok(()),
            Err(err) => Err(err.to_string())
        }
    }
}
