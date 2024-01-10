use crate::extensions::*;
use anyhow::Result;
use routes::*;
use tower_http::services::ServeDir;

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

use axum::{extract::Request, routing::get, Router};

#[tokio::main]
async fn main() {
    let built_assets_browser_prefix = {
        let browser_prefix = ::assets::paths::built_assets_browser_prefix();
        format!("/{}", browser_prefix.to_string_lossy())
    };
    let built_assets_dir = ::assets::paths::built_assets_dir();

    let app = Router::new()
        .route("/", get(handle_request)) // The wildcard "/*anthing" syntax doesn't match the root route, so we have to register that one separately.
        .route("/*anything", get(handle_request))
        .route("/healthz", get(health_check))
        .nest_service(
            &built_assets_browser_prefix,
            ServeDir::new(built_assets_dir),
        );

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let host_and_port = format!("0.0.0.0:{}", port);
    // Run our app with hyper, listening globally on the specified port.
    let listener = tokio::net::TcpListener::bind(host_and_port).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// For now, all of our routes return HTML.
async fn handle_request(req: Request) -> axum::response::Html<String> {
    let route = Route::from_request(&req);
    route.html().into_axum_html_response()
}

async fn health_check() {}
