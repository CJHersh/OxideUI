mod airbnb;
mod notion;
mod openai;

pub use airbnb::{airbnb_dark_theme, airbnb_theme};
pub use notion::{notion_dark_theme, notion_theme};
pub use openai::{openai_dark_theme, openai_theme};

/// Returns all built-in themes (light variants first, then dark).
pub fn all_themes() -> Vec<super::Theme> {
    vec![
        openai::openai_theme(),
        airbnb::airbnb_theme(),
        notion::notion_theme(),
        openai::openai_dark_theme(),
        airbnb::airbnb_dark_theme(),
        notion::notion_dark_theme(),
    ]
}

/// Returns only the light built-in themes.
pub fn light_themes() -> Vec<super::Theme> {
    vec![
        openai::openai_theme(),
        airbnb::airbnb_theme(),
        notion::notion_theme(),
    ]
}

/// Returns only the dark built-in themes.
pub fn dark_themes() -> Vec<super::Theme> {
    vec![
        openai::openai_dark_theme(),
        airbnb::airbnb_dark_theme(),
        notion::notion_dark_theme(),
    ]
}
