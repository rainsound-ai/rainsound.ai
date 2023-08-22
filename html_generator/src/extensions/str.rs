pub trait StrExtension {
    fn with_leading_slash(self) -> String;
}

impl StrExtension for &str {
    fn with_leading_slash(self) -> String {
        if self.starts_with('/') {
            self.to_string()
        } else {
            format!("/{}", self)
        }
    }
}
