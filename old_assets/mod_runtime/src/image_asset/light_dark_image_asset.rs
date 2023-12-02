use super::*;

pub struct LightDarkImageAsset {
    pub light_mode: ImageAsset,
    pub dark_mode: ImageAsset,
    pub placeholder: LightDarkPlaceholder,
}

impl LightDarkImageAsset {
    pub fn new(light_mode: ImageAsset, dark_mode: ImageAsset) -> LightDarkImageAsset {
        let placeholder =
            LightDarkPlaceholder::new(&light_mode.placeholder, &dark_mode.placeholder);

        LightDarkImageAsset {
            light_mode,
            dark_mode,
            placeholder,
        }
    }

    pub fn resized_copies(&self) -> Vec<&RunTimeResizedImage> {
        self.light_mode
            .resized_copies
            .iter()
            .chain(self.dark_mode.resized_copies.iter())
            .collect()
    }
}

impl Asset for LightDarkImageAsset {
    fn files_to_save(&self) -> Vec<FileToSave> {
        let light_mode_files_to_save = self.light_mode.files_to_save().into_iter();
        let dark_mode_files_to_save = self.dark_mode.files_to_save().into_iter();

        light_mode_files_to_save
            .chain(dark_mode_files_to_save)
            .collect()
    }
}

#[derive(PartialEq)]
pub enum LightDarkPlaceholder {
    Lqip {
        light_mode_data_uri: String,
        dark_mode_data_uri: String,
    },
    Color {
        light_mode_css_string: String,
        dark_mode_css_string: String,
    },
}

impl LightDarkPlaceholder {
    pub fn new(
        light_mode: &BuiltPlaceholder,
        dark_mode: &BuiltPlaceholder,
    ) -> LightDarkPlaceholder {
        match (light_mode, dark_mode) {
            (
                BuiltPlaceholder::Lqip {
                    data_uri: light_mode_data_uri,
                },
                BuiltPlaceholder::Lqip {
                    data_uri: dark_mode_data_uri,
                },
            ) => LightDarkPlaceholder::Lqip {
                light_mode_data_uri: light_mode_data_uri.clone(),
                dark_mode_data_uri: dark_mode_data_uri.clone(),
            },
            (
                BuiltPlaceholder::Color {
                    css_string: light_mode_css_string,
                },
                BuiltPlaceholder::Color {
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
