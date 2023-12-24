use assets::*;
use once_cell::sync::Lazy;

pub static assets: Lazy<Assets> = Lazy::new(Assets::new);

pub struct Assets {
    pub css: CssAsset,
    pub browser_crate: BrowserCrateAsset,
    pub favicon: FileAsset,
    pub logo: FileAsset,
    pub project_flower_one: ImageAsset,
    pub project_flower_two: ImageAsset,
    pub project_flower_three: ImageAsset,
    pub project_flower_four: ImageAsset,
    pub project_flower_five: ImageAsset,
    pub project_flower_six: ImageAsset,

    // Fonts
    pub fugi: FontAsset,
    pub aurora_grotesk_bold: FontAsset,
    pub aurora_grotesk_medium: FontAsset,
    pub aurora_grotesk_light: FontAsset,
    pub clearface_bold: FontAsset,
}

impl Assets {
    pub fn new() -> Self {
        let css = assets::include_tailwind!(
            path_to_input_file: "serverless_functions/src/assets/main.css",
            url_path: "built-assets/built.css",
            performance_budget_millis: 150,
        );

        let browser_crate = assets::include_browser_crate!(
            path_to_browser_crate: "browser",
            js_url_path: "built-assets/browser.js",
            js_performance_budget_millis: 150,
            wasm_url_path: "built-assets/browser_bg.wasm",
            wasm_performance_budget_millis: 150,
        );

        let favicon = assets::include_file!(
            path_to_input_file: "serverless_functions/src/assets/images/favicon.ico",
            url_path: "built-assets/favicon.ico",
            performance_budget_millis: 150,
        );

        let logo = assets::include_file!(
            path_to_input_file: "serverless_functions/src/assets/images/logo.png",
            url_path: "built-assets/favicon.ico",
            performance_budget_millis: 150,
        );

        // This image is decorative, so we skip the alt text.
        let project_flower_one = assets::include_image!(
            path_to_image: "serverless_functions/src/assets/images/project_flower_one.png",
            alt: "",
            placeholder: automatic_color,
        );

        // This image is decorative, so we skip the alt text.
        let project_flower_two = assets::include_image!(
            path_to_image: "serverless_functions/src/assets/images/project_flower_two.png",
            alt: "",
            placeholder: automatic_color,
        );

        // This image is decorative, so we skip the alt text.
        let project_flower_three = assets::include_image!(
            path_to_image: "serverless_functions/src/assets/images/project_flower_three.png",
            alt: "",
            placeholder: automatic_color,
        );

        // This image is decorative, so we skip the alt text.
        let project_flower_four = assets::include_image!(
            path_to_image: "serverless_functions/src/assets/images/project_flower_four.png",
            alt: "",
            placeholder: automatic_color,
        );

        // This image is decorative, so we skip the alt text.
        let project_flower_five = assets::include_image!(
            path_to_image: "serverless_functions/src/assets/images/project_flower_five.png",
            alt: "",
            placeholder: automatic_color,
        );

        // This image is decorative, so we skip the alt text.
        let project_flower_six = assets::include_image!(
            path_to_image: "serverless_functions/src/assets/images/project_flower_six.png",
            alt: "",
            placeholder: automatic_color,
        );

        // Fonts

        let fugi = assets::include_font!(
            path_to_input_file: "serverless_functions/src/assets/fonts/fugi.ttf",
            url_path: "built-assets/fonts/fugi.ttf",
            performance_budget_millis: 150,
        );

        let aurora_grotesk_bold = assets::include_font!(
            path_to_input_file: "serverless_functions/src/assets/fonts/aurora_grotesk_bold.otf",
            url_path: "built-assets/fonts/aurora-grotesk-bold.otf",
            performance_budget_millis: 150,
        );

        let aurora_grotesk_medium = assets::include_font!(
            path_to_input_file: "serverless_functions/src/assets/fonts/aurora_grotesk_medium.otf",
            url_path: "built-assets/fonts/aurora-grotesk-medium.otf",
            performance_budget_millis: 150,
        );

        let aurora_grotesk_light = assets::include_font!(
            path_to_input_file: "serverless_functions/src/assets/fonts/aurora_grotesk_light.otf",
            url_path: "built-assets/fonts/aurora-grotesk-light.otf",
            performance_budget_millis: 150,
        );

        let clearface_bold = assets::include_font!(
            path_to_input_file: "serverless_functions/src/assets/fonts/clearface_bold.ttf",
            url_path: "built-assets/fonts/clearface-bold.ttf",
            performance_budget_millis: 150,
        );

        Self {
            css,
            browser_crate,
            favicon,
            logo,
            project_flower_one,
            project_flower_two,
            project_flower_three,
            project_flower_four,
            project_flower_five,
            project_flower_six,

            fugi,
            aurora_grotesk_bold,
            aurora_grotesk_medium,
            aurora_grotesk_light,
            clearface_bold,
        }
    }
}

impl Default for Assets {
    fn default() -> Self {
        Self::new()
    }
}
