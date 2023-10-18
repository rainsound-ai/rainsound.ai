use super::*;
use cfg_if::cfg_if;

#[derive(PartialEq)]
pub struct LightDarkImageAsset {
    pub alt: &'static str,
    pub light_mode: ImageAsset,
    pub dark_mode: ImageAsset,
    pub placeholder: LightDarkPlaceholder,
}

impl LightDarkImageAsset {
    pub fn new(
        alt: &'static str,
        light_mode: ImageAsset,
        dark_mode: ImageAsset,
    ) -> LightDarkImageAsset {
        let placeholder =
            LightDarkPlaceholder::new(&light_mode.placeholder, &dark_mode.placeholder);

        LightDarkImageAsset {
            alt,
            light_mode,
            dark_mode,
            placeholder,
        }
    }

    pub fn resized_variants(&self) -> Vec<&ResizedImageAsset> {
        self.light_mode
            .resized_variants
            .iter()
            .chain(self.dark_mode.resized_variants.iter())
            .collect()
    }
}

cfg_if! {
if #[cfg(feature = "build")] {
    impl CanSaveToDisk for LightDarkImageAsset {
        fn save_to_disk(&self) {
            self.light_mode.save_to_disk();
            self.dark_mode.save_to_disk();
        }
    }
}
}

#[derive(PartialEq)]
pub enum LightDarkPlaceholder {
    Lqip {
        light_mode_data_uri: String,
        light_mode_mime_type: &'static str,
        dark_mode_data_uri: String,
        dark_mode_mime_type: &'static str,
    },
    Color {
        light_mode_css_string: String,
        dark_mode_css_string: String,
    },
}

impl LightDarkPlaceholder {
    pub fn new(
        light_mode: &GeneratedPlaceholder,
        dark_mode: &GeneratedPlaceholder,
    ) -> LightDarkPlaceholder {
        match (light_mode, dark_mode) {
            (
                GeneratedPlaceholder::Lqip {
                    data_uri: light_mode_data_uri,
                    mime_type: light_mode_mime_type,
                },
                GeneratedPlaceholder::Lqip {
                    data_uri: dark_mode_data_uri,
                    mime_type: dark_mode_mime_type,
                },
            ) => LightDarkPlaceholder::Lqip {
                light_mode_data_uri: light_mode_data_uri.clone(),
                light_mode_mime_type,
                dark_mode_data_uri: dark_mode_data_uri.clone(),
                dark_mode_mime_type,
            },
            (
                GeneratedPlaceholder::Color {
                    css_string: light_mode_css_string,
                },
                GeneratedPlaceholder::Color {
                    css_string: dark_mode_css_string,
                },
            ) => LightDarkPlaceholder::Color {
                light_mode_css_string: light_mode_css_string.clone(),
                dark_mode_css_string: dark_mode_css_string.clone(),
            },
            (_, _) => panic!(
                "When defining a light-dark image asset, all images must have the same kind of placeholder (LQIP or color)."
            ),
        }
    }
}
