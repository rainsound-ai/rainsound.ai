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
    // This saves our assets to the assets/built_assets directory at build time.
    assets::save_to_disk!(debug: true);

    let route = Route::from_request(&req);
    let response = route.html().into_response();
    Ok(response)
}
