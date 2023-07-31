use crate::utils::run_shell;
use crate::commands::runnable_cmd::RunnableCmd;
use clap::Args;

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

impl RunnableCmd for AddArgs {
    fn run_with(&self, package_manager: &String) {
        let AddArgs{
            package,
            save,
            save_dev,
            save_optional,
            global,
        } = self;

        if *save {
            run_shell(format!("{} add --save {}", package_manager, package));
        } else if *save_dev {
            run_shell(format!("{} add --save-dev {}", package_manager, package));
        } else if *save_optional {
            run_shell(format!(
                "{} add --save-optional {}",
                package_manager, package
            ))
        } else if *global {
            match package.as_str() {
                "yarn" => run_shell(format!("yarn global add {}", package)),
                _ => run_shell(format!("{} add --global {}", package_manager, package)),
            }
        } else {
            run_shell(format!("{} add {}", package_manager, package));
        }
    }
}
