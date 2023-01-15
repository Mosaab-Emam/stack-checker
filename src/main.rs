mod enms;
mod pm_tools;
mod stack_checker;

use pm_tools::PMTools;
use stack_checker::StackChecker;

fn main() {
    let path = "/workspaces/laravel";
    let pm_tools: PMTools = path.into();
    let stack_checker = StackChecker::new(pm_tools);

    stack_checker.output(enms::OutputFormat::YAML);
}
