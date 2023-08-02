pub struct AddArgs {
    pub save: bool,
    pub save_dev: bool,
    pub save_optional: bool,
    pub save_exact: bool,
    pub global: bool,
}

pub struct InstallArgs {
    pub frozen_lockfile: bool,
}

pub trait PackageManager {
    fn install(&self, args: InstallArgs) -> String;
    fn add(&self, package: &str, args: AddArgs) -> String;
}
