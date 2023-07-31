use clap::Args;

use super::command_handler::CommandHandler;

#[derive(Args)]
pub struct UpgradeArgs {
    /// Target package
    #[arg(value_name = "package")]
    pub package: String,
}

impl CommandHandler for UpgradeArgs {
    fn get_runnable_cmd(&self, package_manager: &String) -> String {
        format!("{} upgrade {}", package_manager, self.package)
    }
}
