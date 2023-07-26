use crate::utils::run_shell;
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

pub fn handler(package_manager: &String, un_install_args: UnInstallArgs) {
    let UnInstallArgs { package, global } = un_install_args;
    if global {
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
