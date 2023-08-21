use clap::Args;
use std::error::Error;
use fnm::commands::command::Command;
use super::command_handler::CommandHandler;

#[derive(Args)]
pub struct UseArgs {
    #[clap(flatten)]
    pub config: fnm::config::FnmConfig,

    // TODO: apply self.node_version param to fnm

    // #[arg(value_name = "node_version")]
    // pub node_version: Option<String>,
}

impl CommandHandler for UseArgs {
    fn get_runnable_cmd(&self, _package_manager: &str) -> Result<String, Box<dyn Error>> {
        let use_cmd = fnm::commands::r#use::Use{
            version: None,
            install_if_missing: false,
            silent_if_unchanged: false,
        };

        match use_cmd.apply(&self.config) {
            Ok(()) => Ok(String::new()),
            Err(err) => {
                let msg = format!("failed to execute use command: {}", err);
                Err(msg.into())
            },
        }
    }
}
