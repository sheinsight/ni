use serde_json::Value;
use std::fs;
use std::path::Path;
use subprocess::Exec;

pub fn read_package_manager() -> Option<Vec<String>> {
    let path = Path::new("./package.json");

    if path.exists() {
        let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
        let parsed: Value = serde_json::from_str(&contents).unwrap();
        match parsed["packageManager"].as_str() {
            Some(manager) => {
                return Some(manager.split('@').map(|s| s.to_string()).collect());
            }
            None => println!("No packageManager field found"),
        };
    } else {
        println!("😭 Package.json file does not exist");
    }

    None
}

pub fn run_install_command(
    package_manager: &String,
    cmd_arg: Option<&str>,
) -> Result<subprocess::Popen, String> {
    let mut cmd = Exec::cmd(package_manager).stdout(subprocess::Redirection::Pipe);
    if let Some(arg) = cmd_arg {
        cmd = cmd.arg(arg);
    }
    cmd.popen()
        .map_err(|e| format!("Failed to start subprocess: {}", e))
}
