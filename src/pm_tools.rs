use crate::enms::{PHPFramework, PackageManager, ProgrammingLanguage};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::read_to_string;
extern crate glob;
use glob::glob;

#[derive(Serialize, Deserialize, Debug)]
struct ComposerJsonFile {
    name: String,
    license: String,
    require: Value,
}

#[derive(Debug)]
pub struct PMTools {
    package_manager: PackageManager,
    contents: String,
}

impl From<&str> for PMTools {
    fn from(path: &str) -> Self {
        let package_manager =
            Self::detect_package_manager(path).expect("Failed to detect package manager.");

        let contents = match package_manager {
            PackageManager::Composer => read_to_string(path.to_owned() + "/composer.json").unwrap(),
            _ => panic!("Package manager not supported yet."),
        };

        PMTools {
            contents,
            package_manager,
        }
    }
}

impl PMTools {
    pub fn detect_package_manager(path: &str) -> Option<PackageManager> {
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

    pub fn get_package_manager(&self) -> PackageManager {
        match self.package_manager {
            PackageManager::Composer => PackageManager::Composer,
            PackageManager::Pip => PackageManager::Pip,
            PackageManager::Npm => PackageManager::Npm,
            PackageManager::Yarn => PackageManager::Yarn,
            PackageManager::Pnpm => PackageManager::Pnpm,
        }
    }

    pub fn get_name(&self) -> String {
        match self.package_manager {
            PackageManager::Composer => {
                let composer_json_file =
                    serde_json::from_str::<ComposerJsonFile>(&self.contents).unwrap();
                composer_json_file.name
            }
            _ => panic!("Package manager not supported yet."),
        }
    }

    pub fn get_license(&self) -> String {
        match self.package_manager {
            PackageManager::Composer => {
                let composer_json_file =
                    serde_json::from_str::<ComposerJsonFile>(&self.contents).unwrap();
                composer_json_file.license
            }
            _ => panic!("Package manager not supported yet."),
        }
    }

    pub fn get_packages(&self) -> Vec<(String, String)> {
        match self.package_manager {
            PackageManager::Composer => {
                let composer_json_file =
                    serde_json::from_str::<ComposerJsonFile>(&self.contents).unwrap();
                composer_json_file
                    .require
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect()
            }
            _ => panic!("Package manager not supported yet."),
        }
    }

    pub fn get_programming_language(&self) -> ProgrammingLanguage {
        match self.package_manager {
            PackageManager::Composer => ProgrammingLanguage::PHP,
            PackageManager::Pip => ProgrammingLanguage::Python,
            PackageManager::Npm | PackageManager::Yarn | PackageManager::Pnpm => {
                ProgrammingLanguage::Javascript
            }
        }
    }

    pub fn get_framework(&self) -> Option<PHPFramework> {
        if self.is_laravel() {
            return Some(PHPFramework::Laravel);
        }

        None
    }

    pub fn has_package(&self, package: &str) -> bool {
        self.get_packages().iter().any(|(k, _)| k == package)
    }

    pub fn is_laravel(&self) -> bool {
        self.has_package("laravel/framework")
    }
}
