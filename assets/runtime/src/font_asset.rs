use crate::asset_url_path;
use cfg_if::cfg_if;
use std::{path::PathBuf, time::Duration};

#[derive(PartialEq)]
pub struct FontAsset {
    pub full_url_path: PathBuf, // Used for loading the asset in the browser.
    pub url_path_starting_from_built_assets_dir: PathBuf, // Used for saving the asset to disk.
    pub load_time_budget: Duration,
    pub size_in_bytes: usize, // For checking performance budgets.
}

impl FontAsset {
    pub fn new(
        url_path_starting_from_built_assets_dir: PathBuf,
        load_time_budget: Duration,
        size_in_bytes: usize,
    ) -> Self {
        let full_url_path = asset_url_path(&url_path_starting_from_built_assets_dir);

        let asset = Self {
            full_url_path,
            url_path_starting_from_built_assets_dir,
            load_time_budget,
            size_in_bytes,
        };

        #[cfg(feature = "build_time")]
        asset.check_performance_budget();

        asset
    }
}

cfg_if! {
if #[cfg(feature = "build_time")] {
    use crate::performance_budget::HasPerformanceBudget;
    use proc_macro2::TokenStream;
    use quote::{quote, ToTokens};

    impl HasPerformanceBudget for FontAsset {
        fn load_time_budget(&self) -> Duration {
            self.load_time_budget
        }

        fn size_in_bytes(&self) -> usize{
            self.size_in_bytes
        }

        fn path_for_reporting_asset_over_budget(&self) -> &std::path::Path {
            &self.url_path_starting_from_built_assets_dir
        }
    }

    impl ToTokens for FontAsset {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            log::info!("Converting FontAsset to tokens.");

            let full_url_path = self.full_url_path.to_str().unwrap();
            // log::info!("full_url_path: {}", full_url_path);

            let url_path_starting_from_built_assets_dir = self
                .url_path_starting_from_built_assets_dir
                .to_str()
                .unwrap();
            // log::info!(
            //     "url_path_starting_from_built_assets_dir: {}",
            //     url_path_starting_from_built_assets_dir
            // );


            let load_time_budget_millis = self.load_time_budget.as_millis() as u64;
            // log::info!("load_time_budget_millis: {}", load_time_budget_millis);

            let size_in_bytes = self.size_in_bytes;
            // log::info!("size_in_bytes: {}", size_in_bytes);

            let quoted = quote! {
                FontAsset {
                    full_url_path: std::path::PathBuf::from(#full_url_path),
                    url_path_starting_from_built_assets_dir: std::path::PathBuf::from(#url_path_starting_from_built_assets_dir),
                    load_time_budget: std::time::Duration::from_millis(#load_time_budget_millis),
                    size_in_bytes: #size_in_bytes,
                }
            };
            // log::info!("quoted: {}", quoted);

            tokens.extend(quoted);
            // log::info!("Extended tokens.");
        }
    }

}
}
