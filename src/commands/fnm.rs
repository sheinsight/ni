use clap::{Args, Subcommand};
use std::error::Error;
use fnm::commands::command::Command;
use std::string::String;
use fnm::user_version_reader::UserVersionReader;

#[derive(Debug, Args)]
pub struct NodeArgs {
    #[clap(flatten)]
    pub config: fnm::config::FnmConfig,

    #[command(subcommand)]
    pub cmd: Option<NodeSubCommands>,
}

impl NodeArgs {
    pub fn get_runnable_cmd(&self, _package_manager: &str) -> Result<String, Box<dyn Error>> {
        if let Some(inner) = &self.cmd {
            return match inner {
                NodeSubCommands::Install(args) => args.get_runnable_cmd(_package_manager, &self.config),
                NodeSubCommands::Use(args) => args.get_runnable_cmd(_package_manager, &self.config),
                NodeSubCommands::Env(args) => args.get_runnable_cmd(_package_manager, &self.config),
            }
        }

        return Ok(String::new());
    }
}


#[derive(Debug, Subcommand)]
pub enum NodeSubCommands {
    Install(NodeInstallArgs),
    Use(NodeUseArgs),
    Env(NodeEnvArgs),
}

#[derive(Debug, Args)]
pub struct NodeInstallArgs {
    node_version: Option<fnm::user_version::UserVersion>,

    #[clap(long, conflicts_with_all = &["node_version", "latest"])]
    pub lts: bool,

    /// Install latest version
    #[clap(long, conflicts_with_all = &["node_version", "lts"])]
    pub latest: bool,
}

impl NodeInstallArgs {
    fn get_runnable_cmd(&self, _package_manager: &str, config: &fnm::config::FnmConfig) -> Result<String, Box<dyn Error>> {
        let cloned = self.node_version.clone();
        let install_cmd = fnm::commands::install::Install{
            version: cloned,
            lts: self.lts,
            latest: self.latest,
        };

        match install_cmd.apply(config) {
            Ok(()) => Ok(String::new()),
            Err(err) => {
                let msg = format!("failed to execute use command: {}", err);
                Err(msg.into())
            },
        }
    }
}

#[derive(Debug, Args)]
pub struct NodeUseArgs {
    node_version: Option<fnm::user_version::UserVersion>,

    #[clap(long)]
    pub install_if_missing: bool,

    /// Don't output a message identifying the version being used
    /// if it will not change due to execution of this command
    #[clap(long)]
    pub silent_if_unchanged: bool,
}

impl NodeUseArgs {
    fn get_runnable_cmd(&self, _package_manager: &str, config: &fnm::config::FnmConfig) -> Result<String, Box<dyn Error>> {
        let reader: Option<UserVersionReader> = match self.node_version.clone() {
            Some(v) => Some(UserVersionReader::Direct(v)),
            None => None,
        };

        let use_cmd = fnm::commands::r#use::Use{
            version: reader,
            install_if_missing: self.install_if_missing,
            silent_if_unchanged: self.silent_if_unchanged,
        };

        match use_cmd.apply(config) {
            Ok(()) => Ok(String::new()),
            Err(err) => {
                let msg = format!("failed to execute use command: {}", err);
                Err(msg.into())
            },
        }
    }
}

#[derive(Debug, Args)]
pub struct NodeEnvArgs {
    /// The shell syntax to use. Infers when missing.
    #[clap(long)]
    pub shell: Option<fnm::shell::Shells>,

    /// Print JSON instead of shell commands.
    #[clap(long, conflicts_with = "shell")]
    pub json: bool,

    /// Deprecated. This is the default now.
    #[clap(long, hide = true)]
    pub multi: bool,

    /// Print the script to change Node versions every directory change
    #[clap(long)]
    pub use_on_cd: bool,
}

impl NodeEnvArgs {
    fn get_runnable_cmd(&self, _package_manager: &str, config: &fnm::config::FnmConfig) -> Result<String, Box<dyn Error>> {
        let cloned = self.shell.clone();
        let env_cmd = fnm::commands::env::Env{
            shell: cloned,
            json: self.json,
            multi: self.multi,
            use_on_cd: self.use_on_cd,
        };

        match env_cmd.apply(config) {
            Ok(()) => Ok(String::new()),
            Err(err) => {
                let msg = format!("failed to execute env command: {}", err);
                Err(msg.into())
            },
        }
    }
}
