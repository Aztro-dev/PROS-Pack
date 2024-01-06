use clap::Args;

const PACKAGE_NAMES: [&str; 2] = ["LemLib", "EZ"];

#[derive(Args, Debug, Clone)]
pub struct RemoveArgs {
    pub name: String,
}

pub fn remove_package(args: RemoveArgs) {
    let lowercase = args.name.clone().to_lowercase();
    let package_name = lowercase_to_package_name(lowercase);
    if package_name.is_none() {
        println!("Couldn't recognize package, available packages are: ");
        for package in PACKAGE_NAMES {
            println!("{package}");
        }
        std::process::exit(1);
    }

    let package_name = package_name.unwrap();

    if std::process::Command::new("pros")
        .args(["conductor", "uninstall", package_name.as_str()])
        .output()
        .is_err()
    {
        println!("Could not uninstall the package");
        std::process::exit(1);
    }
    println!("Uninstall success!");
}

// Probably have to refactor this too
fn lowercase_to_package_name(name: String) -> Option<String> {
    match name.as_str() {
        "lemlib" => Some("LemLib".to_string()),
        "ez" => Some("EZ-Template".to_string()),
        _ => None,
    }
}
