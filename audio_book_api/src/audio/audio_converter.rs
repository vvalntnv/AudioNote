use core::panic;
use std::path::PathBuf;
use std::sync::Arc;
use std::str;
use std::{io::{prelude::*}, path::Path};
use std::process::{Command, Stdio};
use std::fs::File;

use std::{env, thread};

use crate::logger::ffmpeg_logger::FFMpegLogger;
use crate::logger::logger::Logger;


pub struct AudioConverterTask {
    book_id: String,
}

impl AudioConverterTask {
    pub fn new<S: AsRef<str>>(book_id: S) -> Self {
        Self { 
            book_id: book_id.as_ref().to_string(),
        }
    }


    /// Spawns the task. This method consumes self
    pub fn spawn(self, file: PathBuf, output_dir: &Path) -> Result<(), String> {
        self.execute_chunk_splitting(file, output_dir)
    }

    fn execute_chunk_splitting(&self, file: PathBuf, output_dir: &Path) -> Result<(), String> {
        let segment_naming = format!("{}/segment_%04d.ts", output_dir.to_str().unwrap());

        let book_dir = output_dir.parent().unwrap();
        let playlist_file = format!("{}/playlist.m3u8", book_dir.to_str().unwrap());

        let base_url = if let Ok(var) = env::var("API_URI") { 
            var + "/" 
        } else { 
            return Err("Api base url not set".to_string())
        };

        let thread_name = format!("{}-process", &self.book_id);
        let thread_builder = thread::Builder::new()
            .name(thread_name);
        
        let log_file = self.create_log_file();

        let book_length = self.get_content_length(file.as_ref())?;
        let file_arc = Arc::new(file);

        thread_builder.spawn(move || {
            let command = Command::new("ffmpeg").args([
                "-i",
                file_arc.to_str().unwrap(),

                "-progress",
                "pipe:2",
                "-v",
                "quiet",

                // Sets the codec to Advanced Audio Coding
                "-c:a", 
                "aac", 

                // Specifies the audio bitrate
                "-b:a",
                "128k",

                // Sets the duration of each HLS chunk
                "-hls_time",
                "10",

                // Specifies maximum number of segments in playlist
                "-hls_list_size",
                "0", // This means continuous stream

                // Specifies how each segment should be named
                "-hls_segment_filename",
                &segment_naming,

                // Sets the base uri of each segment
                "-hls_base_url",
                &base_url,

                // Playlist file
                &playlist_file
                    ])
            .stderr(Stdio::piped())
            .spawn();

            let logger = FFMpegLogger::new(&log_file, book_length)
                .unwrap();

            match command {
                Ok(mut child) => {
                    let mut output = child.stderr.take().unwrap();
                    let mut buffer = [0u8; 1024];

                    loop {
                        let bytes_read = if let Ok(size) = output.read(&mut buffer) { 
                            size 
                        } else { 
                            break 
                        };
                        if bytes_read == 0 { break };
                        if let Err(_) = logger.write_to_file(&buffer) {
                            panic!("Are eba si maikata neshto")
                        };
                    }
                }
                Err(_) => panic!("ffmpeg pai si eba veko")
            }
            logger.write_complete().unwrap();

        })
        .map_err(|err| err.to_string())?;

        Ok(())
    }

    fn create_log_file(&self) -> Box<Path>{
        let log_filename = format!("logs/{}.log", &self.book_id);
        let log_file = Path::new(&log_filename);

        if !log_file.exists() {
            File::create(log_file).unwrap();
        };

        Box::from(log_file)
    }

    fn get_content_length(&self, file: &Path) -> Result<f64, String> {
        let child = Command::new("ffprobe")
            .args([
                "-v",
                "quiet",
                "-show_entries",
                "format=duration",
                "-of",
                "csv=p=0",
                file.to_str().unwrap()
            ])
            .stdout(Stdio::piped())
            .spawn();

        match child {
            Ok(mut child) => {
                let mut stdout = child.stdout.take().unwrap();
                let mut buffer = [0u8; 1024];
                stdout.read(&mut buffer).map_err(|err| err.to_string())?;

                let size_in_seconds = str::from_utf8(&buffer)
                    .map_err(|err| err.to_string())?
                    .replace("\0", "");

                let size_in_seconds = size_in_seconds.trim().parse::<f64>()
                    .map_err(|err| err.to_string())?;

                Ok(size_in_seconds * 1000.0)
            },
            Err(err) => Err(err.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_creating_float() {
        let data = "24959.524000";
        let data_f = data.trim().parse::<f64>();
        
        match data_f {
            Err(_) => assert!(false),
            Ok(number) => {
                println!("Just converted {}", number);
                assert!(true)
            }
        }
    }
}
