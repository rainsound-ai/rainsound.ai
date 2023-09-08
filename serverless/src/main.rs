use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    console_log!(
        "{} {}, located at: {:?}, within: {}",
        req.method().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );

    return Response::ok("Hello, world!").map(|mut r| {
        r.headers_mut().set("Content-Type", "text/plain");
        r
    });

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

    Response::error("Bad Request", 400)
}
