use axum::extract::Request;
use maud::Markup;

use super::*;

pub use shared::route::Route;

pub trait ServerSideRouteExtension {
    fn from_request(req: &Request) -> Self;
    fn html(&self) -> Markup;
}

impl ServerSideRouteExtension for Route {
    fn from_request(req: &Request) -> Route {
        let uri_path = req.uri().path();
        Route::parse_path(uri_path)
    }

    fn html(&self) -> Markup {
        match self {
            Route::ArtbreederUserStory => not_found_page(), // Should be a link to Notion for now.
            Route::BuildTime => build_time_page(),
            Route::Contact => not_found_page(), // Should be a mailto link for now.
            Route::Home => home_page(),
            Route::LevelAllUserStory => not_found_page(), // Should be a link to Notion for now.
            Route::NotFound => not_found_page(),
            Route::Paurtfaurliaur => portfolio_page(),
            Route::Portfolio => portfolio_page(),
            // Routes::SubmitContactForm => Route {
            //     verb: HttpVerb::Post,
            //     path: "/contact".to_string(),
            // },
        }
    }
}

// This code is WIP.
// use http::method::Method;
//
// trait Route<Input: RouteInput, Response> {
//     fn response(&self, input: Input) -> Response;

//     fn matches_request(&self, request: Request) -> bool {
//         self.uri() == request.uri() && self.method() == request.method()
//     }
//     fn method(&self) -> Method;
//     fn uri(&self) -> String;
// }

// trait RouteInput {
//     fn from_request_params(params: RequestParams) -> Self;
//     fn into_request_params(&self) -> RequestParams;
// }

// struct RequestParams {
//     url: String,
//     method_with_body: MethodWithBody,
// }

// pub enum MethodWithBody {
//     Get,
//     Post { body: String },
//     Put { body: String },
//     Delete,
// }

// struct HomePageRoute {}

// impl Route<(), Markup> for HomePageRoute {
//     fn method(&self) -> Method {
//         Method::GET
//     }

//     fn uri(&self) -> String {
//         "/".to_string()
//     }

//     fn get_response(&self, _: ()) -> Markup {
//         home_page()
//     }
// }
