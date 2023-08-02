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
    fn get_runnable_cmd(
        &self,
        package_manager: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let UnInstallArgs { package, global } = self;
        let cmd;
        if *global {
            cmd = match package_manager {
                "npm" => format!("npm uninstall -g {}", package),
                "yarn" => format!("yarn global remove {}", package),
                "pnpm" => format!("pnpm remove -g {}", package),
                _ => return Err("package_manager is invalid".into()),
            }
        } else {
            cmd = match package_manager {
                "npm" => format!("npm uninstall {}", package),
                "yarn" => format!("yarn remove {}", package),
                "pnpm" => format!("pnpm remove {}", package),
                _ => return Err("package_manager is invalid".into()),
            }
        }
        Ok(cmd)
    }
}
