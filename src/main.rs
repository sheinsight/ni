use clap::{Arg, ArgAction, Command};
use serde_json::Value;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use subprocess::Exec;
//  ni
//  nr
//  nlx
//  nu
//  nun
//  nci

fn main() {
    let pkg_manager = read_package_manager().unwrap();

    if let [p, v] = &pkg_manager[..2] {
        println!("👏 Package manage tool is {} , the version is {}", p, v);
        let matches = Command::new("n")
            .subcommand(
                Command::new("i").about("Install operations").arg(
                    Arg::new("package")
                        .required(false)
                        .action(ArgAction::Append),
                ),
            )
            .subcommand(Command::new("ci"))
            .subcommand(
                Command::new("r")
                    .about("Running script")
                    .arg(Arg::new("script").required(true)),
            )
            .subcommand(
                Command::new("x")
                    .about("Operation X")
                    .arg(Arg::new("package").required(true).action(ArgAction::Append)),
            )
            .get_matches();

        // Then you can check which subcommand was used
        match matches.subcommand() {
            Some(("i", i_matches)) => {
                if let Some(package) = i_matches.get_one::<String>("package") {
                    run_install_command(p, vec!["install", package])
                } else {
                    run_install_command(p, vec!["install"])
                }
            }
            Some(("ci", _ci_matches)) => match p.as_str() {
                "npm" => {
                    run_install_command(p, vec!["ci"]);
                }
                "pnpm" => {
                    run_install_command(p, vec!["install", "--frozen-lockfile"]);
                }
                "yarn" => {
                    run_install_command(p, vec!["install", "--frozen-lockfile"]);
                }
                _ => {}
            },
            Some(("r", r_matches)) => match r_matches.get_one::<String>("script") {
                Some(script) => {
                    run_install_command(p, vec![script]);
                }
                None => {
                    panic!("😢")
                }
            },
            Some(("x", x_matches)) => {
                if let Some(_package) = x_matches.get_many::<String>("package") {
                    // println!("{}", package.map(|s| s.as_str()));
                    match p.as_str() {
                        "npm" => {
                            // run_install_command(&"npx".to_string(), vec![package]);
                        }
                        "pnpm" => {
                            // run_install_command(&"pnpx".to_string(), vec![package]);
                        }
                        "yarn" => {
                            // run_install_command(&"yarn".to_string(), vec![package]);
                        }
                        _ => {}
                    }
                }
            }
            None => (),
            _ => unreachable!(),
        }
    }
}

pub fn read_package_manager() -> Result<Vec<String>, &'static str> {
    let path = Path::new("./package.json");
    if path.exists() {
        if let Ok(contents) = fs::read_to_string(path) {
            let parsed: Value = serde_json::from_str(&contents).unwrap();
            match parsed["packageManager"].as_str() {
                Some(manager) => {
                    let vec: Vec<String> = manager.split('@').map(|s| s.to_string()).collect();

                    return Ok(vec);
                }
                None => {
                    return Err(
                        "😢 Sorry, you must to be configure packageManager in package.json file ",
                    )
                }
            }
        } else {
            return Err("😢 Failure to read package.json ");
        }
    }
    Err("🔎 Could not found package.json")
}

pub fn run_install_command(package_manager: &String, cmd_arg: Vec<&str>) {
    let mut cmd = Exec::cmd(package_manager).stdout(subprocess::Redirection::Pipe);
    for arg in cmd_arg {
        cmd = cmd.arg(arg);
    }
    let mut popen = cmd.popen().unwrap();
    let stdout = popen.stdout.take().unwrap();
    let reader = BufReader::new(stdout);
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
