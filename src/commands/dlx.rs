use crate::utils::run_shell;
use clap::Args;

#[derive(Args)]
pub struct DlxArgs {
    /// Target package
    #[arg(value_name = "package")]
    pub package: String,
}

pub fn handler(package_manager: &String, dlx_args: DlxArgs) {
    let DlxArgs { package } = dlx_args;
    match package_manager.as_str() {
        "npm" => run_shell(format!("npx {}", package)),
        _ => run_shell(format!("{} dlx {}", package_manager, package)),
    }
}
