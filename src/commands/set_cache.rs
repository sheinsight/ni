use crate::utils::run_shell;
use crate::commands::runnable_cmd::RunnableCmd;
use clap::Args;

#[derive(Args)]
pub struct SetCacheArgs {
    #[arg(name = "path")]
    pub path: String,
}

impl RunnableCmd for SetCacheArgs {
    fn run_with(&self, package_manager: &String) {
        let SetCacheArgs { path } = self;
        match package_manager.as_str() {
            "npm" => run_shell(format!("npm config set cache {}", path)),
            "yarn" => run_shell(format!("yarn config set cache-folder {}", path)),
            "pnpm" => run_shell(format!("pnpm config set store-dir {}", path)),
            _ => panic!("packageManager configure error"),
        }
    }
}
