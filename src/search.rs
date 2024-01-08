use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct SearchArgs {
    pub name: String,
}

pub fn search_packages(args: SearchArgs) {
    let mut output: String = String::from("");
    for package_information in crate::packages::load_data().iter() {
        if package_information.matches(args.name.clone()) {
            output.push_str("\n");
            output.push_str(package_information.package_name.as_str());
        }
    }
    if output.is_empty() {
        println!("No packages with that sequence found!");
    } else {
        print!("Packages:");
        println!("{output}");
    }
}
