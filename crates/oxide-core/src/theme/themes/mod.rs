mod airbnb;
mod notion;
mod openai;
mod shadcn;

pub use airbnb::{airbnb_dark_theme, airbnb_theme};
pub use notion::{notion_dark_theme, notion_theme};
pub use openai::{openai_dark_theme, openai_theme};
pub use shadcn::{shadcn_dark_theme, shadcn_theme};

/// Returns the active built-in themes (shadcn light + dark).
///
/// Additional brand themes (OpenAI, Airbnb, Notion) are available via their
/// individual constructor functions but are not included here by default.
/// To re-enable them, add them to this list and call `ThemeEngine::init`.
pub fn all_themes() -> Vec<super::Theme> {
    vec![shadcn::shadcn_theme(), shadcn::shadcn_dark_theme()]
}

/// Returns only the light built-in themes.
pub fn light_themes() -> Vec<super::Theme> {
    vec![shadcn::shadcn_theme()]
}

/// Returns only the dark built-in themes.
pub fn dark_themes() -> Vec<super::Theme> {
    vec![shadcn::shadcn_dark_theme()]
}
