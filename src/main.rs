use clap::{arg, command, Parser, Subcommand};

use regex::Regex;
use serde_json::Value;
use std::fs;
use std::path::Path;
use subprocess::Exec;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add new packages to dependencies
    #[command(arg_required_else_help = true)]
    Add {
        /// Save installed packages to a package.json file as dependencies.
        #[arg(value_name = "package")]
        package: String,

        /// Save installed packages to a package.json file as dependencies.
        #[arg(group = "position", short = 'S', long = "save")]
        save: bool,

        /// Package will appear in your devDependencies.
        #[arg(group = "position", short = 'D', long = "save-dev")]
        save_dev: bool,

        /// Install the specified packages as optionalDependencies.
        #[arg(group = "position", short = 'O', long = "save-optional")]
        save_optional: bool,

        /// Install a package globally.
        #[arg(group = "position", short = 'g', long = "global")]
        global: bool,
    },

    /// Like npm ci
    #[command(name = "clean-install", alias = "ci")]
    CleanInstall {},

    /// Used to install all dependencies for a project.
    #[command(name = "install", alias = "i")]
    Install {},

    /// This runs an arbitrary command from a package's 'scripts' object.
    #[command(name = "run", arg_required_else_help = true)]
    Run {
        #[arg(value_name = "script", help = "package's 'scripts' object.")]
        script: String,

        #[arg(value_name = "--")]
        pass_on: Vec<String>,
    },
}

fn main() {
    let pkg_manager = read_package_manager();
    if let [p, v] = &pkg_manager[..2] {
        println!(
            "🥳 The current package manager being used is : '{}@{}' ",
            p, v
        );
        let cli = Cli::parse();
        match cli.commands {
            Commands::Add {
                package,
                save,
                save_dev,
                save_optional,
                global,
            } => {
                if save {
                    run_shell(format!("{} add --save {}", p, package));
                } else if save_dev {
                    run_shell(format!("{} add --save-dev {}", p, package));
                } else if save_optional {
                    run_shell(format!("{} add --save-optional {}", p, package))
                } else if global {
                    match p.as_str() {
                        "yarn" => run_shell(format!("yarn global add {}", package)),
                        _ => run_shell(format!("{} add --global {}", p, package)),
                    }
                } else {
                    run_shell(format!("{} add {}", p, package));
                }
            }
            Commands::CleanInstall {} => match p.as_str() {
                "npm" => run_shell(format!("npm ci")),
                _ => run_shell(format!("{} install --frozen-lockfile", p)),
            },
            Commands::Install {} => run_shell(format!("{} install", p)),
            Commands::Run { script, pass_on } => {
                match p.as_str() {
                    "npm" => run_shell(format!("npm run dev -- {}", pass_on.join(" "))),
                    _ => run_shell(format!("{} run dev {}", p, pass_on.join(" "))),
                }
                run_shell(format!("{} run {}", p, script));
            }
            _ => {
                panic!("error")
            }
        }
    }
}

pub fn read_package_manager() -> Vec<String> {
    let path = Path::new("./package.json");
    if path.exists() {
        let contents = fs::read_to_string(path).unwrap();
        let parsed: Value = serde_json::from_str(&contents).unwrap();
        match parsed["packageManager"].as_str() {
            Some(manager) => {
                let re = Regex::new(r"(npm|pnpm|yarn)@(.*)").unwrap();
                if let Some(caps) = re.captures(manager) {
                    return vec![caps[1].to_string(), caps[2].to_string()];
                } else {
                    panic!("😢 PackageManager parsing failed, possibly due to incorrect format. ");
                }
            }
            None => {
                panic!("😢 Sorry, you must to be configure packageManager in package.json file ");
            }
        }
    }
    panic!("🔎 Could not found package.json");
}

fn run_shell(cmd: String) {
    println!("🎯 The instruction to be executed is : '{}' ", cmd);
    Exec::cmd("sh").arg("-c").arg(cmd).popen().unwrap();
}
