use super::*;
use crate::mime_type::MimeType;

#[derive(PartialEq)]
pub struct RunTimeImageWrapper {
    path: PathBuf,
    serialized: SerializedImageWrapper,
}

impl ImageWrapperMethods for RunTimeImageWrapper {
    fn new(_bytes: &'static [u8], path: PathBuf) -> Self {
        println!("Loading image wrapper from disk.");
        let serialized = SerializedImageWrapper::load_from_disk(&path);
        Self { path, serialized }
    }

    fn dimensions(&self) -> (u32, u32) {
        self.serialized.dimensions
    }

    fn generate_placeholder(&self, placeholder: Placeholder) -> GeneratedPlaceholder {
        let generated_placeholder = self.serialized.generated_placeholder.clone();

        if !placeholder.matches(&generated_placeholder) {
            panic!(
                "Placeholder mismatch for image at path: {:?}. Expected: {:?}, got: {:?}",
                &self.path, &placeholder, &generated_placeholder
            );
        }

        generated_placeholder
    }

    fn width(&self) -> u32 {
        self.dimensions().0
    }

    fn mime_type(&self) -> MimeType {
        self.serialized.mime_type
    }
}
