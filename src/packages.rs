use ron::de::from_str;

#[derive(Debug, serde::Deserialize)]
pub struct PackageInformation {
    /// lemlib, ez, etc.
    pub name: String,
    /// LemLib, EZ-Template, etc.
    pub package_name: String,
    /// https://github.com/LemLib/LemLib/releases/download/v0.4.7/LemLib@0.4.7.zip, etc.
    pub url: String,
    /// LemLib@0.4.7.zip, EZ-Template@2.2.0.zip, etc.
    pub zip_name: String,
}

impl PackageInformation {
    pub fn matches(&self, str: String) -> bool {
        return str == self.name
            || str == self.package_name
            || str == self.url
            || str == self.zip_name;
    }
}

/// Yes, this will be used multiple times, no I do not care
pub fn load_data() -> Vec<PackageInformation> {
    // So we can embed the package info in the executable
    let file: &str = core::str::from_utf8(include_bytes!("../packages.ron")).unwrap();
    let output: Vec<PackageInformation> = from_str(file).unwrap(); // Error handling mid
    return output;
}
