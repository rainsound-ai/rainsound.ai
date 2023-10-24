#![allow(non_upper_case_globals)]
pub use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn handle_request() -> String {
    return "Hello from Rust!".to_string();

    // Router::new()
    //     .get_async("/", get_home_page)
    //     .run(req, env)
    //     .await
}
