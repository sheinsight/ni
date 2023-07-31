use clap::Args;

use super::command_handler::CommandHandler;

#[derive(Args)]
pub struct RunArgs {
    #[arg(value_name = "script", help = "package's 'scripts' object.")]
    pub script: String,

    #[arg(value_name = "--")]
    pub pass_on: Vec<String>,
}

impl CommandHandler for RunArgs {
    fn get_runnable_cmd(&self, package_manager: &String) -> String {
        let RunArgs { script, pass_on } = self;
        match package_manager.as_str() {
            "npm" => format!("npm run {} -- {}", script, pass_on.join(" ")),
            _ => format!("{} run {} {}", package_manager, script, pass_on.join(" ")),
        }
    }
}
