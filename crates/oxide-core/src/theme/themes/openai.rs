use crate::theme::{
    hex_to_vec4, ElevationScale, FocusRingTokens, MotionScale, OpacityScale, RadiusScale,
    SemanticColors, Shadow, ShadowScale, SpacingScale, Theme, ThemeMode, TypographyScale,
};
use makepad_widgets::makepad_draw::*;

fn h(hex: &str) -> Vec4 {
    hex_to_vec4(hex)
}

/// Returns the OpenAI-inspired light theme with teal accents and Söhne typography.
pub fn openai_theme() -> Theme {
    Theme {
        name: "OpenAI",
        mode: ThemeMode::Light,
        colors: SemanticColors {
            surface_primary: h("#FFFFFF"),
            surface_secondary: h("#F7F7F8"),
            surface_tertiary: h("#EAEBEE"),
            surface_inverse: h("#202023"),
            text_primary: h("#202023"),
            text_secondary: h("#676A70"),
            text_tertiary: h("#8E8EA0"),
            text_disabled: h("#ACACBE"),
            text_inverse: h("#FFFFFF"),
            text_link: h("#10A37F"),
            interactive_default: h("#10A37F"),
            interactive_hover: h("#0D8A6A"),
            interactive_pressed: h("#0B7359"),
            interactive_disabled: h("#C5C5D1"),
            border_default: h("#EAEBEE"),
            border_hover: h("#D1D3D8"),
            border_focus: h("#10A37F"),
            border_error: h("#E53935"),
            feedback_success: h("#10A37F"),
            feedback_warning: h("#F59E0B"),
            feedback_error: h("#E53935"),
            feedback_info: h("#3B82F6"),
        },
        spacing: SpacingScale::default(),
        radius: RadiusScale {
            none: 0.0,
            xs: 2.0,
            sm: 4.0,
            md: 8.0,
            lg: 12.0,
            xl: 16.0,
            full: 9999.0,
        },
        typography: TypographyScale {
            font_family: "Söhne",
            font_weight_bold: 600.0,
            ..Default::default()
        },
        shadows: ShadowScale {
            none: Shadow::none(),
            sm: Shadow::new(0.0, 1.0, 2.0, 0.0, vec4(0.0, 0.0, 0.0, 0.05)),
            md: Shadow::new(0.0, 2.0, 4.0, 0.0, vec4(0.0, 0.0, 0.0, 0.1)),
            lg: Shadow::new(0.0, 4.0, 8.0, 0.0, vec4(0.0, 0.0, 0.0, 0.12)),
            xl: Shadow::new(0.0, 8.0, 16.0, 0.0, vec4(0.0, 0.0, 0.0, 0.15)),
        },
        elevation: ElevationScale::default(),
        motion: MotionScale::default(),
        opacity: OpacityScale::default(),
        focus_ring: FocusRingTokens {
            color: h("#10A37F"),
            width: 2.0,
            offset: 2.0,
        },
    }
}

/// Returns the OpenAI-inspired dark theme.
pub fn openai_dark_theme() -> Theme {
    Theme {
        name: "OpenAI Dark",
        mode: ThemeMode::Dark,
        colors: SemanticColors {
            surface_primary: h("#0D0D0D"),
            surface_secondary: h("#1A1A1E"),
            surface_tertiary: h("#2A2A2E"),
            surface_inverse: h("#ECECF1"),
            text_primary: h("#ECECF1"),
            text_secondary: h("#A1A1AA"),
            text_tertiary: h("#71717A"),
            text_disabled: h("#52525B"),
            text_inverse: h("#0D0D0D"),
            text_link: h("#19C99D"),
            interactive_default: h("#19C99D"),
            interactive_hover: h("#15B589"),
            interactive_pressed: h("#10A37F"),
            interactive_disabled: h("#3F3F46"),
            border_default: h("#2A2A2E"),
            border_hover: h("#3F3F46"),
            border_focus: h("#19C99D"),
            border_error: h("#EF5350"),
            feedback_success: h("#19C99D"),
            feedback_warning: h("#FBBF24"),
            feedback_error: h("#EF5350"),
            feedback_info: h("#60A5FA"),
        },
        spacing: SpacingScale::default(),
        radius: RadiusScale {
            none: 0.0,
            xs: 2.0,
            sm: 4.0,
            md: 8.0,
            lg: 12.0,
            xl: 16.0,
            full: 9999.0,
        },
        typography: TypographyScale {
            font_family: "Söhne",
            font_weight_bold: 600.0,
            ..Default::default()
        },
        shadows: ShadowScale {
            none: Shadow::none(),
            sm: Shadow::new(0.0, 1.0, 2.0, 0.0, vec4(0.0, 0.0, 0.0, 0.20)),
            md: Shadow::new(0.0, 2.0, 4.0, 0.0, vec4(0.0, 0.0, 0.0, 0.30)),
            lg: Shadow::new(0.0, 4.0, 8.0, 0.0, vec4(0.0, 0.0, 0.0, 0.35)),
            xl: Shadow::new(0.0, 8.0, 16.0, 0.0, vec4(0.0, 0.0, 0.0, 0.40)),
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
            color: h("#19C99D"),
            width: 2.0,
            offset: 2.0,
        },
    }
}
