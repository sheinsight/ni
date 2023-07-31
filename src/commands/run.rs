use crate::utils::run_shell;
use crate::commands::runnable_cmd::RunnableCmd;
use clap::Args;

#[derive(Args)]
pub struct RunArgs {
    #[arg(value_name = "script", help = "package's 'scripts' object.")]
    pub script: String,

    #[arg(value_name = "--")]
    pub pass_on: Vec<String>,
}

impl RunnableCmd for RunArgs {
    fn run_with(&self, package_manager: &String) {
        let RunArgs { script, pass_on } = self;
        match package_manager.as_str() {
            "npm" => run_shell(format!("npm run {} -- {}", script, pass_on.join(" "))),
            _ => run_shell(format!(
                "{} run {} {}",
                package_manager,
                script,
                pass_on.join(" ")
            )),
        }
    }
}
