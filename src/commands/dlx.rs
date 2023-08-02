use clap::Args;

use super::command_handler::CommandHandler;

#[derive(Args)]
pub struct DlxArgs {
    /// Target package
    #[arg(value_name = "package")]
    pub package: String,

    #[arg(value_name = "--")]
    pub pass_on: Vec<String>,
}

impl CommandHandler for DlxArgs {
    fn get_runnable_cmd(
        &self,
        package_manager: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let DlxArgs { package, pass_on } = self;
        let pass_on_str = pass_on.join(" ");
        let cmd = match package_manager {
            "npm" => format!("npx {} {}", package, pass_on_str),
            "yarn" => format!("yarn dlx {} {}", package, pass_on_str),
            "pnpm" => format!("pnpm dlx {} {}", package, pass_on_str),
            _ => return Err("package_manager is invalid".into()),
        };
        Ok(cmd)
    }
}
