use std::path::Path;

struct FFMpegController<'a> {
    book_directory: &'a Path    
}

impl<'a> FFMpegController<'a> {
    fn new<P: AsRef<Path>>(book_path: P) {
         
    }
}
