use crate::prelude::*;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

mod asset_path;
pub use asset_path::*;

mod css_asset;
use css_asset::*;

mod html_asset;
use html_asset::*;

mod image_asset;
pub use image_asset::*;

mod js_asset;
use js_asset::*;

mod text_asset;
use text_asset::*;

mod wasm_asset;
use wasm_asset::*;

pub static non_html_assets: Lazy<NonHtmlAssets> = Lazy::new(NonHtmlAssets::new);

pub struct Assets {
    index_html: HtmlAsset,
}

// We have to separate out the non-html assets because
// we want to reference them when generating html.
//
// If we didn't do this, we'd have a circular dependency.
// This causes problems. For example, it can lead to
// deadlocking if we're using a lazily initialized global variable.
#[derive(PartialEq)]
pub struct NonHtmlAssets {
    pub main_css: CssAsset,
    pub browser_js: JsAsset,
    pub browser_bg_wasm: WasmAsset,
    pub build_time: TextAsset,
    pub images: ImageAssets,
}

#[derive(PartialEq)]
pub struct ImageAssets {
    pub hasui_hero: LightDarkImageAsset,
}

pub trait NonImageAsset {
    fn asset_path(&self) -> &str;

    fn bytes(&self) -> Vec<u8>;

    fn save_to_disk(&self, built_dir: &Path) {
        println!("Saving asset: {:?}", self.asset_path());
        let path = Assets::path_on_disk(built_dir, self.asset_path());
        if let Err(error) = fs::remove_file(&path) {
            println!("Error removing file: {}", error);
        }
        let bytes = self.bytes();
        fs::write(path, bytes).unwrap();
    }
}

impl Assets {
    pub fn new() -> Assets {
        let index_html = HtmlAsset {
            asset_path: "index.html",
            contents: crate::routes::get(),
        };

        Assets { index_html }
    }

    pub fn save_to_disk(&self, built_dir: &Path) {
        println!("index.html");
        self.index_html.save_to_disk(built_dir);
        non_html_assets.save_to_disk(built_dir);
    }

    fn path_on_disk(build_dir: &Path, asset_path: impl Into<String>) -> PathBuf {
        build_dir.join(asset_path.into()).to_str().unwrap().into()
    }
}

impl Default for Assets {
    fn default() -> Self {
        Self::new()
    }
}

