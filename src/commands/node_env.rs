use clap::Args;
use std::error::Error;
use fnm::commands::command::Command;
use super::command_handler::CommandHandler;

#[derive(Args)]
pub struct NodeEnvArgs {}

impl CommandHandler for NodeEnvArgs {
    fn get_runnable_cmd(&self, _package_manager: &str) -> Result<String, Box<dyn Error>> {
        println!("parsing env command");
        let value = fnm::cli::parse();
        let env_cmd = fnm::commands::env::Env{
            shell: Some(fnm::shell::Shells::Bash),
            json: false,
            multi: true,
            use_on_cd: true,
        };

        match env_cmd.apply(&value.config) {
            Ok(()) => Ok(String::new()),
            Err(err) => {
                let msg = format!("get env shell command failed: {}", err);
                Err(msg.into())
            },
        }
    }
}