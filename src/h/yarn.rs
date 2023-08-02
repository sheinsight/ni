use crate::h::package_manager::AddArgs;

use super::package_manager::{InstallArgs, PackageManager};

pub struct Yarn;

impl PackageManager for Yarn {
    fn install(&self, args: InstallArgs) -> String {
        let InstallArgs { frozen_lockfile } = args;
        if frozen_lockfile {
            format!("yarn install --frozen-lockfile")
        } else {
            format!("yarn install")
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
            format!("yarn add {}", package)
        } else if save_dev {
            format!("yarn add {} --dev", package)
        } else if save_optional {
            format!("yarn add {} --optional", package)
        } else if save_exact {
            format!("yarn add {} --exact", package)
        } else if global {
            format!("yarn global add {}", package)
        } else {
            format!("yarn add {}", package)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::package_manager::AddArgs;
    use super::*;

    #[test]
    fn test_install_with_default() {
        let yarn = Yarn {};
        let result = yarn.install(InstallArgs {
            frozen_lockfile: false,
        });
        assert_eq!(result, "yarn install")
    }

    #[test]
    fn test_install_with_frozen_lockfile_enable() {
        let yarn = Yarn {};
        let result = yarn.install(InstallArgs {
            frozen_lockfile: true,
        });
        assert_eq!(result, "yarn install --frozen-lockfile")
    }

    #[test]
    fn test_add_with_default_enable() {
        let yarn = Yarn {};
        let result = yarn.add(
            "react",
            AddArgs {
                save: false,
                save_dev: false,
                save_optional: false,
                save_exact: false,
                global: false,
            },
        );
        assert_eq!(result, "yarn add react");
    }

    #[test]
    fn test_add_with_save_enable() {
        let yarn = Yarn {};
        let result = yarn.add(
            "react",
            AddArgs {
                save: true,
                save_dev: false,
                save_optional: false,
                save_exact: false,
                global: false,
            },
        );
        assert_eq!(result, "yarn add react");
    }
    #[test]
    fn test_add_with_save_dev_enable() {
        let yarn = Yarn {};
        let result = yarn.add(
            "react",
            AddArgs {
                save: false,
                save_dev: true,
                save_optional: false,
                save_exact: false,
                global: false,
            },
        );
        assert_eq!(result, "yarn add react --dev");
    }
    #[test]
    fn test_add_with_save_optional_enable() {
        let yarn = Yarn {};
        let result = yarn.add(
            "react",
            AddArgs {
                save: false,
                save_dev: false,
                save_optional: true,
                save_exact: false,
                global: false,
            },
        );
        assert_eq!(result, "yarn add react --optional");
    }
    #[test]
    fn test_add_with_save_exact_enable() {
        let yarn = Yarn {};
        let result = yarn.add(
            "react",
            AddArgs {
                save: false,
                save_dev: false,
                save_optional: false,
                save_exact: true,
                global: false,
            },
        );
        assert_eq!(result, "yarn add react --exact");
    }

    #[test]
    fn test_add_with_global_enable() {
        let yarn = Yarn {};
        let result = yarn.add(
            "react",
            AddArgs {
                save: false,
                save_dev: false,
                save_optional: false,
                save_exact: false,
                global: true,
            },
        );
        assert_eq!(result, "yarn global add react");
    }
}
