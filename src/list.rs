pub fn list() {
    println!("Packages:");
    let data = crate::packages::load_data();
    for package in data {
        println!("{}", package.name);
    }
}
