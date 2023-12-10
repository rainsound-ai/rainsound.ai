#![allow(non_upper_case_globals)]

use anyhow::Result;
use routes::*;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

mod components;

mod css_class_groups;

mod extensions;
// mod notion;

mod prelude;
use self::prelude::*;

mod routes;

mod serverless_functions_assets;

#[http_component]
fn main(req: Request) -> Result<Response> {
    let route = Route::from_request(&req);
    let response = route.html().into_response();
    Ok(response)
}
