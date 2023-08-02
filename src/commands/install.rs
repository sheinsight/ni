use clap::Args;

use super::command_handler::CommandHandler;

#[derive(Args)]
pub struct InstallArgs {}

impl CommandHandler for InstallArgs {
    fn get_runnable_cmd(
        &self,
        package_manager: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let cmd = match package_manager {
            "npm" => format!("npm install"),
            "yarn" => format!("yarn install"),
            "pnpm" => format!("pnpm install"),
            _ => return Err("package_manager is invalid".into()),
        };
        Ok(cmd)
    }
}
