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
    let route_name = Route::from_request(&req);
    let response = route_name.html().into_response();
    Ok(response)
}
