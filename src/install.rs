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

    let current_dir = std::env::current_dir()
        .unwrap()
        .canonicalize() // To absolute string
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let root_dir = find_root_dir();

    if root_dir.is_none() {
        println!("Are you in a pros project? Try changing the to directory of your project and run this command again");
        std::process::exit(1);
    }
    let root_dir = root_dir.unwrap();

    if root_dir != current_dir {
        std::env::set_current_dir(root_dir.clone()).unwrap();
    }

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

    if root_dir != current_dir {
        std::env::set_current_dir(current_dir.clone()).unwrap();
    }
}

fn find_root_dir() -> Option<String> {
    loop {
        let current_dir = std::env::current_dir()
            .ok()?
            .as_path()
            .canonicalize()
            .ok()?; // To absolute string

        for entry in std::fs::read_dir(current_dir.clone()).ok()? {
            let entry = entry.unwrap();
            let (f_name, path) = (entry.file_name(), entry.path());

            if f_name.to_string_lossy() == "project.pros" {
                return Some(path.parent().unwrap().to_str().unwrap().to_string());
            }
        }

        if current_dir.parent().is_none() {
            // parent() returns None if the path terminates in a root or prefix, or if it’s an empty string.
            break;
        }
        std::env::set_current_dir(current_dir.parent()?).ok()?;
    }
    return None;
}
