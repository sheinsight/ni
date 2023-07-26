mod commands;
mod utils;
use crate::commands::{add, clean_install, dlx, install, run, set_cache, un_install, upgrade};
use crate::utils::read_package_manager;
use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(name= "n",author, version, about, long_about = None,disable_help_subcommand=true )]
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
}

fn main() {
    let pkg_manager = read_package_manager();
    if let [package_manager, package_manager_version] = &pkg_manager[..2] {
        println!(
            "🥳 The current package manager being used is : '{}@{}' ",
            package_manager, package_manager_version
        );
        let cli = Cli::parse();
        match cli.commands {
            Commands::Add(add_args) => add::handler(package_manager, add_args),
            Commands::CleanInstall(clean_install_args) => {
                clean_install::handler(package_manager, clean_install_args);
            }
            Commands::Install(install_args) => install::handler(package_manager, install_args),
            Commands::UnInstall(un_install_args) => {
                un_install::handler(package_manager, un_install_args);
            }
            Commands::Run(run_args) => run::handler(package_manager, run_args),
            Commands::Upgrade(upgrade_args) => {
                upgrade::handler(package_manager, upgrade_args);
            }
            Commands::Dlx(dlx_args) => {
                dlx::handler(package_manager, dlx_args);
            }
            Commands::SetCache(set_cache_args) => {
                set_cache::handler(package_manager, set_cache_args)
            }
        }
    }
}
