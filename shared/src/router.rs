use enum_iterator::{all, Sequence};
use std::fmt::Display;

#[derive(Clone, Sequence)]
pub enum Route {
    Home,
    Contact,
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
        let string = match self {
            Route::Home => "/".to_string(),
            Route::Contact => "/contact".to_string(),
            Route::NotFound => "/not-found".to_string(),
            Route::BuildTime => "/build-time".to_string(),
        };

        write!(f, "{}", string)
    }
}
