use clap::Args;

use super::command_handler::CommandHandler;

#[derive(Args)]
pub struct DlxArgs {
    /// Target package
    #[arg(value_name = "package")]
    pub package: String,

    #[arg(value_name = "--")]
    pub pass_on: Vec<String>,
}

impl CommandHandler for DlxArgs {
    fn get_runnable_cmd(&self, package_manager: &String) -> String {
        let DlxArgs { package, pass_on } = self;
        match package_manager.as_str() {
            "npm" => format!("npx {} {}", package, pass_on.join(" ")),
            _ => format!("{} dlx {} {}", package_manager, package, pass_on.join(" ")),
        }
    }
}
