struct DirectoryHandler {
    base_dir: String
}

impl DirectoryHandler {
    fn new(base_dir: String) -> Self {
        DirectoryHandler { base_dir }
    }
    
    fn find_free_directory() -> usize {
        todo!()
    }
}
