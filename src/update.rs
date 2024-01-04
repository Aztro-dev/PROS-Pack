pub struct UpdateStatus {
    pub version: String,
}

impl UpdateStatus {
    pub fn new(version: &str) -> Self {
        return Self {
            version: version.to_string(),
        };
    }
}

pub fn update_cli() -> Option<UpdateStatus> {
    // TODO
    return Some(UpdateStatus::new("0.0.1"));
}
