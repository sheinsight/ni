use clap::Args;

use super::command_handler::CommandHandler;

#[derive(Args)]
pub struct CleanInstallArgs {}

impl CommandHandler for CleanInstallArgs {
    fn get_runnable_cmd(
        &self,
        package_manager: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let cmd = match package_manager {
            "npm" => format!("npm ci"),
            "yarn" => format!("yarn install --frozen-lockfile"),
            "pnpm" => format!("pnpm install --frozen-lockfile"),
            _ => return Err("package_manager is invalid".into()),
        };
        Ok(cmd)
    }
}
