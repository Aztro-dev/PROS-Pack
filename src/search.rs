use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct SearchArgs {
    pub name: String,
}

pub fn search_packages(args: SearchArgs) {
    let mut output: String = String::from("");
    for package in crate::list::PACKAGE_LIST {
        if package.contains(args.name.clone().as_str()) {
            output.push_str("\n");
            output.push_str(package);
        }
    }
    if output.is_empty() {
        println!("No packages with that sequence found!");
    } else {
        print!("Packages:");
        println!("{output}");
    }
}
