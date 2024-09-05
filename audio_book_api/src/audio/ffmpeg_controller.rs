use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};

pub struct FFMpegController<'a> {
    book_directory: &'a Path    
}

impl<'a> FFMpegController<'a> {
    pub fn new<P: AsRef<Path>>(book_path: &'a P) -> Self{
        FFMpegController {
            book_directory: book_path.as_ref()
        } 
    }

    // TODO: finsh this audio merging
    /// Megres many audio files in the books dircetory into one
    /// If the file is only one, then it renames it and places it in the root of the book directory
    /// Note that this file will be deleted sooner or later
    pub fn merge_many_audio_files(&self) -> Result<(), String> {
        let mut path_buf = self.book_directory.to_path_buf();        
        
        path_buf.push("original");
        
        if path_buf.components().count() <= 2 {
            return self.move_file_to_root(path_buf.join("filelist.txt")); 
        }

        let filelist_file = path_buf.clone();
        path_buf.pop();

        let filelist_extension = if let Some(ext) = self.get_single_file_extension(&filelist_file){
            ext
        } else {
            "".to_string()
        };

        let output_file_name = format!("merged_output.{ext}", ext=filelist_extension);

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

                &output_file_name
            ])
            .stdout(Stdio::piped())
            .spawn();

        match command {
            Ok(mut handle) => {
                // TODO: in the future return the handle to the child process
                // and read the stdout to get the percentage of completion
                if let Err(err) = handle.wait(){
                    Err(err.to_string())
                } else {
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

        let file_path = match reader.read_line(&mut line) {
            Ok(_) => {
                let file_path = Path::new(&line);
                if file_path.is_file() {
                    file_path
                } else {
                    return None
                }
            },
            Err(_) => return None
        };

        if let Some(extension) = file_path.extension() {
            Some(extension.to_string_lossy().into_owned())
        } else {
            None
        }

    }

    pub fn encode_book(&self) -> Result<(), String> {
        let mut original = self.book_directory.to_path_buf();
        original.push("original");         

        todo!()
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
