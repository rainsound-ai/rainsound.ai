use crate::prelude::*;
use image::imageops::FilterType::Lanczos3;
use image::{DynamicImage, GenericImageView};
use rayon::prelude::*;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;

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

    pub fn resized_variants(&self) -> Vec<ResizedImageAsset> {
        self.light_mode
            .resized_variants
            .iter()
            .chain(self.dark_mode.resized_variants.iter())
            .cloned()
            .collect()
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

#[derive(PartialEq)]
pub struct ImageAsset {
    pub path: PathBuf,
    pub alt: &'static str,
    pub bytes: &'static [u8],
    pub placeholder: GeneratedPlaceholder,

    mime_type: String,

    pub width: u32,
    pub height: u32,
    pub resized_variants: Vec<ResizedImageAsset>,

    srcset: String,
    image: Arc<DynamicImage>,
}

impl ImageAsset {
    pub fn new(
        path: PathBuf,
        alt: &'static str,
        bytes: &'static [u8],
        placeholder: Placeholder,
    ) -> ImageAsset {
        let path = PathBuf::from_str("images/").unwrap().join(path);
        let image = image::load_from_memory(bytes).unwrap();
        let image = Arc::new(image);
        let (width, height) = image.dimensions();
        let srcset = Self::create_srcset(&path, width);
        let resized_variants = Self::resized_variants(&path, &image);
        let mime_type = tree_magic::from_u8(bytes);

        ImageAsset {
            path,
            alt,
            bytes,
            placeholder: placeholder.generate(image.clone()),
            width,
            height,
            srcset,
            image,
            resized_variants,
            mime_type,
        }
    }

    pub fn src(&self) -> &str {
        // If their browser doesn't have support for the srcset attribute,
        // it's probably an old mobile browser. If that's the case, they
        // also probably don't have a lot of bandwidth so go with the smallest
        // image possible.
        self.resized_variants
            .first()
            .unwrap()
            .path
            .to_str()
            .unwrap()
    }

    pub fn srcset(&self) -> &str {
        &self.srcset
    }

    pub fn mime_type(&self) -> &str {
        &self.mime_type
    }

    fn resized_variants(path: &Path, original_image: &Arc<DynamicImage>) -> Vec<ResizedImageAsset> {
        let original_width = original_image.width();

        Self::available_widths(original_width)
            .into_par_iter()
            .map(|target_width| ResizedImageAsset {
                path: Self::path_with_width(path, target_width),
                width: target_width,
                image: original_image.clone(),
            })
            .collect()
    }

    fn create_srcset(path: &Path, image_width: u32) -> String {
        Self::available_widths(image_width)
            .into_iter()
            .map(|width| {
                let path_with_width = Self::path_with_width(path, width);
                let path_string = path_with_width.to_str().unwrap();
                format!("{path_string} {width}w")
            })
            .collect::<Vec<String>>()
            .join(", ")
    }

    fn available_widths(image_width: u32) -> Vec<u32> {
        Self::possible_widths()
            .into_iter()
            .filter(|possible_width| possible_width <= &image_width)
            .collect()
    }

    fn possible_widths() -> Vec<u32> {
        (100..=4000).step_by(100).collect()
    }

    fn path_with_width(path: &Path, width: u32) -> PathBuf {
        let old_file_stem = path.file_stem().unwrap().to_str().unwrap();
        let old_file_extension = path.extension().unwrap().to_str().unwrap();
        let new_file_name = format!("{}-{}w.{}", old_file_stem, width, old_file_extension);
        path.with_file_name(new_file_name)
    }
}

#[derive(PartialEq, Clone)]
pub struct ResizedImageAsset {
    path: PathBuf,
    width: u32,
    image: Arc<DynamicImage>,
}

impl ResizedImageAsset {
    pub fn save_to_disk(&self, built_dir: &Path, paths_of_files_in_built_dir: &HashSet<PathBuf>) {
        println!("Deciding whether to save resized image to disk.");
        let path = Assets::path_on_disk(built_dir, &self.path);
        if self.needs_to_be_recreated(&path, paths_of_files_in_built_dir) {
            let parent_dir = path.parent().unwrap();
            if !parent_dir.exists() {
                std::fs::create_dir_all(parent_dir).unwrap();
            }

            println!("Saving resized image to disk: {:?}", &self.path);
            self.image
                .resize_to_width(self.width)
                .save_with_format(path, image::ImageFormat::Jpeg)
                .unwrap();

            return;
        }

        println!(
            "Resized image {} already exists, so skipping saving it to disk.",
            &self.path.to_str().unwrap()
        );
    }

    pub fn needs_to_be_recreated(
        &self,
        path_to_resized_image: &Path,
        paths_of_files_in_built_dir: &HashSet<PathBuf>,
    ) -> bool {
        // let image_on_disk = image::open(path_to_resized_image).unwrap();
        // if image_on_disk.width() == self.width {
        //     return false;
        // }
        let already_exists = paths_of_files_in_built_dir.contains(path_to_resized_image);
        !already_exists
    }
}

pub enum Placeholder {
    Lqip,
    AutomaticColor,
    Color { css_string: String },
}

impl Placeholder {
    fn generate(self, image: Arc<DynamicImage>) -> GeneratedPlaceholder {
        match self {
            Placeholder::Lqip => {
                let DataUriAndMimeType {
                    data_uri,
                    mime_type,
                } = image.resize_to_width(40).to_data_uri();

                GeneratedPlaceholder::Lqip {
                    data_uri,
                    mime_type,
                }
            }

            Placeholder::Color { css_string } => GeneratedPlaceholder::Color { css_string },

            Placeholder::AutomaticColor => {
                let [red, green, blue, alpha] =
                    image.resize_exact(1, 1, Lanczos3).get_pixel(0, 0).0;

                let css_string = format!(
                    "rgba({red}, {green}, {blue}, {alpha})",
                    red = red,
                    green = green,
                    blue = blue,
                    alpha = alpha
                );

                GeneratedPlaceholder::Color { css_string }
            }
        }
    }
}

#[derive(PartialEq)]
pub enum GeneratedPlaceholder {
    Lqip {
        data_uri: String,
        mime_type: &'static str,
    },
    Color {
        css_string: String,
    },
}
