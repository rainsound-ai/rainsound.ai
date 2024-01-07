use shared::route::Route;

pub struct Project {
    pub text: &'static str,
    pub link: Route,
}

pub fn all_projects() -> Vec<Project> {
    vec![
        Project {
            text: "Taking the pain out of Level All's essential operations.",
            link: Route::LevelAllUserStory,
        },
        Project {
            text: "Reclaiming time for Artbreeder’s founder and CEO to focus on what matters.",
            link: Route::ArtbreederUserStory,
        },
    ]
}
