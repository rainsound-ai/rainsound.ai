#![allow(non_upper_case_globals)]
use new_assets::non_html_assets_by_path;
use routes::contact::form_submission::ContactFormSubmission;
use worker::*;

mod notion;
mod routes;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    console_debug!("Reading browser.js");
    std::fs::read_to_string("built/browser.js").unwrap();
    console_debug!("Done reading browser.js");

    if let Some(response) = serve_assets(&req) {
        return response;
    }

    Router::new()
        .get_async("/", get_home_page)
        // .get_async("/built.css", get_built_css)
        .get_async("/browser.js", get_browser_js)
        .get_async("/browser_bg.wasm", get_browser_bg_wasm)
        .post_async("/contact", post_contact)
        // .get_async("/*asset", serve_assets)
        .run(req, env)
        .await
}

fn serve_assets(req: &Request) -> Option<Result<Response>> {
    let path = req.path();
    console_debug!("Path from request {}", path);

    non_html_assets_by_path
        .get(&path)
        .map(|(content_type, bytes)| {
            console_debug!("Generating response for asset {}", path);
            Response::from_bytes(bytes.clone()).map(|mut r| {
                r.headers_mut().set("Content-Type", content_type).unwrap();
                r
            })
        })
}

async fn get_home_page(_req: Request, _ctx: worker::RouteContext<()>) -> Result<Response> {
    console_debug!("Rendering html dynamically.");
    let html = crate::routes::home_page().into_string();
    Response::from_html(html)
}

async fn get_browser_js(_req: Request, _ctx: worker::RouteContext<()>) -> Result<Response> {
    let bytes = include_str!("../../built/browser.js");
    Response::ok(bytes).map(|mut r| {
        r.headers_mut()
            .set("Content-Type", "text/javascript")
            .unwrap();
        r
    })
}

async fn get_browser_bg_wasm(_req: Request, _ctx: worker::RouteContext<()>) -> Result<Response> {
    let bytes = include_bytes!("../../built/browser_bg.wasm").to_vec();
    Response::from_bytes(bytes).map(|mut r| {
        r.headers_mut()
            .set("Content-Type", "application/wasm")
            .unwrap();
        r
    })
}

async fn post_contact(req: Request, _ctx: worker::RouteContext<()>) -> Result<Response> {
    console_debug!("POST /contact");
    let form_data = ContactFormSubmission::from_request(req).await?;
    console_debug!("{}", form_data);

    notion::add_contact_form_submission_to_database(form_data).await;

    Response::ok("response from POST /contact")
}

// if !matches!(req.method(), Method::Post) {
//     return Response::error("Method Not Allowed", 405);
// }

// if let Some(file) = req.form_data().await?.get("file") {
//     return match file {
//         FormEntry::File(buf) => {
//             Response::ok(&format!("size = {}", buf.bytes().await?.len()))
//         }
//         _ => Response::error("`file` part of POST form must be a file", 400),
//     };
// }
// .get("/", |_req, _ctx| {
//     let html = include_str!("../../built/index.html");
//     Response::from_html(html)
// })
// .get("/main.css", |_req, _ctx| {
//     let css = include_str!("../../built/main.css");
//     Response::ok(css).map(|mut r| {
//         r.headers_mut().set("Content-Type", "text/css").unwrap();
//         r
//     })
// })
// .get("/browser.js", |_req, _ctx| {
//     let js = include_str!("../../built/browser.js");
//     Response::ok(js).map(|mut r| {
//         r.headers_mut()
//             .set("Content-Type", "text/javascript")
//             .unwrap();
//         r
//     })
// })
// .get("/browser_bg.wasm", |_req, _ctx| {
//     let wasm = include_bytes!("../../built/browser_bg.wasm").to_vec();
//     Response::from_bytes(wasm).map(|mut r| {
//         r.headers_mut()
//             .set("Content-Type", "application/wasm")
//             .unwrap();
//         r
//     })
// })
