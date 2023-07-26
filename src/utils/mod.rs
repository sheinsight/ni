use regex::Regex;
use serde_json::Value;
use std::fs;
use std::path::Path;
use subprocess::Exec;

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

pub fn run_shell(cmd: String) {
    println!("🎯 The instruction to be executed is : '{}' ", cmd);
    Exec::cmd("sh").arg("-c").arg(cmd).popen().unwrap();
}
