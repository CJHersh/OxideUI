use crate::theme::{
    hex_to_vec4, ElevationScale, FocusRingTokens, MotionScale, OpacityScale, RadiusScale,
    SemanticColors, Shadow, ShadowScale, SpacingScale, Theme, ThemeMode, TypographyScale,
};
use makepad_widgets::makepad_draw::*;

fn h(hex: &str) -> Vec4 {
    hex_to_vec4(hex)
}

/// Returns the Airbnb-inspired light theme with coral accents and Cereal typography.
pub fn airbnb_theme() -> Theme {
    Theme {
        name: "Airbnb",
        mode: ThemeMode::Light,
        colors: SemanticColors {
            surface_primary: h("#FFFFFF"),
            surface_secondary: h("#F7F7F7"),
            surface_tertiary: h("#EFEFEF"),
            surface_inverse: h("#222222"),
            text_primary: h("#222222"),
            text_secondary: h("#717171"),
            text_tertiary: h("#999999"),
            text_disabled: h("#B0B0B0"),
            text_inverse: h("#FFFFFF"),
            text_link: h("#FF5A5F"),
            interactive_default: h("#FF5A5F"),
            interactive_hover: h("#E84E53"),
            interactive_pressed: h("#D9454A"),
            interactive_disabled: h("#CCCCCC"),
            border_default: h("#DDDDDD"),
            border_hover: h("#CCCCCC"),
            border_focus: h("#FF5A5F"),
            border_error: h("#C13515"),
            feedback_success: h("#008A05"),
            feedback_warning: h("#FFB400"),
            feedback_error: h("#C13515"),
            feedback_info: h("#00A699"),
        },
        spacing: SpacingScale::default(),
        radius: RadiusScale {
            none: 0.0,
            sm: 8.0,
            md: 12.0,
            lg: 16.0,
            xl: 24.0,
            full: 9999.0,
        },
        typography: TypographyScale {
            font_family: "Cereal",
            font_size_xl: 22.0,
            font_size_xxl: 26.0,
            font_weight_bold: 700.0,
            ..Default::default()
        },
        shadows: ShadowScale {
            none: Shadow::none(),
            sm: Shadow::new(0.0, 1.0, 3.0, 0.0, vec4(0.0, 0.0, 0.0, 0.06)),
            md: Shadow::new(0.0, 2.0, 6.0, 0.0, vec4(0.0, 0.0, 0.0, 0.1)),
            lg: Shadow::new(0.0, 4.0, 12.0, 0.0, vec4(0.0, 0.0, 0.0, 0.12)),
            xl: Shadow::new(0.0, 8.0, 24.0, 0.0, vec4(0.0, 0.0, 0.0, 0.15)),
        },
        elevation: ElevationScale::default(),
        motion: MotionScale::default(),
        opacity: OpacityScale::default(),
        focus_ring: FocusRingTokens {
            color: h("#FF5A5F"),
            width: 2.0,
            offset: 2.0,
        },
    }
}

/// Returns the Airbnb-inspired dark theme.
pub fn airbnb_dark_theme() -> Theme {
    Theme {
        name: "Airbnb Dark",
        mode: ThemeMode::Dark,
        colors: SemanticColors {
            surface_primary: h("#121212"),
            surface_secondary: h("#1E1E1E"),
            surface_tertiary: h("#2C2C2C"),
            surface_inverse: h("#F7F7F7"),
            text_primary: h("#F7F7F7"),
            text_secondary: h("#A3A3A3"),
            text_tertiary: h("#787878"),
            text_disabled: h("#525252"),
            text_inverse: h("#121212"),
            text_link: h("#FF7B7F"),
            interactive_default: h("#FF7B7F"),
            interactive_hover: h("#FF6B6F"),
            interactive_pressed: h("#FF5A5F"),
            interactive_disabled: h("#404040"),
            border_default: h("#2C2C2C"),
            border_hover: h("#404040"),
            border_focus: h("#FF7B7F"),
            border_error: h("#EF5350"),
            feedback_success: h("#34D058"),
            feedback_warning: h("#FFD54F"),
            feedback_error: h("#EF5350"),
            feedback_info: h("#4DD0E1"),
        },
        spacing: SpacingScale::default(),
        radius: RadiusScale {
            none: 0.0,
            sm: 8.0,
            md: 12.0,
            lg: 16.0,
            xl: 24.0,
            full: 9999.0,
        },
        typography: TypographyScale {
            font_family: "Cereal",
            font_size_xl: 22.0,
            font_size_xxl: 26.0,
            font_weight_bold: 700.0,
            ..Default::default()
        },
        shadows: ShadowScale {
            none: Shadow::none(),
            sm: Shadow::new(0.0, 1.0, 3.0, 0.0, vec4(0.0, 0.0, 0.0, 0.24)),
            md: Shadow::new(0.0, 2.0, 6.0, 0.0, vec4(0.0, 0.0, 0.0, 0.30)),
            lg: Shadow::new(0.0, 4.0, 12.0, 0.0, vec4(0.0, 0.0, 0.0, 0.36)),
            xl: Shadow::new(0.0, 8.0, 24.0, 0.0, vec4(0.0, 0.0, 0.0, 0.40)),
        },
        elevation: ElevationScale {
            level0: Shadow::none(),
            level1: Shadow::new(0.0, 1.0, 3.0, 0.0, vec4(0.0, 0.0, 0.0, 0.24)),
            level2: Shadow::new(0.0, 3.0, 8.0, -1.0, vec4(0.0, 0.0, 0.0, 0.30)),
            level3: Shadow::new(0.0, 8.0, 24.0, -4.0, vec4(0.0, 0.0, 0.0, 0.36)),
            level4: Shadow::new(0.0, 16.0, 48.0, -8.0, vec4(0.0, 0.0, 0.0, 0.44)),
        },
        motion: MotionScale::default(),
        opacity: OpacityScale {
            disabled: 0.38,
            hover_overlay: 0.10,
            pressed_overlay: 0.16,
            backdrop: 0.6,
        },
        focus_ring: FocusRingTokens {
            color: h("#FF7B7F"),
            width: 2.0,
            offset: 2.0,
        },
    }
}
