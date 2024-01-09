use clap::Args;

use std::process::Command;

#[derive(Args, Debug, Clone)]
pub struct InstallArgs {
    pub name: String,
}

pub fn install_package(args: InstallArgs) {
    let name = args.name;

    let data = crate::packages::load_data()
        .into_iter()
        .find(|package| package.matches(name.clone()));

    if data.is_none() {
        println!("Package \"{}\" could not be found. Try checking for typos or if the package is registered.", name);
        std::process::exit(1);
    }

    let data = data.unwrap();

    let download_link = data.url.clone();

    let current_dir = crate::utils::change_to_root_dir();

    // Download zip file
    if Command::new("wget")
        .args([download_link.as_str()])
        .output()
        .is_err()
    {
        println!("Could not download the package. Check if the command \"wget\" is installed.");
        std::process::exit(1);
    }

    let zip_name = data.zip_name.clone();

    if Command::new("pros")
        .args(["c", "fetch", zip_name.clone().as_str()])
        .output()
        .is_err()
    {
        println!(
            "Could not run \"pros c fetch {}\". Check if \"pros\" is installed.",
            zip_name.clone()
        );
        // Remove file if pros didn't work
        if Command::new("rm")
            .args(["-f", zip_name.clone().as_str()])
            .output()
            .is_err()
        {
            println!("Could not remove zip after downloading");
        }
        std::process::exit(1);
    }

    let package_name = data.package_name.clone();
    if Command::new("pros")
        .args(["c", "apply", package_name.clone().as_str()])
        .output()
        .is_err()
    {
        println!(
            "Could not run \"pros c apply {}\". Check if \"pros\" is installed.",
            package_name.clone()
        );
        // Remove file if pros didn't work
        if Command::new("rm")
            .args(["-f", zip_name.clone().as_str()])
            .output()
            .is_err()
        {
            println!("Could not remove zip after downloading");
        }
        std::process::exit(1);
    }

    if Command::new("rm")
        .args(["-f", zip_name.clone().as_str()])
        .output()
        .is_err()
    {
        println!("Could not remove zip after downloading");
        std::process::exit(1);
    }

    if crate::utils::find_root_dir().unwrap() != current_dir {
        std::env::set_current_dir(current_dir.clone()).unwrap();
    }
}
