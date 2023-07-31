use crate::utils::run_shell;
use crate::commands::runnable_cmd::RunnableCmd;
use clap::Args;

#[derive(Args)]
pub struct DlxArgs {
    /// Target package
    #[arg(value_name = "package")]
    pub package: String,

    #[arg(value_name = "--")]
    pub pass_on: Vec<String>,
}

impl RunnableCmd for DlxArgs {
    fn run_with(&self, package_manager: &String) {
        let DlxArgs { package, pass_on } = self;
        match package_manager.as_str() {
            "npm" => run_shell(format!("npx {} {}", package, pass_on.join(" "))),
            _ => run_shell(format!(
                "{} dlx {} {}",
                package_manager,
                package,
                pass_on.join(" ")
            )),
        }    
    }
}
