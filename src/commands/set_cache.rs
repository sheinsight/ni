use crate::utils::run_shell;
use clap::Args;

#[derive(Args)]
pub struct SetCacheArgs {
    #[arg(name = "path")]
    pub path: String,
}

pub fn handler(package_manager: &String, set_cache_args: SetCacheArgs) {
    let SetCacheArgs { path } = set_cache_args;
    match package_manager.as_str() {
        "npm" => run_shell(format!("npm config set cache {}", path)),
        "yarn" => run_shell(format!("yarn config set cache-folder {}", path)),
        "pnpm" => run_shell(format!("pnpm config set store-dir {}", path)),
        _ => panic!("packageManager configure error"),
    }
}
