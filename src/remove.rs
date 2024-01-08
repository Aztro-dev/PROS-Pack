use clap::Args;

use crate::packages;

#[derive(Args, Debug, Clone)]
pub struct RemoveArgs {
    pub name: String,
}

pub fn remove_package(args: RemoveArgs) {
    let lowercase = args.name.clone().to_lowercase();
    let data = packages::load_data()
        .into_iter()
        .filter(|package| package.matches(lowercase.clone()))
        .next();
    if data.is_none() {
        println!("Couldn't recognize package, available packages are: ");

        let data = packages::load_data();

        for package_info in data.iter() {
            println!("{}", package_info.name);
        }
        std::process::exit(1);
    }
    let data = data.unwrap();

    let package_name = data.package_name.clone();

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
