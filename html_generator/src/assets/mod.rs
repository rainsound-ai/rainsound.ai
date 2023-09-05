use crate::prelude::*;
use arraygen::Arraygen;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use std::any::Any;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use struct_iterable::Iterable;

mod path;
pub use self::path::*;

mod css_asset;
pub use self::css_asset::*;

mod html_asset;
pub use self::html_asset::*;

mod image_asset;
pub use self::image_asset::*;

mod js_asset;
pub use self::js_asset::*;

mod size_budget;
pub use self::size_budget::*;

mod text_asset;
pub use self::text_asset::*;

mod wasm_asset;
pub use self::wasm_asset::*;

pub static non_html_assets: Lazy<NonHtmlAssets> = Lazy::new(NonHtmlAssets::new);

#[derive(Arraygen)]
#[gen_array(fn html_assets_with_size_budget: &dyn NonImageAsset, implicit_select_all: HtmlAsset)]
pub struct Assets {
    html_assets: Vec<HtmlAsset>,
}

// We have to separate out the non-html assets because
// we want to reference them when generating html.
//
// If we didn't do this, we'd have a circular dependency.
// This causes problems. For example, it can lead to
// deadlocking if we're using a lazily initialized global variable.
#[derive(PartialEq, Arraygen)]
#[gen_array(fn assets_with_size_budget: &dyn NonImageAsset, implicit_select_all: CssAsset, JsAsset, WasmAsset, TextAsset)]
pub struct NonHtmlAssets {
    pub main_css: CssAsset,
    pub browser_js: JsAsset,
    pub browser_bg_wasm: WasmAsset,
    pub build_time: TextAsset,
    pub images: ImageAssets,
}

#[derive(PartialEq, Iterable)]
pub struct ImageAssets {
    pub hasui_hero: LightDarkImageAsset,
}

pub trait NonImageAsset {
    fn path(&self) -> &Path;

    fn bytes(&self) -> Vec<u8>;

    fn save_to_disk(&self, built_dir: &Path) {
        println!("Saving asset: {:?}", self.path());
        let path = self.path_on_disk(built_dir);

        if let Err(error) = fs::remove_file(&path) {
            println!("Error removing file: {}", error);
        }

        let bytes = self.bytes();
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, bytes).unwrap();
    }

    fn check_size_budget(&self) -> HowCloseToBudget {
        HowCloseToBudget::new(self)
    }

    fn path_on_disk(&self, built_dir: &Path) -> PathBuf {
        Assets::path_on_disk(built_dir, self.path())
    }

    fn size_budget(&self) -> NumBytes;
}

impl Assets {
    pub async fn new() -> Assets {
        let html_assets = crate::routes::get_pages().await;
        Assets { html_assets }
    }

    pub fn save_to_disk(&self, built_dir: &Path) {
        dbg!(&self.html_assets);

        for html_asset in &self.html_assets {
            println!("Saving asset: {:?}", html_asset.path());
            html_asset.save_to_disk(built_dir);
        }
        non_html_assets.save_to_disk(built_dir);
    }

    pub fn built_dir() -> PathBuf {
        manifest::dir().join("built")
    }

    fn path_on_disk(built_dir: &Path, asset_path: &Path) -> PathBuf {
        built_dir.join(asset_path)
    }
}

// impl Default for Assets {
// fn default() -> Self {
//     Self::new()
// }
// }

impl NonHtmlAssets {
    pub fn new() -> NonHtmlAssets {
        println!("main.css");
        let main_css = CssAsset {
            path: PathBuf::from_str("main.css").unwrap(),
            contents: include_str!("../../../target/tailwind/built.css"),
            size_budget: NumBytes(1),
        };

        println!("browser.js");
        let browser_js = JsAsset {
            path: PathBuf::from_str("browser.js").unwrap(),
            contents: include_str!("../../../target/browser/browser.js"),
            size_budget: NumBytes(1),
        };

        println!("browser_bg.wasm");
        let browser_bg_wasm = WasmAsset {
            path: PathBuf::from_str("browser_bg.wasm").unwrap(),
            bytes: include_bytes!("../../../target/browser/browser_bg.wasm"),
            size_budget: NumBytes(1),
        };

        println!("build_time.txt");
        let build_time = TextAsset {
            path: PathBuf::from_str("site-build-time").unwrap(),
            content: chrono::Local::now().to_rfc3339(),
            size_budget: NumBytes(1),
        };

        let images = ImageAssets::new();

        NonHtmlAssets {
            main_css,
            browser_js,
            browser_bg_wasm,
            build_time,
            images,
        }
    }

    fn save_to_disk(&self, built_dir: &Path) {
        println!("main.css");
        self.main_css.save_to_disk(built_dir);
        println!("browser.js");
        self.browser_js.save_to_disk(built_dir);
        println!("browser_bg.wasm");
        self.browser_bg_wasm.save_to_disk(built_dir);
        println!("build-time.json");
        self.build_time.save_to_disk(built_dir);

        self.images.save_to_disk(built_dir);
    }
}

impl Default for NonHtmlAssets {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageAssets {
    fn new() -> ImageAssets {
        println!("hasui-light.jpeg");
        let hasui_light = ImageAsset::new(
            PathBuf::from_str("hasui-light.jpeg").unwrap(),
            "A mountain in the distance",
            include_bytes!("./original_images/hasui_light.jpeg"),
        );

        println!("hasui-dark.jpeg");
        let hasui_dark = ImageAsset::new(
            PathBuf::from_str("hasui-dark.jpeg").unwrap(),
            "A town at night",
            include_bytes!("./original_images/hasui_dark.jpeg"),
        );

        ImageAssets {
            hasui_hero: LightDarkImageAsset {
                alt: "A woodblock print by Kawase Hasui. In dark mode, it's a town at night. In light mode, it's a mountain in the distance.",
                light_mode: hasui_light,
                dark_mode: hasui_dark,
            },
        }
    }

    fn save_to_disk(&self, built_dir: &Path) {
        let paths_of_images_in_built_dir = get_file_paths(built_dir);
        self.save_resized_image_assets_to_disk(built_dir, &paths_of_images_in_built_dir);
    }

    fn save_resized_image_assets_to_disk(
        &self,
        built_dir: &Path,
        paths_of_images_in_built_dir: &HashSet<PathBuf>,
    ) {
        self.iter()
            .flat_map(|(field_name, image_asset)| get_resized_variants(field_name, image_asset))
            .collect::<Vec<_>>()
            .into_iter()
            .par_bridge()
            .for_each(|resized_image_asset| {
                resized_image_asset.save_to_disk(built_dir, paths_of_images_in_built_dir);
            })
    }
}

fn get_resized_variants(field_name: &'static str, image_asset: &dyn Any) -> Vec<ResizedImageAsset> {
    if let Some(image_asset) = image_asset.downcast_ref::<ImageAsset>() {
        return image_asset.resized_variants.clone();
    }

    if let Some(image_asset) = image_asset.downcast_ref::<LightDarkImageAsset>() {
        return image_asset.resized_variants();
    }

    panic!("{} isn't an ImageAsset or LightDarkImageAsset", field_name);
}

fn get_file_paths(built_dir: &Path) -> HashSet<PathBuf> {
    let images_dir = built_dir.join("images");

    fs::read_dir(&images_dir)
        .unwrap_or_else(|error| {
            println!(
                "Error reading directory {:?}. Error message: {}",
                images_dir, error
            );
            fs::create_dir_all(&images_dir).unwrap();
            fs::read_dir(&images_dir).unwrap()
        })
        .map(|entry| entry.unwrap().path())
        .collect::<HashSet<PathBuf>>()
}
