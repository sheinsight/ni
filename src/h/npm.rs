use crate::h::package_manager::AddArgs;

use super::package_manager::{InstallArgs, PackageManager};

pub struct Npm;

impl PackageManager for Npm {
    fn install(&self, args: InstallArgs) -> String {
        let InstallArgs { frozen_lockfile } = args;
        if frozen_lockfile {
            format!("npm clean-install")
        } else {
            format!("npm install")
        }
    }

    fn add(&self, package: &str, args: AddArgs) -> String {
        let AddArgs {
            save,
            save_dev,
            save_optional,
            save_exact,
            global,
        } = args;
        if save {
            format!("npm add {} --save-prod", package)
        } else if save_dev {
            format!("npm add {} --save-dev", package)
        } else if save_optional {
            format!("npm add {} --save-optional", package)
        } else if save_exact {
            format!("npm add {} --save-exact", package)
        } else if global {
            format!("npm add {} --global", package)
        } else {
            format!("npm add {} --save-prod", package)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::package_manager::AddArgs;
    use super::*;

    #[test]
    fn test_install_with_default() {
        let npm = Npm {};
        let result = npm.install(InstallArgs {
            frozen_lockfile: false,
        });
        assert_eq!(result, "npm install")
    }

    #[test]
    fn test_install_with_frozen_lockfile_enable() {
        let npm = Npm {};
        let result = npm.install(InstallArgs {
            frozen_lockfile: true,
        });
        assert_eq!(result, "npm clean-install")
    }

    #[test]
    fn test_add_with_default() {
        let npm = Npm {};
        let result = npm.add(
            "react",
            AddArgs {
                save: false,
                save_dev: false,
                save_optional: false,
                save_exact: false,
                global: false,
            },
        );
        assert_eq!(result, "npm add react --save-prod");
    }

    #[test]
    fn test_add_with_save_enable() {
        let npm = Npm {};
        let result = npm.add(
            "react",
            AddArgs {
                save: true,
                save_dev: false,
                save_optional: false,
                save_exact: false,
                global: false,
            },
        );
        assert_eq!(result, "npm add react --save-prod");
    }
    #[test]
    fn test_add_with_save_dev_enable() {
        let npm = Npm {};
        let result = npm.add(
            "react",
            AddArgs {
                save: false,
                save_dev: true,
                save_optional: false,
                save_exact: false,
                global: false,
            },
        );
        assert_eq!(result, "npm add react --save-dev");
    }
    #[test]
    fn test_add_with_save_optional_enable() {
        let npm = Npm {};
        let result = npm.add(
            "react",
            AddArgs {
                save: false,
                save_dev: false,
                save_optional: true,
                save_exact: false,
                global: false,
            },
        );
        assert_eq!(result, "npm add react --save-optional");
    }
    #[test]
    fn test_add_with_save_exact_enable() {
        let npm = Npm {};
        let result = npm.add(
            "react",
            AddArgs {
                save: false,
                save_dev: false,
                save_optional: false,
                save_exact: true,
                global: false,
            },
        );
        assert_eq!(result, "npm add react --save-exact");
    }

    #[test]
    fn test_add_with_global_enable() {
        let npm = Npm {};
        let result = npm.add(
            "react",
            AddArgs {
                save: false,
                save_dev: false,
                save_optional: false,
                save_exact: false,
                global: true,
            },
        );
        assert_eq!(result, "npm add react --global");
    }
}
