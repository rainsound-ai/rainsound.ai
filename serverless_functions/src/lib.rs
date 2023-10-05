#![allow(non_upper_case_globals)]
use forms::ContactFormSubmission;
use worker::*;

mod forms;
mod notion;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    Router::new()
        .get_async("/pre-rendered", get_pre_rendered_home_page)
        .get_async("/dynamically-rendered", get_dynamically_rendered_home_page)
        .get_async("/main.css", get_main_css)
        .get_async("/browser.js", get_browser_js)
        .get_async("/browser_bg.wasm", get_browser_bg_wasm)
        .post_async("/contact", post_contact)
        .run(req, env)
        .await
}

async fn get_pre_rendered_home_page(
    _req: Request,
    _ctx: worker::RouteContext<()>,
) -> Result<Response> {
    Response::from_html(include_str!("../../built/index.html"))
}

async fn get_dynamically_rendered_home_page(
    _req: Request,
    _ctx: worker::RouteContext<()>,
) -> Result<Response> {
    let html = html_generator_hack::assets::HtmlAsset::home_page();
    Response::from_html(html)
}

async fn get_main_css(_req: Request, _ctx: worker::RouteContext<()>) -> Result<Response> {
    let bytes = include_str!("../../built/main.css");
    Response::ok(bytes).map(|mut r| {
        r.headers_mut().set("Content-Type", "text/css").unwrap();
        r
    })
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
