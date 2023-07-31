use crate::utils::run_shell;
use clap::Args;

#[derive(Args)]
pub struct DlxArgs {
    /// Target package
    #[arg(value_name = "package")]
    pub package: String,

    #[arg(value_name = "--")]
    pub pass_on: Vec<String>,
}

pub fn handler(package_manager: &String, dlx_args: DlxArgs) {
    let DlxArgs { package, pass_on } = dlx_args;
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
