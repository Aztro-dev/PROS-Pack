use clap::Args;

use serde_json::Value;

use crate::install;
use crate::remove;

#[derive(Debug, Clone, Args)]
pub struct UpdateArgs {
    pub packages: Option<Vec<String>>,
}

pub fn update_packages(update_args: UpdateArgs) {
    let current_dir = crate::utils::change_to_root_dir();

    let templates = &read_project_pros().unwrap()["py/state"]["templates"];

    let data = crate::packages::load_data();

    if update_args.packages.is_none() {
        println!("Updating all packages...");

        // For all recognized packages
        for package in data.iter() {
            let package_name = package.package_name.clone();

            // Check if current package is installed
            let template = &templates[package_name.as_str()];
            if template.is_null() {
                // Not installed
                continue;
            }

            if template["version"].as_str().unwrap() == package.get_version() {
                println!("{} already up to date", package_name);
                continue;
            }

            remove::remove_package(remove::RemoveArgs {
                name: package_name.clone(),
            });
            install::install_package(install::InstallArgs {
                name: package_name.clone(),
            });

            println!(
                "Updated {} {} => {}",
                package_name,
                template["version"].as_str().unwrap(),
                package.get_version()
            );
        }
    } else {
        let packages = update_args.packages.unwrap();

        // For every one of the requested packages
        for package_name in packages.iter() {
            let mut package: Option<&crate::packages::PackageInformation> = None;

            for p in data.iter() {
                if p.matches(package_name.clone()) {
                    package = Some(p);
                    break;
                }
            }

            // Check if the package is even installed
            if package.is_none() {
                println!("Package not recognized");
                continue;
            }

            let package = package.unwrap();

            let template = &templates[package.package_name.as_str()];
            if template.is_null() {
                println!(
                    "{} isn\'t installed, continuing with the rest of the packages",
                    package_name
                );
                continue;
            }

            // Get rid of quotation marks
            let template_version = template["version"].as_str().unwrap().trim();
            if template_version == package.get_version() {
                println!("{} already up to date", package_name);
                continue;
            }

            remove::remove_package(remove::RemoveArgs {
                name: package_name.clone(),
            });
            install::install_package(install::InstallArgs {
                name: package_name.clone(),
            });

            println!(
                "Updated {} {} => {}",
                package_name.clone(),
                template_version,
                package.get_version()
            );
        }
    }

    if crate::utils::find_root_dir().unwrap() != current_dir {
        std::env::set_current_dir(current_dir.clone()).unwrap();
    }
}

/// Assumes we are already in the root directory
fn read_project_pros() -> Option<Value> {
    // TODO: Handle errors
    let contents = std::fs::read_to_string("project.pros").ok()?;
    let output = serde_json::from_str(contents.as_str());
    return output.ok();
}
