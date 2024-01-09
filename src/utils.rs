/// Returns the current dir
pub fn change_to_root_dir() -> String {
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

    return current_dir;
}

pub fn find_root_dir() -> Option<String> {
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
