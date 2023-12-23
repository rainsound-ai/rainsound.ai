use enum_iterator::{all, Sequence};
use std::fmt::Display;

#[derive(Clone, Sequence)]
pub enum Route {
    Home,
    Contact,
    Portfolio,
    Paurtfaurliaur,
    NotFound,
    BuildTime,
}

impl Route {
    pub fn all() -> impl Iterator<Item = Route> {
        all::<Route>()
    }

    pub fn parse_path(path: &str) -> Route {
        Route::all()
            .find(|route| route.to_string() == path)
            .unwrap_or(Route::NotFound)
    }
}

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let route_str = match self {
            Route::Home => "/",
            Route::Contact => "/contact",
            Route::Portfolio => "/portfolio",
            Route::Paurtfaurliaur => "/paurtfaurliaur",
            Route::NotFound => "/not-found",
            Route::BuildTime => "/build-time",
        };

        write!(f, "{}", route_str)
    }
}
