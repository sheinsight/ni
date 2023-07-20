use serde_json::Value;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use std::process::{Command as ProcessCommand, Stdio};

//  ni
//  nr
//  nlx
//  nu
//  nun
//  nci
static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog\n";
fn main() {
    match read_package_manager() {
        Some(names) => {
            if let Some(name) = names.first() {
                println!("{}", name);
                let process = match ProcessCommand::new(name)
                    .arg("install")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()
                {
                    Err(why) => panic!("couldn't spawn wc: {:?}", why),
                    Ok(process) => process,
                };

                match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
                    Err(why) => panic!("couldn't write to wc stdin: {:?}", why),
                    Ok(_) => println!("sent pangram to wc"),
                }

                let mut s = String::new();
                match process.stdout.unwrap().read_to_string(&mut s) {
                    Err(why) => panic!("couldn't read wc stdout: {:?}", why),
                    Ok(_) => print!("wc responded with:\n{}", s),
                }
            }
        }
        None => println!("No package manager found or file does not exist"),
    }
}

fn read_package_manager() -> Option<Vec<String>> {
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
        println!("File does not exist");
    }

    None
}
