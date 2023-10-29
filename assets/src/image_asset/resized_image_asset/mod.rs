use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "build")] {
    mod build_time_resized_image_asset;
    use build_time_resized_image_asset::BuildTimeResizedImageAsset;
    pub type ResizedImageAsset = BuildTimeResizedImageAsset;
} else {
    mod run_time_resized_image_asset;
    pub use run_time_resized_image_asset::RunTimeResizedImageAsset;
    pub type ResizedImageAsset = RunTimeResizedImageAsset;
}
}
