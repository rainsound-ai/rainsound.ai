use crate::extensions::*;
use anyhow::Result;
use routes::*;

mod components;
mod css_class_groups;
mod extensions;
// mod notion;
mod assets;
mod routes;
mod side;

// #[http_component]
// fn main(req: Request) -> Result<Response> {
//     let route = Route::from_request(&req);
//     let response = route.html().into_response();
//     Ok(response)
// }

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
