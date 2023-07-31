use crate::utils::run_shell;
use crate::commands::runnable_cmd::RunnableCmd;
use clap::Args;

#[derive(Args)]
pub struct InstallArgs {}

impl RunnableCmd for InstallArgs {
    fn run_with(&self, package_manager: &String) {
        run_shell(format!("{} install", package_manager))
    }
}
