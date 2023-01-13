use regex::Regex;
use std::fs::{self, DirEntry, ReadDir};

// Search for file in files
// pub fn has_file(files: ReadDir, file_name: &str) -> Option<DirEntry> {
//     for file in files {
//         let current = file.unwrap();
//         let name = current.file_name().into_string().unwrap();
//         if name == file_name {
//             return Some(current);
//         }
//     }
//     None
// }

pub fn has_file(files: &mut ReadDir, file_name: &str) -> bool {
    // let mut result: bool = false;
    // files.into_iter().for_each(|file| {
    //     let current = file.unwrap();
    //     let name = current.file_name().into_string().unwrap();
    //     if name == file_name {
    //         result = true;
    //     }
    // });
    // result

    files
        .into_iter()
        .any(|file| file.unwrap().file_name().into_string().unwrap() == file_name)
}

pub trait UsesComposerPackageManager {
    fn has_laravel_package(file: DirEntry) -> bool {
        let contents = fs::read_to_string(file.path()).unwrap();
        let re = Regex::new(r#""laravel/framework": "#).unwrap();
        match re.captures(&contents) {
            Some(_) => true,
            None => false,
        }
    }
}
// pub fn is_laravel(files: ReadDir) -> bool {
//     let has_composer = has_file(files, "composer.json");

//     match has_file(files, "composer.json") {
//         Some(composer) => match Self::has_laravel_package(composer) {
//             true => true,
//             false => false,
//         },
//         None => false,
//     }

#[derive(Debug)]
pub enum PackageManager {
    Composer,
    Npm,
    Yarn,
    Pnpm,
    Pip,
}

// pub fn detect_package_manager(files: ReadDir) -> Option<PackageManager> {
//     if has_file(files, "composer.json").is_some() {
//         return Some(PackageManager::Composer);
//     }

//     if has_file(files, "package.json").is_some() {
//         if has_file(files, "yarn.lock").is_some() {
//             return Some(PackageManager::Yarn);
//         }
//         if has_file(files, "pnpm-lock.yaml").is_some() {
//             return Some(PackageManager::Pnpm);
//         }

//         return Some(PackageManager::Npm);
//     }
// }

// pub struct CopmoserPackageManager {
//     pub contents: String,
// }

// impl CopmoserPackageManager {
//     pub fn new(file: DirEntry) -> CopmoserPackageManager {
//         let contents = fs::read_to_string(file.path()).unwrap();

//         CopmoserPackageManager { contents }
//     }

//     pub fn has_package(self, package: &str) -> bool {
//         let re = Regex::new(&format!(r#""{}": "#, package)).unwrap();
//         match re.captures(&self.contents) {
//             Some(_) => true,
//             None => false,
//         }
//     }
// }

// package manager file (composer.json, package.json, etc)
// pub struct PMFile {
//     pub package_manager: PackageManager,
// }

// impl From<&str> for PMFile {
//     fn from(path: &str) -> Self {
//         let files = fs::read_dir(path).unwrap();
//     }
// }

// impl PMFile {
//     pub fn new(files: ReadDir) -> PMFile {
//         let package_manager = {
//             if has_file(&files, "composer.json") {
//                 PackageManager::Composer
//             } else if has_file(&files, "package.json") {
//                 if has_file(&files, "yarn.lock") {
//                     PackageManager::Yarn
//                 } else if has_file(&files, "pnpm-lock.yaml") {
//                     PackageManager::Pnpm
//                 } else {
//                     PackageManager::Npm
//                 }
//             } else {
//                 PackageManager::Pip
//             }
//         };

//         PMFile { package_manager }
//     }
// }
