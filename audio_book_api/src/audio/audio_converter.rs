use std::path::Path;

enum BookType {
    Mp3(usize),
    M3A(usize),
    M4A(usize),
}

struct AudioConverter {
    type_: BookType,
}

impl AudioConverter {
    fn determine_type(book_dir: &Path) -> Option<BookType>{
        let book_ext = book_dir.extension()
            .and_then(|extension| extension.to_str())?;

        todo!()  
    }

    fn convert_audio() {

    }
}
