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
    fn get_runnable_cmd(&self, package_manager: &String) -> String {
        let AddArgs {
            package,
            save,
            save_dev,
            save_optional,
            global,
        } = self;

        if *save {
            format!("{} add --save {}", package_manager, package)
        } else if *save_dev {
            format!("{} add --save-dev {}", package_manager, package)
        } else if *save_optional {
            format!("{} add --save-optional {}", package_manager, package)
        } else if *global {
            match package.as_str() {
                "yarn" => format!("yarn global add {}", package),
                _ => format!("{} add --global {}", package_manager, package),
            }
        } else {
            format!("{} add {}", package_manager, package)
        }
    }
}
