use build_time::build_time_utc;
use maud::{Markup, PreEscaped};

pub fn build_time_page() -> Markup {
    PreEscaped(build_time_utc!().to_string())
}
