pub fn minify(source: &str) -> Vec<u8> {
    let session = minify_js::Session::new();
    let js_bytes = source.as_bytes();
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
