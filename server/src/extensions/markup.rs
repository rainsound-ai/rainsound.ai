use maud::Markup;

pub trait MarkupExtension {
    fn into_response(self) -> Response;
}

impl MarkupExtension for Markup {
    fn into_response(self) -> Response {
        let body = self.into_string().into();
        http::Response::builder()
            .status(200)
            .header("Content-Type", "text/html")
            .body(Some(body))
            .unwrap()
    }
}
