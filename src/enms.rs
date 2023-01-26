use serde_derive::Serialize;

#[derive(Debug)]
pub enum OutputFormat {
    STDOUT,
    JSON,
    YAML,
    HTML,
    PDF,
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
