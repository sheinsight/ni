use clap::Args;
use std::error::Error;
use fnm::commands::command::Command;
use super::command_handler::CommandHandler;
use fnm::user_version_reader::UserVersionReader;
use std::str::FromStr;

#[derive(Args)]
pub struct UseArgs {
    #[clap(flatten)]
    pub config: fnm::config::FnmConfig,

    pub node_version: Option<String>,
}

impl CommandHandler for UseArgs {
    fn get_runnable_cmd(&self, _package_manager: &str) -> Result<String, Box<dyn Error>> {
        let mut use_cmd = fnm::commands::r#use::Use{
            version: None,
            install_if_missing: false,
            silent_if_unchanged: false,
        };

        if let Some(version_str) = &self.node_version {
            if let Ok(version) = UserVersionReader::from_str(version_str) {
                use_cmd = fnm::commands::r#use::Use{
                    version: Some(version),
                    install_if_missing: false,
                    silent_if_unchanged: false,
                }
            }
        }

        match use_cmd.apply(&self.config) {
            Ok(()) => Ok(String::new()),
            Err(err) => {
                let msg = format!("failed to execute use command: {}", err);
                Err(msg.into())
            },
        }
    }
}
