pub const PACKAGE_LIST: [&str; 2] = ["lemlib", "ez"];

pub fn list() {
    println!("Packages:");
    for package in PACKAGE_LIST {
        println!("{package}");
    }
}
