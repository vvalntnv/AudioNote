use std::{fs::File, io};

pub trait Logger {
    fn write_to_file(&self, data: &[u8]) -> io::Result<usize>;
}
