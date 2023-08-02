use clap::Args;

use super::command_handler::CommandHandler;

#[derive(Args)]
pub struct SetCacheArgs {
    #[arg(name = "path")]
    pub path: String,
}

impl CommandHandler for SetCacheArgs {
    fn get_runnable_cmd(
        &self,
        package_manager: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let SetCacheArgs { path } = self;
        let cmd = match package_manager {
            "npm" => format!("npm config set cache {}", path),
            "yarn" => format!("yarn config set cache-folder {}", path),
            "pnpm" => format!("pnpm config set store-dir {}", path),
            _ => return Err("package_manager is invalid".into()),
        };
        Ok(cmd)
    }
}
