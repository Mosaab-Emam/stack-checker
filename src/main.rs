use stackcheck::{has_file, PackageManager, UsesComposerPackageManager};
extern crate glob;
use glob::glob;
use std::fmt::Display;
use std::fs::{self, ReadDir};
use std::path::Path;

fn detect_package_manager(path: &str) -> Option<PackageManager> {
    let mut node_pm: Option<PackageManager> = None;

    for entry in glob(&(path.to_owned() + "/*")).expect("Failed to read given path.") {
        match entry
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
            .split("/")
            .last()
            .unwrap()
        {
            "composer.json" => return Some(PackageManager::Composer),
            "requirements.txt" => return Some(PackageManager::Pip),
            "package.json" => {
                if node_pm.is_none() {
                    node_pm = Some(PackageManager::Npm);
                }
            }
            "yarn.lock" => node_pm = Some(PackageManager::Yarn),
            "pnpm-lock.yaml" => node_pm = Some(PackageManager::Pnpm),
            _ => (),
        }
    }

    node_pm
}

#[derive(Debug)]
struct StackChecker {
    package_manager: PackageManager,
}

impl StackChecker {
    fn new(path: &str) -> StackChecker {
        let package_manager =
            detect_package_manager(path).expect("Failed to detect package manager.");

        StackChecker { package_manager }
    }
}

fn main() {
    let path = "/workspaces/laravel";
    let stack_checker = StackChecker::new(path);
}

enum Stack {
    Laravel,
}

// struct StackChecker {}

// impl UsesComposerPackageManager for StackChecker {}

// impl StackChecker {
//     fn is_laravel(files: ReadDir) -> bool {
//         let has_composer = has_file(files, "composer.json");

//         match has_composer {
//             Some(composer) => match Self::has_laravel_package(composer) {
//                 true => true,
//                 false => false,
//             },
//             None => false,
//         }
//     }

//     fn check(files: ReadDir, stack: Stack) -> bool {
//         match stack {
//             Stack::Laravel => Self::is_laravel(files),
//         }
//     }
// }
