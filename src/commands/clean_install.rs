use clap::Args;

use super::command_handler::CommandHandler;

#[derive(Args)]
pub struct CleanInstallArgs {}

impl CommandHandler for CleanInstallArgs {
    fn get_runnable_cmd(&self, package_manager: &String) -> String {
        match package_manager.as_str() {
            "npm" => format!("npm ci"),
            _ => format!("{} install --frozen-lockfile", package_manager),
        }
    }
}
