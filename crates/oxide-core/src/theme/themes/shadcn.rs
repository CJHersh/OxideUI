use crate::theme::{
    hex_to_vec4, ElevationScale, FocusRingTokens, MotionScale, OpacityScale, RadiusScale,
    SemanticColors, Shadow, ShadowScale, SpacingScale, Theme, ThemeMode, TypographyScale,
};
use makepad_widgets::makepad_draw::*;

fn h(hex: &str) -> Vec4 {
    hex_to_vec4(hex)
}

/// Returns the shadcn/ui-inspired light theme with neutral palette and Inter typography.
pub fn shadcn_theme() -> Theme {
    Theme {
        name: "shadcn",
        mode: ThemeMode::Light,
        colors: SemanticColors {
            surface_primary: h("#FFFFFF"),
            surface_secondary: h("#F5F5F5"),
            surface_tertiary: h("#F5F5F5"),
            surface_inverse: h("#171717"),
            text_primary: h("#0A0A0A"),
            text_secondary: h("#737373"),
            text_tertiary: h("#404040"),
            text_disabled: h("#A3A3A3"),
            text_inverse: h("#FAFAFA"),
            text_link: h("#171717"),
            interactive_default: h("#171717"),
            interactive_hover: h("#404040"),
            interactive_pressed: h("#525252"),
            interactive_disabled: h("#D4D4D4"),
            border_default: h("#E5E5E5"),
            border_hover: h("#D4D4D4"),
            border_focus: h("#D4D4D4"),
            border_error: h("#EF4444"),
            feedback_success: h("#16A34A"),
            feedback_warning: h("#F59E0B"),
            feedback_error: h("#DC2626"),
            feedback_info: h("#3B82F6"),
        },
        spacing: SpacingScale::default(),
        radius: RadiusScale::default(),
        typography: TypographyScale {
            font_family: "Inter",
            ..Default::default()
        },
        shadows: ShadowScale {
            none: Shadow::none(),
            sm: Shadow::new(0.0, 1.0, 2.0, 0.0, vec4(0.0, 0.0, 0.0, 0.05)),
            md: Shadow::new(0.0, 2.0, 4.0, -1.0, vec4(0.0, 0.0, 0.0, 0.1)),
            lg: Shadow::new(0.0, 4.0, 6.0, -2.0, vec4(0.0, 0.0, 0.0, 0.1)),
            xl: Shadow::new(0.0, 10.0, 15.0, -3.0, vec4(0.0, 0.0, 0.0, 0.1)),
        },
        elevation: ElevationScale::default(),
        motion: MotionScale::default(),
        opacity: OpacityScale {
            disabled: 0.50,
            hover_overlay: 0.05,
            pressed_overlay: 0.10,
            backdrop: 0.6,
        },
        focus_ring: FocusRingTokens {
            color: h("#D4D4D4"),
            width: 2.0,
            offset: 2.0,
        },
    }
}

/// Returns the shadcn/ui-inspired dark theme.
pub fn shadcn_dark_theme() -> Theme {
    Theme {
        name: "shadcn Dark",
        mode: ThemeMode::Dark,
        colors: SemanticColors {
            surface_primary: h("#000000"),
            surface_secondary: h("#262626"),
            surface_tertiary: h("#171717"),
            surface_inverse: h("#F5F5F5"),
            text_primary: h("#FAFAFA"),
            text_secondary: h("#A3A3A3"),
            text_tertiary: h("#D4D4D4"),
            text_disabled: h("#525252"),
            text_inverse: h("#0A0A0A"),
            text_link: h("#F5F5F5"),
            interactive_default: h("#F5F5F5"),
            interactive_hover: h("#D4D4D4"),
            interactive_pressed: h("#A3A3A3"),
            interactive_disabled: h("#404040"),
            border_default: h("#404040"),
            border_hover: h("#404040"),
            border_focus: h("#404040"),
            border_error: h("#EF4444"),
            feedback_success: h("#4ADE80"),
            feedback_warning: h("#FBBF24"),
            feedback_error: h("#9E4042"),
            feedback_info: h("#60A5FA"),
        },
        spacing: SpacingScale::default(),
        radius: RadiusScale::default(),
        typography: TypographyScale {
            font_family: "Inter",
            ..Default::default()
        },
        shadows: ShadowScale {
            none: Shadow::none(),
            sm: Shadow::new(0.0, 1.0, 2.0, 0.0, vec4(0.0, 0.0, 0.0, 0.20)),
            md: Shadow::new(0.0, 2.0, 4.0, -1.0, vec4(0.0, 0.0, 0.0, 0.30)),
            lg: Shadow::new(0.0, 4.0, 6.0, -2.0, vec4(0.0, 0.0, 0.0, 0.30)),
            xl: Shadow::new(0.0, 10.0, 15.0, -3.0, vec4(0.0, 0.0, 0.0, 0.30)),
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
            disabled: 0.50,
            hover_overlay: 0.10,
            pressed_overlay: 0.15,
            backdrop: 0.6,
        },
        focus_ring: FocusRingTokens {
            color: h("#404040"),
            width: 2.0,
            offset: 2.0,
        },
    }
}
