use clap::Args;
use std::error::Error;
use fnm::commands::command::Command;
use super::command_handler::CommandHandler;

#[derive(Args)]
pub struct NodeEnvArgs {
    #[clap(flatten)]
    pub config: fnm::config::FnmConfig,
}

impl CommandHandler for NodeEnvArgs {
    fn get_runnable_cmd(&self, _package_manager: &str) -> Result<String, Box<dyn Error>> {
        let env_cmd = fnm::commands::env::Env{
            shell: None,
            json: false,
            multi: true,
            use_on_cd: true,
        };

        match env_cmd.apply(&self.config) {
            Ok(()) => Ok(String::new()),
            Err(err) => {
                let msg = format!("failed to execute env command: {}", err);
                Err(msg.into())
            },
        }
    }
}