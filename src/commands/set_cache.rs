use clap::Args;

use super::command_handler::CommandHandler;

#[derive(Args)]
pub struct SetCacheArgs {
    #[arg(name = "path")]
    pub path: String,
}

impl CommandHandler for SetCacheArgs {
    fn get_runnable_cmd(&self, package_manager: &String) -> String {
        let SetCacheArgs { path } = self;
        match package_manager.as_str() {
            "npm" => format!("npm config set cache {}", path),
            "yarn" => format!("yarn config set cache-folder {}", path),
            "pnpm" => format!("pnpm config set store-dir {}", path),
            _ => format!("npm config set cache {}", path),
        }
    }
}
