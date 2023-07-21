use core;
use std::io::{BufRead, BufReader};

//  ni
//  nr
//  nlx
//  nu
//  nun
//  nci

fn main() {
    match core::read_package_manager() {
        Some(names) => {
            if let [package_manager, version] = &names[..] {
                println!("{} {}", package_manager, version);
                if ["npm", "yarn", "pnpm"].contains(&package_manager.as_str()) {
                    match get_package_manager_args(&package_manager) {
                        Ok(cmd_arg) => match core::run_install_command(&package_manager, cmd_arg) {
                            Ok(mut popen) => {
                                let stdout = popen.stdout.take().unwrap();
                                let reader = BufReader::new(stdout);
                                for line in reader.lines() {
                                    println!("{}", line.unwrap());
                                }
                            }
                            Err(error) => println!("Error: {}", error),
                        },
                        Err(error) => println!("Invalid package manager: {}", error),
                    }
                }
            }
        }
        None => println!("No package manager found or file does not exist"),
    }
}

fn get_package_manager_args(package_manager: &String) -> Result<Option<&'static str>, String> {
    match package_manager.as_str() {
        "npm" | "pnpm" => Ok(Some("install")),
        "yarn" => Ok(None),
        _ => Err(format!("😭 Invalid package manager: {}", package_manager)),
    }
}
