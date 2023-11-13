use std::path::{Path, PathBuf};

#[cfg(feature = "build")]
pub fn workspace_root_dir() -> PathBuf {
    let cargo_workspace_dir = std::env!("CARGO_WORKSPACE_DIR");
    Path::new(&cargo_workspace_dir).to_path_buf()
}

// At runtime, built assets get mounted to /built_assets.
#[cfg(not(feature = "build"))]
pub fn workspace_root_dir() -> PathBuf {
    // If you change this, you also need to change the files mount path in spin.toml.
    Path::new("/").to_path_buf()
}

pub fn built_assets_dir() -> PathBuf {
    workspace_root_dir().join(built_assets_dir_name())
}

pub fn built_assets_dir_name() -> &'static str {
    "built_assets"
}

/// When loading assets in the browser, URL paths should
/// start with this prefix.
///
/// For example, if you have an asset at `built_assets/built.css`,
/// then the URL path to that asset in the browser should be
/// `/built-assets/built.css`.
pub fn built_assets_browser_prefix() -> PathBuf {
    PathBuf::from("built-assets")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_built_assets_dir() {
        let invoke_serverless_functions_component =
            find_spin_toml_component_by_id("invoke_serverless_functions");

        let mut file_tables = invoke_serverless_functions_component
            .get("files")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|file_value| file_value.as_table().unwrap());

        let built_assets_table = file_tables
            .find(|file_table| {
                file_table.get("source").unwrap().as_str().unwrap()
                    == super::built_assets_dir_name()
            })
            .unwrap();

        let actual_destination = built_assets_table
            .get("destination")
            .unwrap()
            .as_str()
            .unwrap();
        let expected_destination = super::built_assets_dir().to_string_lossy().to_string();

        assert_eq!(actual_destination, expected_destination);
    }

    #[test]
    fn test_built_assets_browser_prefix() {
        let serve_built_assets_component = find_spin_toml_component_by_id("serve_built_assets");

        let actual_route = serve_built_assets_component
            .get("trigger")
            .unwrap()
            .as_table()
            .unwrap()
            .get("route")
            .unwrap()
            .as_str()
            .unwrap();

        let prefix = super::built_assets_browser_prefix()
            .to_string_lossy()
            .to_string();
        let expected_route = format!("/{prefix}/...");

        assert_eq!(actual_route, expected_route);
    }

    fn find_spin_toml_component_by_id(id: &str) -> toml::Table {
        let component = get_spin_toml_components()
            .into_iter()
            .find(|component| component.get("id").unwrap().as_str().unwrap().eq(id))
            .unwrap();
        component.clone()
    }

    fn get_spin_toml_components() -> Vec<toml::Table> {
        let spin_toml = parse_spin_toml();
        spin_toml
            .get("component")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|component_value| component_value.as_table().unwrap().clone())
            .collect()
    }

    fn parse_spin_toml() -> toml::Table {
        let spin_toml_unparsed = include_str!("../../spin.toml");
        spin_toml_unparsed.parse::<toml::Table>().unwrap()
    }
}
