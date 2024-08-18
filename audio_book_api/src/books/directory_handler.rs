use std::fs;
use std::path::Path;

pub fn find_free_directory<P: AsRef<Path>>(base_dir: P) -> Option<usize> {
    let base_path: &Path = base_dir.as_ref();
    let base_path_children = if let Ok(children) = base_path.read_dir() {
        children
    } else {
        return None
    }; 

    let last_child = if let Ok(dir) = base_path_children.last()? {
        dir
    } else {
        return None
    };

    let last_ancestor_books_count = fs::read_dir(last_child.path());

    todo!(); 
}
