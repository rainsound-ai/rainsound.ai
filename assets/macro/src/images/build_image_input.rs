use crate::parse_macro_arguments::*;
use std::path::PathBuf;
use syn::{
    parse::{Parse, ParseStream},
    Result as SynResult,
};

/// This struct represents the input to the `build_image!` macro.
pub struct BuildImageInput {
    pub absolute_path_to_image: PathBuf,
    pub alt: String,
    pub debug: bool,
}

impl Parse for BuildImageInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let error_message = r#"Please make sure to pass arguments to build_image! like this:

build_image!(path_to_image: \"src/images/santoka.jpg\");

The path should be relative to the workspace root.

You can also pass an optional `debug` argument like this:

build_image!(path_to_image: \"src/images/santoka.jpg\", debug: true);
"#;
        let error = syn::Error::new(input.span(), error_message);

        // This argument is required, so if it's not present we
        // convert None to an error and return early.
        let string_path_to_image_starting_at_workspace_root =
            parse_named_string_argument("path_to_image", &input).ok_or(error.clone())?;

        let absolute_path_to_image = assets_runtime::paths::workspace_root_dir()
            .join(string_path_to_image_starting_at_workspace_root);

        let alt = parse_named_string_argument("alt", &input).ok_or(error)?;

        // This argument is optional, so we default to `false` if it's not present.
        let debug = parse_named_bool_argument("debug", &input).unwrap_or(false);

        Ok(BuildImageInput {
            absolute_path_to_image,
            alt,
            debug,
        })
    }
}
