use serde::Deserialize;
use std::fmt::Display;
use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    Router::new()
        .post_async("/contact", post_contact)
        .run(req, env)
        .await
}

async fn post_contact(req: Request, _ctx: worker::RouteContext<()>) -> Result<Response> {
    let form_data = ContactFormSubmission::from_request(req).await?;
    console_debug!("{}", form_data);
    Response::ok("response from POST /contact")
}

#[derive(Debug, Deserialize)]
struct ContactFormSubmission {
    name: String,
    email: String,
    message: String,
}

impl ContactFormSubmission {
    async fn from_request(mut req: Request) -> Result<Self> {
        let form_data = req.form_data().await?;
        form_data.try_into()
    }
}

impl Display for ContactFormSubmission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ name: {}, email: {}, message: {} }}",
            self.name, self.email, self.message
        )
    }
}

impl TryFrom<worker::FormData> for ContactFormSubmission {
    type Error = Error;
    fn try_from(value: worker::FormData) -> std::result::Result<Self, Self::Error> {
        let name = value
            .get_field("name")
            .ok_or(Error::RustError("Missing Name.".to_string()))?;
        let email = value
            .get_field("email")
            .ok_or(Error::RustError("Missing Email.".to_string()))?;
        let message = value
            .get_field("message")
            .ok_or(Error::RustError("Missing Message.".to_string()))?;

        Ok(Self {
            name,
            email,
            message,
        })
    }
}

trait FormDataExtension {
    fn get_field(&self, field_name: &str) -> Option<String>;
}

impl FormDataExtension for worker::FormData {
    fn get_field(&self, field_name: &str) -> Option<String> {
        let field = match self.get(field_name) {
            Some(form_entry) => form_entry,
            None => return None,
        };

        match field {
            worker::FormEntry::Field(text) => Some(text),
            worker::FormEntry::File(_file) => None,
        }
    }
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
