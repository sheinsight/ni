use clap::Args;

use super::command_handler::CommandHandler;

#[derive(Args)]
pub struct UnInstallArgs {
    /// Target package
    #[arg(value_name = "package")]
    pub package: String,

    /// Install a package globally.
    #[arg(group = "position", short = 'g', long = "global")]
    pub global: bool,
}

impl CommandHandler for UnInstallArgs {
    fn get_runnable_cmd(&self, package_manager: &String) -> String {
        let UnInstallArgs { package, global } = self;
        if *global {
            match package_manager.as_str() {
                "npm" => format!("npm uninstall -g {}", package),
                "yarn" => format!("yarn global remove {}", package),
                "pnpm" => format!("pnpm remove -g {}", package),
                _ => format!("npm uninstall -g {}", package),
            }
        } else {
            match package_manager.as_str() {
                "npm" => format!("npm uninstall {}", package),
                _ => format!("{} remove {}", package_manager, package),
            }
        }
    }
}
