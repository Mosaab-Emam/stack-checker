use clap::Parser;
use core::panic;
use stack_checker::{enums::OutputFormat, StackChecker};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = ".")]
    path: String,

    #[arg(short, long, default_value = "stdout")]
    output_format: String,
}

fn main() {
    let args = Args::parse();
    let output_format = match args.output_format.as_str() {
        "stdout" => OutputFormat::STDOUT,
        "json" => OutputFormat::JSON,
        "yml" => OutputFormat::YAML,
        "yaml" => OutputFormat::YAML,
        "html" => OutputFormat::HTML,
        "pdf" => OutputFormat::PDF,
        _ => panic!("Output format not supported yet."),
    };

    let pm_tools = args.path.into();
    let stack_checker = StackChecker::new(pm_tools);
    stack_checker.output(output_format);
}
