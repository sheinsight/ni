use crate::utils::run_shell;
use clap::Args;

#[derive(Args)]
pub struct CleanInstallArgs {}

pub fn handler(package_manager: &String, _clean_install_args: CleanInstallArgs) {
    match package_manager.as_str() {
        "npm" => run_shell(format!("npm ci")),
        _ => run_shell(format!("{} install --frozen-lockfile", package_manager)),
    }
}
