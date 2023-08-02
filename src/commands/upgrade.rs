use clap::Args;
use std::error::Error;

use super::command_handler::CommandHandler;

#[derive(Args)]
pub struct UpgradeArgs {
    /// Target package
    #[arg(value_name = "package")]
    pub package: String,
}

impl CommandHandler for UpgradeArgs {
    fn get_runnable_cmd(&self, package_manager: &str) -> Result<String, Box<dyn Error>> {
        let cmd = match package_manager {
            "pnpm" => format!("pnpm upgrade {}", self.package),
            "yarn" => format!("yarn upgrade {}", self.package),
            "npm" => format!("npm upgrade {}", self.package),
            _ => return Err("package_manager is invalid".into()),
        };
        Ok(cmd)
    }
}
