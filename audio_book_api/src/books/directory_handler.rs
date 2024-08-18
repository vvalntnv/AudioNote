use std::fs;
use std::path::Path;

pub fn find_free_directory<P: AsRef<Path>>(base_dir: P) -> Option<usize> {
    let base_path: &Path = base_dir.as_ref();
    let base_path_children = if let Ok(children) = base_path.read_dir() {
        children
    } else {
        return None
    }; 

    let last_child = if let Some(child) = base_path_children.last() { 
        child 
    } else { 
        return Some(0) 
    };

    let last_child = if let Ok(dir) = last_child {
        dir
    } else {
        return None
    };

    let last_child_number: usize = last_child.path()
        .file_name()?
        .to_str()?
        .parse::<usize>()
        .ok()?;

    let last_ancestor_children = fs::read_dir(last_child.path()).unwrap();
    let children_count = last_ancestor_children.count();

    if children_count >= 10 {
        Some(last_child_number + 1)
    } else {
        Some(last_child_number)
    }
}
