#[macro_use]
pub mod macros;
use regex::Regex;
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::path::Path;
use subprocess::Exec;
extern crate colored;
use colored::*;

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
                    error!("PackageManager parsing failed, possibly due to incorrect format. ");
                }
            }
            None => {
                error!("Sorry, you must to be configure packageManager in package.json file ");
            }
        }
    }
    error!("Could not found package.json");
}

pub fn run_shell(cmd: String) -> Result<(), Box<dyn Error>> {
    info!("The instruction to be executed is : '{}' ", cmd);
    if let Ok(status) = Exec::cmd("sh").arg("-c").arg(cmd).popen()?.wait() {
        if !status.success() {
            error!("{} ", "error:".red());
        }
    }

    Ok(())
}
