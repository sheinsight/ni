// use serde_json::Value;
// use std::fs;
// use std::path::Path;
// use std::path::PathBuf;

use clap::{arg, command, Command};

//  ni
//  nr
//  nlx
//  nu
//  nun
//  nci

fn main() {
    let matches = command!() // requires `cargo` feature
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Adds files to myapp")
                .arg(arg!([NAME])),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => println!(
            "'myapp add' was used, name is: {:?}",
            sub_matches.get_one::<String>("NAME")
        ),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}

// fn read_package_manager() {
//     let path = Path::new("./package.json");

//     if path.exists() {
//         let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
//         let parsed: Value = serde_json::from_str(&contents).unwrap();
//         match parsed["packageManager"].as_str() {
//             Some(manager) => {
//                 let name: Vec<&str> = manager.split('@').collect();
//                 match name[0] {
//                     "npm" => println!("The package manager is npm"),
//                     "yarn" => println!("The package manager is yarn"),
//                     "pnpm" => println!("The package manager is pnpm"),
//                     _ => println!("Unknown package manager"),
//                 }
//             }
//             None => println!("No packageManager field found"),
//         };
//     } else {
//         println!("File does not exist");
//     }
// }
