use crate::utils::run_shell;
use crate::commands::runnable_cmd::RunnableCmd;
use clap::Args;

#[derive(Args)]
pub struct UnInstallArgs {
    /// Target package
    #[arg(value_name = "package")]
    pub package: String,

    /// Install a package globally.
    #[arg(group = "position", short = 'g', long = "global")]
    pub global: bool,
}

impl RunnableCmd for UnInstallArgs {
    fn run_with(&self, package_manager: &String) {
        let UnInstallArgs { package, global } = self;
        if *global {
            match package_manager.as_str() {
                "npm" => run_shell(format!("npm uninstall -g {}", package)),
                "yarn" => run_shell(format!("yarn global remove {}", package)),
                "pnpm" => run_shell(format!("pnpm remove -g {}", package)),
                _ => {}
            }
        } else {
            match package_manager.as_str() {
                "npm" => run_shell(format!("npm uninstall {}", package)),
                _ => run_shell(format!("{} remove {}", package_manager, package)),
            }
        }
    }
}
