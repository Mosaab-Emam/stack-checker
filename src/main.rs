// generate use statements for the code below
use regex::Regex;
use std::fs::{self, DirEntry, ReadDir};
use std::path::Path;

fn main() {
    let path = "/workspaces/laravel";

    // Check if direcotry exists
    if !Path::new(path).exists() {
        println!("Path does not exist");
        return;
    }

    // Read directory
    let files = fs::read_dir(path).unwrap();

    match StackChecker::check(files, Stack::Laravel) {
        true => println!("Laravel"),
        false => println!("Not Laravel"),
    }
}

enum Stack {
    Laravel,
}

struct StackChecker {}

impl StackChecker {
    // Search for file in files
    fn has_file(files: ReadDir, file_name: &str) -> Option<DirEntry> {
        for file in files {
            let current = file.unwrap();
            let name = current.file_name().into_string().unwrap();
            if name == file_name {
                return Some(current);
            }
        }
        None
    }

    fn has_laravel_package(file: DirEntry) -> bool {
        let contents = fs::read_to_string(file.path()).unwrap();
        let re = Regex::new(r#""laravel/framework": "#).unwrap();
        match re.captures(&contents) {
            Some(_) => true,
            None => false,
        }
    }

    fn is_laravel(files: ReadDir) -> bool {
        let has_composer = Self::has_file(files, "composer.json");

        match has_composer {
            Some(composer) => match Self::has_laravel_package(composer) {
                true => true,
                false => false,
            },
            None => false,
        }
    }

    fn check(files: ReadDir, stack: Stack) -> bool {
        match stack {
            Stack::Laravel => Self::is_laravel(files),
        }
    }
}
