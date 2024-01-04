use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct RemoveArgs {
    pub name: String,
}

pub fn remove_package(args: RemoveArgs) {
    println!("Remove args: {:?}", args);
}
