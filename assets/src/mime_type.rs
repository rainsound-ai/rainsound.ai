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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "image/jpeg" => Ok(MimeType::ImageJpeg),
            _ => Err(()),
        }
    }
}

cfg_if! {
if #[cfg(feature = "build")] {
    impl MimeType {
        pub fn from_bytes(bytes: &'static [u8]) -> MimeType {
            let mime_type_string = tree_magic::from_u8(bytes);
            MimeType::from_str(&mime_type_string).expect("Error parsing mime type.")
        }
    }
}
}
