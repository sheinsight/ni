use clap::Args;

use super::command_handler::CommandHandler;

#[derive(Args)]
pub struct InstallArgs {}

impl CommandHandler for InstallArgs {
    fn get_runnable_cmd(&self, package_manager: &String) -> String {
        format!("{} install", package_manager)
    }
}
