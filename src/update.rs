use clap::Args;

use serde_json::Value;

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

        for package in data.iter() {
            let package_name = package.package_name.clone();
            let template = &templates[package_name.as_str()];
            if template.is_null() {
                continue;
            }
            if template["version"] != package.get_version() {
                println!("Bruh");
            }
        }
    } else {
        print!("Updating ");
        let packages = update_args.packages.unwrap();
        println!("{:#?}", packages);
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
