use serde_json::Value;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use subprocess::Exec;

pub fn read_package_manager() -> Result<Vec<String>, &'static str> {
    let path = Path::new("./package.json");
    if path.exists() {
        if let Ok(contents) = fs::read_to_string(path) {
            let parsed: Value = serde_json::from_str(&contents).unwrap();
            if let Some(manager) = parsed["packageManager"].as_str() {
                let vec: Vec<String> = manager.split('@').map(|s| s.to_string()).collect();
                return Ok(vec);
            }
        } else {
            return Err(" Failure to read package.json ");
        }
    }
    Err("Could not found package.json")
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
