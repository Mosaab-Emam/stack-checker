pub mod enums;
pub mod pm_tools;

use enums::{OutputFormat, PHPFramework, PackageManager, ProgrammingLanguage};
use pm_tools::PMTools;
use serde_derive::Serialize;

#[derive(Serialize, Debug)]
pub struct StackChecker {
    name: String,
    license: String,
    package_manager: PackageManager,
    programming_language: ProgrammingLanguage,
    framework: PHPFramework,
}

impl StackChecker {
    pub fn new(pm_tools: PMTools) -> Self {
        StackChecker {
            name: pm_tools.get_name(),
            license: pm_tools.get_license(),
            package_manager: pm_tools.get_package_manager(),
            programming_language: pm_tools.get_programming_language(),
            framework: pm_tools.get_framework().unwrap(),
        }
    }

    pub fn output(&self, output_format: OutputFormat) {
        match output_format {
            OutputFormat::STDOUT => println!("{:?}", self),
            OutputFormat::JSON => println!("{}", serde_json::to_string(self).unwrap()),
            OutputFormat::YAML => println!("{}", serde_yaml::to_string(self).unwrap()),
            _ => panic!("Output format not supported yet."),
        }
    }
}
