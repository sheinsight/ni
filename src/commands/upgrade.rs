use crate::utils::run_shell;
use clap::Args;

#[derive(Args)]
pub struct UpgradeArgs {
    /// Target package
    #[arg(value_name = "package")]
    pub package: String,
}

pub fn handler(package_manager: &String, upgrade_args: UpgradeArgs) {
    run_shell(format!(
        "{} upgrade {}",
        package_manager, upgrade_args.package
    ))
}
