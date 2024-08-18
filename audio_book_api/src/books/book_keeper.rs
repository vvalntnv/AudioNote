use std::{fs::{self, File}, io::Write, path::{Path, PathBuf}};

use actix_multipart::Field;
use futures_util::StreamExt;

use super::errors::BookError;

pub struct BookKeeper<'a> {
    base_path: &'a Path
}

impl<'a> BookKeeper<'a> {
    pub fn new(base_path: &'a str) -> Result<Self, BookError> {
        let base_path = Path::new(base_path);

        if !base_path.exists() {
            fs::create_dir_all(base_path)
                .map_err(|err| BookError::FileError { details: err.to_string() })?;
        }

        Ok(BookKeeper {
            base_path
        })
    }

    pub async fn insert_file_from_multipart_field(
        &self,
        mut field: Field
    ) -> Result<(), BookError>
    {
        if let Err(message) = self.check_field_correctness(&field) {
            Err(BookError::PayloadError { details: message.to_string() })
        } else {
            let field_name = field.name().unwrap();
            if field_name == "image" {
                match self.save_image_to_dir(&mut field).await {
                    Ok(_) => Ok(()),
                    Err(err) => Err(BookError::FileError { details: err })
                }
            } else {
                match self.save_audio_files_into_dir(&mut field).await {
                    Ok(_) => Ok(()),
                    Err(err) => Err(BookError::FileError { details: err })
                }
            }
        }
    }

    async fn save_audio_files_into_dir(&self, field: &mut Field) -> Result<(), String> {
        let file_path = self.create_file_path(field)?;

        self.write_field_data_to_file(field, file_path).await
    }

    async fn save_image_to_dir(&self, field: &mut Field) -> Result<(), String> {
        let file_name = self.get_file_name(field)?;
        let file_extension = Path::new(file_name)
                                .extension()
                                .and_then(|value| value.to_str())
                                .ok_or_else(|| "Could not get image extension".to_string())?;


        let mut file_path = self.base_path.to_path_buf();
        file_path.push(format!("cover.{extension}", extension=file_extension));

        self.write_field_data_to_file(field, file_path).await
    }

    async fn write_field_data_to_file<P: AsRef<Path>>(&self, field: &mut Field, file_path: P) -> Result<(), String> {
        let create_file = File::create(file_path);
        
        let mut file = match create_file {
            Ok(file) => file,
            Err(err) => {
                return Err(err.to_string())
            }
        };

        while let Some(chunk) = field.next().await {
            let write_result = match chunk {
                Ok(data) => file.write_all(&data),
                Err(err) => return Err(err.to_string()),
            };
            if let Err(err) = write_result {
                return Err(err.to_string())
            }         
        }
        
        Ok(())
    }

    fn create_file_path(&self, field: &Field) -> Result<PathBuf, String> {
        let mut path_buf = self.base_path.to_path_buf();
        let current_file_name = self.get_file_name(field)?;

        path_buf.push(current_file_name);

        Ok(path_buf)
    }

    fn get_file_name<'b>(&self, field: &'b Field) -> Result<&'b str, String> {
        let cd = match field.content_disposition() {
            Some(cd) => cd,
            None => return Err("No Content-Disposition found".to_string())
        };

        match cd.get_filename() {
            Some(name) => Ok(name),
            None => Err("No filename found".to_string()),
        }
    }

    fn check_field_correctness(&self, field: &Field) -> Result<(), &str> {
        match field.name() {
            Some(value) => match value {
                "image" => Ok(()),
                "audio_book_files" => Ok(()),
                _ => {
                    Err("Invalid field name passed")
                }
            }
            None => Err("Some field does not have name!")
        }
    }
}
