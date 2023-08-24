mod commands;
mod utils;
use crate::commands::command_handler::CommandHandler;
use crate::commands::{add, clean_install, dlx, install, run, set_cache, un_install, upgrade, fnm};
use crate::utils::{read_package_manager, run_shell};
use clap::{command, Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(name= "n",author = "ityuany", version, about, long_about = None,disable_help_subcommand=true )]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add new packages to dependencies
    #[command(arg_required_else_help = true)]
    Add(add::AddArgs),

    /// Like npm ci
    #[command(name = "clean-install", visible_aliases = ["ci"])]
    CleanInstall(clean_install::CleanInstallArgs),

    /// Like npm install
    #[command(name = "install",visible_aliases = ["i"])]
    Install(install::InstallArgs),

    /// Like npm uninstall
    #[command(name = "uninstall",visible_aliases = ["un"])]
    UnInstall(un_install::UnInstallArgs),

    /// Like npm run
    #[command(name = "run", arg_required_else_help = true)]
    Run(run::RunArgs),

    /// Like npm update
    #[command(name = "upgrade", visible_aliases = ["up"], arg_required_else_help = true)]
    Upgrade(upgrade::UpgradeArgs),

    /// Like npx
    #[command(name = "dlx", visible_aliases = ["x"],arg_required_else_help = true)]
    Dlx(dlx::DlxArgs),
    
    #[command(name = "set-cache", arg_required_else_help = true)]
    SetCache(set_cache::SetCacheArgs),

    #[command(name = "node")]
    NodeSeries(fnm::NodeArgs),
}

fn main() {
    info!("Welcome to use 'n'");

    info!("Parsing the packageManager in the package.json file.");
    let pkg_manager = read_package_manager();
    if let [package_manager, package_manager_version] = &pkg_manager[..2] {
        info!(
            "The current package manager being used is: '{}@{}'",
            package_manager, package_manager_version,
        );
        let cli = Cli::parse();
        let shell = match cli.commands {
            Commands::Add(args) => args.get_runnable_cmd(package_manager),
            Commands::CleanInstall(args) => args.get_runnable_cmd(package_manager),
            Commands::Install(args) => args.get_runnable_cmd(package_manager),
            Commands::UnInstall(args) => args.get_runnable_cmd(package_manager),
            Commands::Run(args) => args.get_runnable_cmd(package_manager),
            Commands::Upgrade(args) => args.get_runnable_cmd(package_manager),
            Commands::Dlx(args) => args.get_runnable_cmd(package_manager),
            Commands::SetCache(args) => args.get_runnable_cmd(package_manager),
            Commands::NodeSeries(args) => args.get_runnable_cmd(package_manager),
        };

        match shell {
            Ok(cmd) => {
                if cmd.len() == 0 {
                    return
                }
                run_shell(cmd).unwrap()
            },
            Err(err) => {
                // TODO: try use macro here
                let message = format!("👻 {}", err);
                print!("{}\n", message.red());
            }
        }
    }
}
