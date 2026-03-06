//! Theme engine and token system for OxideUI.
//!
//! Provides semantic color tokens, spacing scales, typography, shadows, and runtime theme switching.

pub mod theme;

pub use makepad_widgets;

pub mod prelude {
    pub use crate::theme::engine::{ThemeChangedAction, ThemeEngine};
    pub use crate::theme::themes::all_themes;
    pub use crate::theme::{
        hex_to_vec4, ElevationScale, FocusRingTokens, MotionScale, OpacityScale, RadiusScale,
        SemanticColors, Shadow, ShadowScale, SpacingScale, Theme, ThemeMode, TypographyScale,
    };
    pub use makepad_widgets::*;
}

pub fn script_mod(_vm: &mut makepad_widgets::ScriptVm) {}
