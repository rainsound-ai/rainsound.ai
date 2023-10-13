pub trait StringExtension {
    fn with_leading_slash(self) -> String;
}

impl StringExtension for String {
    fn with_leading_slash(mut self) -> String {
        if self.starts_with('/') {
            return self;
        }

        self.insert_str(0, "/");
        self
    }
}
