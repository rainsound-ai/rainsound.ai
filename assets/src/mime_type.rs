use std::str::FromStr;

use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug, Copy)]
pub enum MimeType {
    ImageJpeg,
}

impl ToString for MimeType {
    fn to_string(&self) -> String {
        match self {
            MimeType::ImageJpeg => "image/jpeg".to_string(),
        }
    }
}

impl FromStr for MimeType {
    type Err = ();

    fn from_str(mime_type_string: &str) -> Result<Self, Self::Err> {
        match mime_type_string {
            "image/jpeg" => Ok(MimeType::ImageJpeg),
            _ => Err(()),
        }
    }
}

cfg_if! {
if #[cfg(feature = "build")] {
    use std::path::Path;

    impl MimeType {
        // pub fn from_bytes(bytes: &'static [u8]) -> MimeType {
        //     let mime_type_string = tree_magic_mini::from_u8(bytes);
        //     MimeType::from_str(mime_type_string).expect("Error parsing mime type.")
        // }
        pub fn from_path<P: AsRef<Path>>(path: P) -> MimeType {
            let mime_type_string = mime_guess::from_path(path).first().unwrap().to_string();
            MimeType::from_str(&mime_type_string).expect("Error parsing mime type.")
        }
    }
}
}
