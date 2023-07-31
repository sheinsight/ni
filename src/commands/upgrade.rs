use crate::utils::run_shell;
use crate::commands::runnable_cmd::RunnableCmd;
use clap::Args;

#[derive(Args)]
pub struct UpgradeArgs {
    /// Target package
    #[arg(value_name = "package")]
    pub package: String,
}

impl RunnableCmd for UpgradeArgs {
    fn run_with(&self, package_manager: &String) {
        run_shell(format!(
            "{} upgrade {}",
            package_manager, self.package
        ))
    }
}
