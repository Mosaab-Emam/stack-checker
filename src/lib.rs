use std::fs::{DirEntry, ReadDir};

pub fn get_file(files: ReadDir, file_name: &str) -> Option<DirEntry> {
    for file in files {
        let current = file.unwrap();
        let name = current.file_name().into_string().unwrap();
        if name == file_name {
            return Some(current);
        }
    }
    None
}
