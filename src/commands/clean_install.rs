use crate::utils::run_shell;
use crate::commands::runnable_cmd::RunnableCmd;
use clap::Args;

#[derive(Args)]
pub struct CleanInstallArgs {}

impl RunnableCmd for CleanInstallArgs {
    fn run_with(&self, package_manager: &String) {
        match package_manager.as_str() {
            "npm" => run_shell(format!("npm ci")),
            _ => run_shell(format!("{} install --frozen-lockfile", package_manager)),
        }
    }
}
