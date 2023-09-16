use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    // console_log!(
    //     "{} {}, located at: {:?}, within: {}",
    //     req.method().to_string(),
    //     req.path(),
    //     req.cf().coordinates().unwrap_or_default(),
    //     req.cf().region().unwrap_or("unknown region".into())
    // );

    Router::new()
        .get("/", |_req, _ctx| {
            let html = include_str!("../../built/index.html");
            Response::from_html(html)
        })
        .get("/main.css", |_req, _ctx| {
            let css = include_str!("../../built/main.css");
            Response::ok(css).map(|mut r| {
                r.headers_mut().set("Content-Type", "text/css").unwrap();
                r
            })
        })
        .get("/browser.js", |_req, _ctx| {
            let js = include_str!("../../built/browser.js");
            Response::ok(js).map(|mut r| {
                r.headers_mut()
                    .set("Content-Type", "text/javascript")
                    .unwrap();
                r
            })
        })
        .get("/browser_bg.wasm", |_req, _ctx| {
            let wasm = include_bytes!("../../built/browser_bg.wasm").to_vec();
            Response::from_bytes(wasm).map(|mut r| {
                r.headers_mut()
                    .set("Content-Type", "application/wasm")
                    .unwrap();
                r
            })
        })
        .run(req, env)
        .await

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
}
