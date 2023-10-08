// use crate::prelude::*;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use std::any::Any;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use struct_iterable::Iterable;

mod assets;
pub use self::assets::*;

mod css_asset;
pub use self::css_asset::*;

mod extensions;
pub use self::extensions::*;

mod html_asset;
pub use self::html_asset::*;

mod image_asset;
pub use self::image_asset::*;

mod js_asset;
pub use self::js_asset::*;

mod non_image_asset;
pub use self::non_image_asset::*;

mod non_html_assets;
pub use self::non_html_assets::*;

mod path;
pub use self::path::*;

mod performance_budget;
pub use self::performance_budget::*;

mod prelude;
pub use self::prelude::*;

pub mod tailwind;
pub use self::tailwind::*;

mod text_asset;
pub use self::text_asset::*;

mod wasm_asset;
pub use self::wasm_asset::*;

mod workspace_root;
pub use self::workspace_root::*;

pub static non_html_assets: Lazy<NonHtmlAssets> = Lazy::new(NonHtmlAssets::new);


#[derive(PartialEq, Iterable)]
pub struct ImageAssets {
    pub hasui_hero: LightDarkImageAsset,
}


impl Assets {
    pub fn new() -> Assets {
        // let html_assets = HtmlAsset::get_pages();
        Assets { 
            // html_assets
        }
    }

    pub fn save_to_disk(&self, built_dir: &Path) {
        // dbg!(&self.html_assets);

        // for html_asset in &self.html_assets {
        //     println!("Saving asset: {:?}", html_asset.path());
        //     html_asset.save_to_disk(built_dir);
        // }
        // non_html_assets.save_to_disk(built_dir);
    }

    pub fn built_dir() -> PathBuf {
        crate::workspace_root::dir().join("built")
    }

    fn path_on_disk(built_dir: &Path, asset_path: &Path) -> PathBuf {
        built_dir.join(asset_path)
    }
}

impl Default for Assets {
    fn default() -> Self {
        Self::new()
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
        // let main_css = CssAsset {
        //     path: PathBuf::from_str("main.css").unwrap(),
        //     contents: include_str!("../../../target/tailwind/built.css"),
        //     load_time_budget: Duration::from_millis(1),
        // };

        // println!("browser.js");
        // let browser_js = JsAsset {
        //     path: PathBuf::from_str("browser.js").unwrap(),
        //     contents: include_str!("../../../target/browser/browser.js"),
        //     load_time_budget: Duration::from_millis(1),
        // };

        // println!("browser_bg.wasm");
        // let browser_bg_wasm = WasmAsset {
        //     path: PathBuf::from_str("browser_bg.wasm").unwrap(),
        //     bytes: include_bytes!("../../../target/browser/browser_bg.wasm"),
        //     load_time_budget: Duration::from_millis(1),
        // };

        // println!("build_time.txt");
        // let build_time = TextAsset {
        //     path: PathBuf::from_str("site-build-time").unwrap(),
        //     content: chrono::Local::now().to_rfc3339(),
        //     load_time_budget: Duration::from_millis(1),
        // };

        let images = ImageAssets::new();

        NonHtmlAssets {
            // main_css,
            // browser_js,
            // browser_bg_wasm,
            // build_time,
            // images,
        }
    }

    fn save_to_disk(&self, built_dir: &Path) {
        // println!("main.css");
        // self.main_css.save_to_disk(built_dir);
        // println!("browser.js");
        // self.browser_js.save_to_disk(built_dir);
        // println!("browser_bg.wasm");
        // self.browser_bg_wasm.save_to_disk(built_dir);
        // println!("build-time.json");
        // self.build_time.save_to_disk(built_dir);

        // self.images.save_to_disk(built_dir);
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
            Placeholder::Lqip,
        );

        println!("hasui-dark.jpeg");
        let hasui_dark = ImageAsset::new(
            PathBuf::from_str("hasui-dark.jpeg").unwrap(),
            "A town at night",
            include_bytes!("./original_images/hasui_dark.jpeg"),
            Placeholder::Lqip,
        );

        ImageAssets {
            hasui_hero: LightDarkImageAsset::new(
                "A woodblock print by Kawase Hasui. In dark mode, it's a town at night. In light mode, it's a mountain in the distance.",
                hasui_light,
                hasui_dark,
            ),
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
