use core::str;
use std::{fs::File, io::Write, path::Path};
use fs2::FileExt;
use std::io::{Error, ErrorKind};
use regex::Regex;

use super::logger::Logger;
pub struct FFMpegLogger<'a> {
    log_file: &'a Path,
    content_ms: f64,
}

impl<'a> FFMpegLogger<'a> {
    pub fn new<P>(path: &'a P, content_ms: f64) -> Result<Self, String> 
    where P: AsRef<Path> {
        if !path.as_ref().exists() {
            return Err("The log file does not exist".to_string())   
        };

        let path = if path.as_ref().is_file() { 
            path.as_ref() 
        } else {
            return Err("The path should be a file".to_string());
        };

        Ok(FFMpegLogger { log_file: path, content_ms })
    }

    pub fn write_complete(&self) -> std::io::Result<()> {
        let mut file = File::options()
            .write(true)
            .open(self.log_file)?;

        file.lock_exclusive()?;
        file.write("100%".as_bytes())?;
        file.unlock()?;

        Ok(())
    }

    fn calculate_percentage(&self, raw_data: &str) -> Option<f64> {
        let regex = Regex::new(r"out_time_us=(\d+)").unwrap();
        
        match regex.captures(raw_data) {
            Some(captures) => {
                captures.get(1).map_or(None, |value| {
                    match value.as_str().parse::<f64>() {
                        Ok(n) => {
                            // n is the current progress
                            let n = n / 1000.0;
                            let percentage = (n / self.content_ms) * 100.0;
                            Some(percentage)
                        },
                        Err(_) => None
                    }
                }) 
            },
            None => None
        }
    }

}

impl<'a> Logger for FFMpegLogger<'a> {
    fn write_to_file(&self, data: &[u8]) -> std::io::Result<usize> {
        let mut file = File::options()
            .write(true)
            .open(self.log_file)?; 

        if data.len() < 1 {
            return Err(Error::new(ErrorKind::InvalidData, "the buffer provided is empty"));
        }
        let data = str::from_utf8(data).unwrap();

        if let Some(percentage) = self.calculate_percentage(data) {
            let percentage = percentage as usize;
            let percentage_str = percentage.to_string() + "%";

            file.lock_exclusive()?;
            file.write(percentage_str.as_bytes())?;
            file.unlock()?;
        }

        Ok(0)
    }
}
