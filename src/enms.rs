use serde_derive::Serialize;

pub enum OutputFormat {
    STDOUT,
    JSON,
    YAML,
}

#[derive(Serialize, Debug)]
pub enum ProgrammingLanguage {
    PHP,
    Python,
    Javascript,
}

#[derive(Serialize, Debug)]
pub enum PackageManager {
    Composer,
    Pip,
    Npm,
    Yarn,
    Pnpm,
}

#[derive(Serialize, Debug)]
pub enum PHPFramework {
    Laravel,
}
