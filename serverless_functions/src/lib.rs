use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

use new_assets::non_html_assets_by_path;
// use routes::contact::form_submission::ContactFormSubmission;

// mod notion;
// mod routes;

#[http_component]
fn main(_req: Request) -> Result<Response> {
    let body = new_assets::non_html_assets.hasui_hero.bytes.into();

    let response = http::Response::builder()
        .status(200)
        .header("Content-Type", "image/jpeg")
        // .header("Content-Type", "text/html")
        .body(Some(body))?;

    Ok(response)
}
