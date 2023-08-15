use clap::Args;
use std::error::Error;
use fnm::commands::command::Command;
use super::command_handler::CommandHandler;
use crate::utils;

#[derive(Args)]
pub struct UseArgs {
    // TODO: apply self.node_version param to fnm

    // #[arg(value_name = "node_version")]
    // pub node_version: Option<String>,
}

impl CommandHandler for UseArgs {
    fn get_runnable_cmd(&self, _package_manager: &str) -> Result<String, Box<dyn Error>> {
        let value = fnm::cli::parse();
        fnm::commands::r#use::Use::default().call(value.config);

        // TODO: verify node_version already change here, or return a error instead
        Ok(String::new())
    }
}
