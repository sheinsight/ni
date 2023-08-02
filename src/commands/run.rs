use clap::Args;

use super::command_handler::CommandHandler;

#[derive(Args)]
pub struct RunArgs {
    #[arg(value_name = "script", help = "package's 'scripts' object.")]
    pub script: String,

    #[arg(value_name = "--")]
    pub pass_on: Vec<String>,
}

impl CommandHandler for RunArgs {
    fn get_runnable_cmd(
        &self,
        package_manager: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let RunArgs { script, pass_on } = self;
        let pass_on_str = pass_on.join(" ");
        let cmd = match package_manager {
            "npm" => format!("npm run {} -- {}", script, pass_on_str),
            "yarn" => format!("yarn run {} {}", script, pass_on_str),
            "pnpm" => format!("pnpm run {} {}", script, pass_on_str),
            _ => return Err("package_manager is invalid".into()),
        };
        Ok(cmd)
    }
}
