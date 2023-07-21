use core;

//  ni
//  nr
//  nlx
//  nu
//  nun
//  nci

fn main() {
    let names = core::read_package_manager().unwrap();

    if let [package_manager, version] = &names[..2] {
        println!("{} {}", package_manager, version);
        if ["npm", "yarn", "pnpm"].contains(&package_manager.as_str()) {
            let cmd_arg = get_package_manager_args(package_manager);
            core::run_install_command(package_manager, cmd_arg)
        }
    }
}

fn get_package_manager_args(package_manager: &String) -> Vec<&str> {
    match package_manager.as_str() {
        "npm" | "pnpm" => vec!["install"],
        "yarn" => vec![],
        _ => vec![],
    }
}
