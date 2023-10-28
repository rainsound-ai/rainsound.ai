use anyhow::Result;
use routes::Route;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

mod extensions;
// mod notion;
mod prelude;
use prelude::*;
mod routes;

#[http_component]
fn main(req: Request) -> Result<Response> {
    let route = Route::from_request(&req);
    let response = route.html().into_response();
    Ok(response)
}
