#![allow(non_upper_case_globals)]

pub static built_wasm: &[u8] = include_bytes!("../target/browser_bg.wasm");
pub static built_js: &str = include_str!("../target/browser.js");
