use clap::Args;

use super::command_handler::CommandHandler;

#[derive(Args)]
pub struct AddArgs {
    /// Save installed packages to a package.json file as dependencies.
    #[arg(value_name = "package")]
    pub package: String,

    /// Save installed packages to a package.json file as dependencies.
    #[arg(group = "position", short = 'S', long = "save")]
    pub save: bool,

    /// Package will appear in your devDependencies.
    #[arg(group = "position", short = 'D', long = "save-dev")]
    pub save_dev: bool,

    /// Install the specified packages as optionalDependencies.
    #[arg(group = "position", short = 'O', long = "save-optional")]
    pub save_optional: bool,

    /// Install a package globally.
    #[arg(group = "position", short = 'g', long = "global")]
    pub global: bool,
}

impl CommandHandler for AddArgs {
    fn get_runnable_cmd(
        &self,
        package_manager: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let AddArgs {
            package,
            save,
            save_dev,
            save_optional,
            global,
        } = self;
        let cmd;
        if *save {
            cmd = match package_manager {
                "npm" => format!("npm add --save {}", package),
                "yarn" => format!("yarn add --save {}", package),
                "pnpm" => format!("pnpm add --save {}", package),
                _ => return Err("package_manager is invalid".into()),
            };
        } else if *save_dev {
            cmd = match package_manager {
                "npm" => format!("npm add --save-dev {}", package),
                "yarn" => format!("yarn add --save-dev {}", package),
                "pnpm" => format!("pnpm add --save-dev {}", package),
                _ => return Err("package_manager is invalid".into()),
            };
        } else if *save_optional {
            cmd = match package_manager {
                "npm" => format!("npm add --save-optional {}", package),
                "yarn" => format!("yarn add --save-optional {}", package),
                "pnpm" => format!("pnpm add --save-optional {}", package),
                _ => return Err("package_manager is invalid".into()),
            };
        } else if *global {
            cmd = match package_manager {
                "npm" => format!("npm add --global {}", package),
                "yarn" => format!("yarn global add {}", package),
                "pnpm" => format!("pnpm add --global {}", package),
                _ => return Err("package_manager is invalid".into()),
            };
        } else {
            cmd = match package_manager {
                "npm" => format!("npm add  {}", package),
                "yarn" => format!("yarn add   {}", package),
                "pnpm" => format!("pnpm add  {}", package),
                _ => return Err("package_manager is invalid".into()),
            };
        }
        Ok(cmd)
    }
}
