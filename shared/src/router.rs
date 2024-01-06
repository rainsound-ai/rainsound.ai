use enum_iterator;
use std::fmt::Display;

#[derive(Clone, Copy, enum_iterator::Sequence)]
pub enum Route {
    ArtbreederUserStory,
    BuildTime,
    Home,
    Contact,
    LevelAllUserStory,
    NotFound,
    Paurtfaurliaur,
    Portfolio,
}

impl Route {
    pub fn all() -> impl Iterator<Item = Route> {
        enum_iterator::all::<Route>()
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
            Route::ArtbreederUserStory => {
                "https://rainsound-ai.notion.site/Reclaiming-time-for-Artbreeder-s-CEO-to-focus-on-what-matters-403c49b167c54e518ef1a6fee8ce4c86"
            }
            Route::BuildTime => "/build-time",
            Route::Contact => "mailto:hello@rainsound.ai",
            Route::Home => "/",
            Route::LevelAllUserStory => {
                "https://rainsound-ai.notion.site/Taking-the-pain-out-of-Level-All-s-essential-operations-7656fd7b3f364b6bb1a1499464a1875b"
            }
            Route::NotFound => "/not-found",
            Route::Paurtfaurliaur => "/paurtfaurliaur",
            Route::Portfolio => "/portfolio",
        };

        write!(f, "{}", route_str)
    }
}
