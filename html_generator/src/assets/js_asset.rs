use crate::prelude::*;

#[derive(PartialEq)]
pub struct JsAsset {
    pub asset_path: &'static str,
    pub contents: &'static str,
    pub size_budget: NumBytes,
}

impl JsAsset {
    pub fn minified_contents(&self) -> Vec<u8> {
        let session = minify_js::Session::new();
        let js_bytes = self.contents.as_bytes();
        let mut out = Vec::new();
        minify_js::minify(
            &session,
            minify_js::TopLevelMode::Module,
            js_bytes,
            &mut out,
        )
        .unwrap();
        out
    }
}

impl HasSizeBudget for JsAsset {
    fn size_budget(&self) -> NumBytes {
        self.size_budget
    }

    fn check_size_budget(&self) -> HowCloseToBudget {
        NonImageAsset::check_size_budget(self)
    }
}

impl NonImageAsset for JsAsset {
    fn asset_path(&self) -> &str {
        self.asset_path
    }

    fn bytes(&self) -> Vec<u8> {
        // Unminified. You can comment this back in if you want to debug.
        // self.contents.as_bytes().to_vec()

        // Minified.
        self.minified_contents()
    }
}
