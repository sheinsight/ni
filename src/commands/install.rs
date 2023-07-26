use crate::utils::run_shell;
use clap::Args;

#[derive(Args)]
pub struct InstallArgs {}

pub fn handler(package_manager: &String, _install_args: InstallArgs) {
    run_shell(format!("{} install", package_manager))
}
