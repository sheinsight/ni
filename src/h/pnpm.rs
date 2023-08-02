use super::package_manager::{AddArgs, InstallArgs, PackageManager};

pub struct Pnpm;

impl PackageManager for Pnpm {
    fn install(&self, args: InstallArgs) -> String {
        let InstallArgs { frozen_lockfile } = args;
        if frozen_lockfile {
            format!("pnpm install --frozen-lockfile")
        } else {
            format!("pnpm install")
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
            format!("pnpm add --save-prod {}", package)
        } else if save_dev {
            format!("pnpm add --save-dev {}", package)
        } else if save_optional {
            format!("pnpm add --save-optional {}", package)
        } else if save_exact {
            format!("pnpm add --save-exact {}", package)
        } else if global {
            format!("pnpm add --global {}", package)
        } else {
            format!("pnpm add --save-prod {}", package)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::package_manager::AddArgs;
    use super::*;

    #[test]
    fn test_install_with_default() {
        let pnpm = Pnpm {};
        let result = pnpm.install(InstallArgs {
            frozen_lockfile: false,
        });
        assert_eq!(result, "pnpm install")
    }

    #[test]
    fn test_install_with_frozen_lockfile_enable() {
        let pnpm = Pnpm {};
        let result = pnpm.install(InstallArgs {
            frozen_lockfile: true,
        });
        assert_eq!(result, "pnpm install --frozen-lockfile")
    }

    #[test]
    fn test_add_with_default_enable() {
        let pnpm = Pnpm {};
        let result = pnpm.add(
            "react",
            AddArgs {
                save: false,
                save_dev: false,
                save_optional: false,
                save_exact: false,
                global: false,
            },
        );
        assert_eq!(result, "pnpm add --save-prod react");
    }

    #[test]
    fn test_add_with_save_enable() {
        let pnpm = Pnpm {};
        let result = pnpm.add(
            "react",
            AddArgs {
                save: true,
                save_dev: false,
                save_optional: false,
                save_exact: false,
                global: false,
            },
        );
        assert_eq!(result, "pnpm add --save-prod react");
    }
    #[test]
    fn test_add_with_save_dev_enable() {
        let pnpm = Pnpm {};
        let result = pnpm.add(
            "react",
            AddArgs {
                save: false,
                save_dev: true,
                save_optional: false,
                save_exact: false,
                global: false,
            },
        );
        assert_eq!(result, "pnpm add --save-dev react");
    }
    #[test]
    fn test_add_with_save_optional_enable() {
        let pnpm = Pnpm {};
        let result = pnpm.add(
            "react",
            AddArgs {
                save: false,
                save_dev: false,
                save_optional: true,
                save_exact: false,
                global: false,
            },
        );
        assert_eq!(result, "pnpm add --save-optional react");
    }
    #[test]
    fn test_add_with_save_exact_enable() {
        let pnpm = Pnpm {};
        let result = pnpm.add(
            "react",
            AddArgs {
                save: false,
                save_dev: false,
                save_optional: false,
                save_exact: true,
                global: false,
            },
        );
        assert_eq!(result, "pnpm add --save-exact react");
    }

    #[test]
    fn test_add_with_global_enable() {
        let pnpm = Pnpm {};
        let result = pnpm.add(
            "react",
            AddArgs {
                save: false,
                save_dev: false,
                save_optional: false,
                save_exact: false,
                global: true,
            },
        );
        assert_eq!(result, "pnpm add --global react");
    }
}