// fn save_build_time() {
//     let mut file = fs::File::create("../target/build_time.txt").unwrap();
//     let build_time = chrono::Local::now().to_rfc3339();
//     write!(file, "{}", build_time).ok();
// }
impl NonHtmlAssets {
    pub fn new() -> NonHtmlAssets {
        println!("main.css");
        let main_css = CssAsset {
            asset_path: "main.css",
            contents: include_str!("../../../target/tailwind/built.css"),
        };

        println!("browser.js");
        let browser_js = JsAsset {
            asset_path: "browser.js",
            contents: include_str!("../../../target/browser/browser.js"),
        };

        println!("browser_bg.wasm");
        let browser_bg_wasm = WasmAsset {
            asset_path: "browser_bg.wasm",
            bytes: include_bytes!("../../../target/browser/browser_bg.wasm"),
        };

        println!("build_time.txt");
        let build_time = TextAsset {
            asset_path: "build-time",
            content: chrono::Local::now().to_rfc3339(),
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
            "hasui-light.jpeg",
            "A mountain in the distance",
            include_bytes!("./original_images/hasui_light.jpeg"),
            // "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQABLAEsAAD/4QDORXhpZgAATU0AKgAAAAgABgESAAMAAAABAAEAAAEaAAUAAAABAAAAVgEbAAUAAAABAAAAXgEoAAMAAAABAAIAAAExAAIAAAAVAAAAZodpAAQAAAABAAAAfAAAAAAAAAEsAAAAAQAAASwAAAABUGl4ZWxtYXRvciBQcm8gMy4zLjkAAAAEkAQAAgAAABQAAACyoAEAAwAAAAEAAQAAoAIABAAAAAEAAAAkoAMABAAAAAEAAAAXAAAAADIwMjM6MDc6MjIgMTg6NTA6MTEA/+EJyWh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC8APD94cGFja2V0IGJlZ2luPSLvu78iIGlkPSJXNU0wTXBDZWhpSHpyZVN6TlRjemtjOWQiPz4gPHg6eG1wbWV0YSB4bWxuczp4PSJhZG9iZTpuczptZXRhLyIgeDp4bXB0az0iWE1QIENvcmUgNi4wLjAiPiA8cmRmOlJERiB4bWxuczpyZGY9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkvMDIvMjItcmRmLXN5bnRheC1ucyMiPiA8cmRmOkRlc2NyaXB0aW9uIHJkZjphYm91dD0iIiB4bWxuczp4bXA9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC8iIHhtcDpDcmVhdG9yVG9vbD0iUGl4ZWxtYXRvciBQcm8gMy4zLjkiIHhtcDpDcmVhdGVEYXRlPSIyMDIzLTA3LTIyVDE4OjUwOjExLTA0OjAwIiB4bXA6TWV0YWRhdGFEYXRlPSIyMDIzLTA3LTIyVDE4OjUwOjQxLTA0OjAwIi8+IDwvcmRmOlJERj4gPC94OnhtcG1ldGE+ICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgPD94cGFja2V0IGVuZD0idyI/PgD/7QBkUGhvdG9zaG9wIDMuMAA4QklNBAQAAAAAACwcAVoAAxslRxwCAAACAAIcAj4ACDIwMjMwNzIyHAI/AAsxODUwMTEtMDQwMDhCSU0EJQAAAAAAEMq/d0QIWqM9c5IZ8KAa21D/wAARCAAXACQDAREAAhEBAxEB/8QAHwAAAQUBAQEBAQEAAAAAAAAAAAECAwQFBgcICQoL/8QAtRAAAgEDAwIEAwUFBAQAAAF9AQIDAAQRBRIhMUEGE1FhByJxFDKBkaEII0KxwRVS0fAkM2JyggkKFhcYGRolJicoKSo0NTY3ODk6Q0RFRkdISUpTVFVWV1hZWmNkZWZnaGlqc3R1dnd4eXqDhIWGh4iJipKTlJWWl5iZmqKjpKWmp6ipqrKztLW2t7i5usLDxMXGx8jJytLT1NXW19jZ2uHi4+Tl5ufo6erx8vP09fb3+Pn6/8QAHwEAAwEBAQEBAQEBAQAAAAAAAAECAwQFBgcICQoL/8QAtREAAgECBAQDBAcFBAQAAQJ3AAECAxEEBSExBhJBUQdhcRMiMoEIFEKRobHBCSMzUvAVYnLRChYkNOEl8RcYGRomJygpKjU2Nzg5OkNERUZHSElKU1RVVldYWVpjZGVmZ2hpanN0dXZ3eHl6goOEhYaHiImKkpOUlZaXmJmaoqOkpaanqKmqsrO0tba3uLm6wsPExcbHyMnK0tPU1dbX2Nna4uPk5ebn6Onq8vP09fb3+Pn6/9sAQwABAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEB/9sAQwEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEB/90ABAAF/9oADAMBAAIRAxEAPwD9/wDwv8Ufgj8SfFlt4U+Hf7QHwe8eeKtdmuZdP8OeGfil4J8Qa9rUlnbS3WoS2ej6Zq93fX8Vhp9vJc3skMKv9mSS5cKgllb9Hw+dYOFOHOoRUI6WXL7NaP3WuWzvL3U4qyV7ae7+P1sizWdSTjCpPnkk3KEpKUoqUW7PRq9m+Z+9d7anvt38MNR0SybUNdv7TQ7BblcXus6rp+nQyR7mEcMtzcPGqvIp8qNV27f3ZG7KJXNX4qymg4KriqNNSS+OSjeUYvSN5Wqcu1lq7tNqycuzBcEcRZhUlTwmV47FzjFy9nhqM69T2dvinGnTlKmpRi1zONrK121aPMeMvBGqfD/S77X/ABFDeva6fYXeriz00PqOp38enxGVbXTbSEpLeXDBlwJGFruzNdXEFvE8yc+acdZNluDdetN1G4SjQw9CnKpi8VU5qf7vB4Wnz1cRVc5xjalCKhz807QvKPfw/wCHPEOfZlTwGFhGhz1KUMTjMfXhhctwFKTm/bZljqvJSoUYxhOSc3z1HTdOlTnV5IT+cPA/x4+F/wAStWvbAanqXgvxUQceHfFlpAupyD7PaXMSW7aPearpsdzLJH5EFml+moT6nEILaGaWOB3vD8RuWCw+JxuVZxlNGpXjh4LMsE6NacvZSqOcXSqYhezXvU5VKlVQVSLpveLl6GaeGuZYTMcTQy7Ncg4gqYbDyxdavkeYe2w1KmsTGhBSeMpYCvz8zp1eWlS5XRl9YV4U6jpe43luls8UV/rFlYz+VuFvda1LbTrG8krK0lu8mYi5LMqjgoVYlnZmrWWKwvNJ2lJt3cnTU7tpfadWGqVotcqs010PEhlGdVIp0aU6sI3gnBVpRi4tpxvGMo6PVcrs4yT1u2f/0Pwpf9nXxnpN9bJoug6pa3stybG1ubOYWs7X0Et3LF9ivtGurXUBqUcES3s0elXckNzZTIj3c+8Qt9esKpxbjCU6TlyznVlzxnUd5RcFPmc5uUnzKKipwa1V7ny39u4ST5niYrkUpydOKp8sack5Oo3zOmudpvmV+eM3K8U4x+jNT+Ln/BQeSW3sNZ+On7Uuo6Vpc0FitlF8Sdb1zSYLrU3LPDpulTNLd2Et7H5qXHnyzwWOQxltJBHb1jWyPKKcP3mXYaSox56bdNKlD2tSK/dKpVUqM5O6UL6ckZxqaQjH28FxznKlXr4PiXMac8XTUcRbEck68MPGShTxMqGH/ewopKPLbVXco6+79V+F/wDgqN+3/wCGvGmh3vxa1vxH8XdKSzTRb3wt4w+HVzpNhpGm3SZ07xIPFPh7RI9O0LWF0vT7yz1LWNUW/sUiuGfVoRMsjxPB4XKsL708LQrwi1UnCr7OVWFWPNKPs60ZqdKmlzKNKMainOWvM4xmOtxHnGOnKVPExwr5OWFTAYeGFhWU1TVWFSlGMXWnGfs0qsuSdOD92EFJs3I/+Ck3jjxNqviHRh4Usfhlo+oWFvZavYfB/QPD3i2G+sdQMupTRapcatNo80OnRwSXDtq8N59sjkWW3jljhtoorX65cR4PnozxOGnU9glVp1a+JeI9mlTvzL2/s1TpcsVeVOcYae9dqXL8/wDVZq3s8Q6Ht1yypxw7w8qilNr97CEZRnNSnJQhKMppz0snzS+1ND/aLv30PR/7V13V/EMiafBFZ3tppviOxeHTItyWFleW1yIp0ureBRjfvH2V7XymWEJDF85mvifwXgMXLDt1sVJRUpVMBgISw/M5STUZVpUuaTtzuVPnpSU041JNyjHtwmXYqVG8sTCk+Zrlq4nEQnZWSuqNFw0XurZpRs0rJy//0fbPin/wUn/Zy8GwC11f4L+EbnTJ7S61O10JfBOmXH2rUrBrXWJUhSTRRZRamkmpiYX11dQwPqtw9yLkxl3i/QquHy7LMLGVbHV6UPenGlShWabo2m+XlvG7i/c5/wCZRckos/GKGEznN8ZL6pRpzlFwpOcqtGm4RxEpUYpuo3Jwj7ylyxblaUuRc0Yn4ieOf+CnPxv8R+NPE9z4U8ZWHwv8F2fizV9a8Far8Hvh14a+F/xO0rwuLySa08P3XivQR/aN5pt7BctDrGm32qXcd/JHb3d5Ld3cFvcQfnWY4nEY+rVlTrzdBt/VlKKjNU3O1P2nNKok73clGPWys25n6vlmS08sw8IQo2xc6dNYu9b2lCrVULyTXJTaV1DWK2g7JaKUdj/wVh/a+1Tw5q/hpPil8UPF+iaxZXem3Wn/ABF+IFzrFnrdtrNjHZ3Gja1Z2trBHqGivZNdWVxptw6Q36XUslxcRK7wV40stnKcatbH14uztToe5GNl9m/NeWj96b95QkrJyjKPpKg/aa1FCDk0+SLa9m22n35vhv0+zaVuaPJ+DP2nk8HxyIvgqDwmktnFNe3Pgv8As22ma7RGt54I7PyLOB7TZLttxPcytDG0Sb3S2SB+N5bGpBc8pzcXOPNUnJuMHK7i7ykmmlZxUZRet43d5c9TDzqVJyU3NvlbUpSiouKv7jUvdb5YyXLCNm33bOji/aP01kBnsr66nPMy30FhLPaSEBjZFxbyxsYAQJHhnnhkmaWWKQo4RfHeRVZP93ClGKck1GrKK5lKXNb3NdXvaOlklpcmU6dG0as480lzfw6j927jHWM0to6X1tvZu0f/2Q==",
        );

        println!("hasui-dark.jpeg");
        let hasui_dark = ImageAsset::new(
            "hasui-dark.jpeg",
            "A town at night",
            include_bytes!("./original_images/hasui_dark.jpeg"),
            // "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQACWAJYAAD/4QEGRXhpZgAATU0AKgAAAAgACAEGAAMAAAABAAIAAAESAAMAAAABAAEAAAEaAAUAAAABAAAAbgEbAAUAAAABAAAAdgEoAAMAAAABAAIAAAExAAIAAAAVAAAAfgEyAAIAAAAUAAAAlIdpAAQAAAABAAAAqAAAAAAAAAJYAAAAAQAAAlgAAAABUGl4ZWxtYXRvciBQcm8gMy4zLjkAADIwMjA6MTA6MzEgMjA6MzA6MzkAAAWQBAACAAAAFAAAAOqRAQAHAAAABAECAwCgAQADAAAAAQABAACgAgAEAAAAAQAAACSgAwAEAAAAAQAAABkAAAAAMjAyMDoxMDoxNyAxNjo0MTo1OQD/4Q3HaHR0cDovL25zLmFkb2JlLmNvbS94YXAvMS4wLwA8P3hwYWNrZXQgYmVnaW49Iu+7vyIgaWQ9Ilc1TTBNcENlaGlIenJlU3pOVGN6a2M5ZCI/PiA8eDp4bXBtZXRhIHhtbG5zOng9ImFkb2JlOm5zOm1ldGEvIiB4OnhtcHRrPSJYTVAgQ29yZSA2LjAuMCI+IDxyZGY6UkRGIHhtbG5zOnJkZj0iaHR0cDovL3d3dy53My5vcmcvMTk5OS8wMi8yMi1yZGYtc3ludGF4LW5zIyI+IDxyZGY6RGVzY3JpcHRpb24gcmRmOmFib3V0PSIiIHhtbG5zOnhtcD0iaHR0cDovL25zLmFkb2JlLmNvbS94YXAvMS4wLyIgeG1sbnM6eG1wTU09Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9tbS8iIHhtbG5zOnN0RXZ0PSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvc1R5cGUvUmVzb3VyY2VFdmVudCMiIHhtbG5zOnBob3Rvc2hvcD0iaHR0cDovL25zLmFkb2JlLmNvbS9waG90b3Nob3AvMS4wLyIgeG1wOkNyZWF0b3JUb29sPSJQaXhlbG1hdG9yIFBybyAzLjMuOSIgeG1wOk1vZGlmeURhdGU9IjIwMjAtMTAtMzFUMjA6MzA6MzkiIHhtcDpDcmVhdGVEYXRlPSIyMDIwLTEwLTE3VDE2OjQxOjU5IiB4bXA6TWV0YWRhdGFEYXRlPSIyMDIzLTA3LTIyVDIyOjI4OjA2LTA0OjAwIiB4bXBNTTpPcmlnaW5hbERvY3VtZW50SUQ9InhtcC5kaWQ6ZTM2NGJjZGYtZmU1NC00ZDQ4LTlkYTAtYjRjY2FkNmQ2MDJlIiB4bXBNTTpEb2N1bWVudElEPSJhZG9iZTpkb2NpZDpwaG90b3Nob3A6YWEwN2FlZjYtMWI2Yi0xMWViLThkM2YtYTM2YWQ4ZmM5ZjBhIiB4bXBNTTpJbnN0YW5jZUlEPSJ4bXAuaWlkOjA2NGIwNzU5LTdmZDAtYjM0MS1hNWJkLWQzOTNmMTg5NThjMiIgcGhvdG9zaG9wOklDQ1Byb2ZpbGU9IkVQU09OICBzUkdCIiBwaG90b3Nob3A6Q29sb3JNb2RlPSIzIj4gPHhtcE1NOkhpc3Rvcnk+IDxyZGY6U2VxPiA8cmRmOmxpIHN0RXZ0OnNvZnR3YXJlQWdlbnQ9IkFkb2JlIFBob3Rvc2hvcCBDQyAyMDE1IChXaW5kb3dzKSIgc3RFdnQ6d2hlbj0iMjAyMC0xMC0xN1QxNjo0MTo1OSswOTowMCIgc3RFdnQ6aW5zdGFuY2VJRD0ieG1wLmlpZDplMzY0YmNkZi1mZTU0LTRkNDgtOWRhMC1iNGNjYWQ2ZDYwMmUiIHN0RXZ0OmFjdGlvbj0iY3JlYXRlZCIvPiA8cmRmOmxpIHN0RXZ0OmFjdGlvbj0iY29udmVydGVkIiBzdEV2dDpwYXJhbWV0ZXJzPSJmcm9tIGltYWdlL3RpZmYgdG8gaW1hZ2UvanBlZyIvPiA8cmRmOmxpIHN0RXZ0OnNvZnR3YXJlQWdlbnQ9IkFkb2JlIFBob3Rvc2hvcCBDQyAyMDE1IChXaW5kb3dzKSIgc3RFdnQ6Y2hhbmdlZD0iLyIgc3RFdnQ6d2hlbj0iMjAyMC0xMC0zMVQyMDoyNDozOSswOTowMCIgc3RFdnQ6aW5zdGFuY2VJRD0ieG1wLmlpZDowNjRiMDc1OS03ZmQwLWIzNDEtYTViZC1kMzkzZjE4OTU4YzIiIHN0RXZ0OmFjdGlvbj0ic2F2ZWQiLz4gPC9yZGY6U2VxPiA8L3htcE1NOkhpc3Rvcnk+IDwvcmRmOkRlc2NyaXB0aW9uPiA8L3JkZjpSREY+IDwveDp4bXBtZXRhPiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIDw/eHBhY2tldCBlbmQ9InciPz4A/+0AYFBob3Rvc2hvcCAzLjAAOEJJTQQEAAAAAAAnHAFaAAMbJUccAgAAAgACHAI+AAgyMDIwMTAxNxwCPwAGMTY0MTU5ADhCSU0EJQAAAAAAEDt+6GrTppIXYs/L3zoTOvT/wAARCAAZACQDASIAAhEBAxEB/8QAHwAAAQUBAQEBAQEAAAAAAAAAAAECAwQFBgcICQoL/8QAtRAAAgEDAwIEAwUFBAQAAAF9AQIDAAQRBRIhMUEGE1FhByJxFDKBkaEII0KxwRVS0fAkM2JyggkKFhcYGRolJicoKSo0NTY3ODk6Q0RFRkdISUpTVFVWV1hZWmNkZWZnaGlqc3R1dnd4eXqDhIWGh4iJipKTlJWWl5iZmqKjpKWmp6ipqrKztLW2t7i5usLDxMXGx8jJytLT1NXW19jZ2uHi4+Tl5ufo6erx8vP09fb3+Pn6/8QAHwEAAwEBAQEBAQEBAQAAAAAAAAECAwQFBgcICQoL/8QAtREAAgECBAQDBAcFBAQAAQJ3AAECAxEEBSExBhJBUQdhcRMiMoEIFEKRobHBCSMzUvAVYnLRChYkNOEl8RcYGRomJygpKjU2Nzg5OkNERUZHSElKU1RVVldYWVpjZGVmZ2hpanN0dXZ3eHl6goOEhYaHiImKkpOUlZaXmJmaoqOkpaanqKmqsrO0tba3uLm6wsPExcbHyMnK0tPU1dbX2Nna4uPk5ebn6Onq8vP09fb3+Pn6/9sAQwADAwMDAwMFAwMFBgUFBQYJBgYGBgkLCQkJCQkLDQsLCwsLCw0NDQ0NDQ0NEBAQEBAQExMTExMVFRUVFRUVFRUV/9sAQwEDAwMFBQUJBQUJFg8MDxYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYW/90ABAAD/9oADAMBAAIRAxEAPwD5ytb64ll8i8RH5L+bkgY7AAYGOT0FdHZWeh6nCbZLjypWLZSfEahYwD8zsVTrnHOeMU9tLgODbhGhkJZVBOTgfMGYHt34qjLYwzhEt2WTdkIqEZDAE7QOuQB37+5rscbI8/2l3dG3q+n6bp8Bklvrd7hsFYbQq2R2zsA+Ybup4PPPFcpHPYyLNb3IuIyzAhy5YtyBgqMDODzzgdM1ej/sySMqjosnBzu/hY4GAR69eo6Zx0rIurxLe9SO3VZY5crySrGT3LDC8/496PdBOW1jcstP0OWHd56qdxBExZWB+g4x9Kuf2Xof/PzB/wB9P/jT4tNtrmCO5dxbiRdyKygkrkgN+OKf/Y1l/wA/Sf8AfIp+zXYXtpH/0Playm1J5mu3d280J+/TO7APIGOm7HP+FS6otxb3E0sQKCdy+HAcsS38O0AevI9O1V7P/U6d/wBdpP51an/4+o/x/mapyZmoK9ilci7WJTKJFI4w3UDnCkDPHcc0SXV8wU3e5yOm7nPPGO4wPWkj/wBXc/8AXb/2Zagu/wDVn/rq/wDJa0T0Ja1saUMs8sYaQM56dduAOMYz2qXMn/PNv++h/jV2x/49U/H+dW6fMzT6t5n/2Q==",
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
        println!("Images");
        let images_folder = built_dir.join("images");
        // Get list of all files in built_dir
        let paths_of_files_in_built_dir = fs::read_dir(images_folder)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect::<HashSet<PathBuf>>();

        vec![&self.hasui_hero.light_mode, &self.hasui_hero.dark_mode]
            .into_iter()
            .flat_map(|image_asset| &image_asset.resized_variants)
            .par_bridge()
            .for_each(|resized_image_asset| {
                resized_image_asset.save_to_disk(built_dir, &paths_of_files_in_built_dir);
            });
    }
}
