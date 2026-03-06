use crate::theme::{
    hex_to_vec4, ElevationScale, FocusRingTokens, MotionScale, OpacityScale, RadiusScale,
    SemanticColors, Shadow, ShadowScale, SpacingScale, Theme, ThemeMode, TypographyScale,
};
use makepad_widgets::makepad_draw::*;

fn h(hex: &str) -> Vec4 {
    hex_to_vec4(hex)
}

/// Returns the Notion-inspired light theme with blue accents and Inter typography.
pub fn notion_theme() -> Theme {
    Theme {
        name: "Notion",
        mode: ThemeMode::Light,
        colors: SemanticColors {
            surface_primary: h("#FFFFFF"),
            surface_secondary: h("#F9F9F8"),
            surface_tertiary: h("#EFEFED"),
            surface_inverse: h("#252525"),
            text_primary: h("#252525"),
            text_secondary: h("#737373"),
            text_tertiary: h("#9B9A97"),
            text_disabled: h("#B3B2AF"),
            text_inverse: h("#FFFFFF"),
            text_link: h("#346CA3"),
            interactive_default: h("#346CA3"),
            interactive_hover: h("#2B5A8A"),
            interactive_pressed: h("#244A73"),
            interactive_disabled: h("#D3D3D1"),
            border_default: h("#E3E2E0"),
            border_hover: h("#D3D3D1"),
            border_focus: h("#346CA3"),
            border_error: h("#EB5757"),
            feedback_success: h("#0F9D58"),
            feedback_warning: h("#F2994A"),
            feedback_error: h("#EB5757"),
            feedback_info: h("#346CA3"),
        },
        spacing: SpacingScale::default(),
        radius: RadiusScale {
            none: 0.0,
            sm: 3.0,
            md: 4.0,
            lg: 6.0,
            xl: 8.0,
            full: 9999.0,
        },
        typography: TypographyScale {
            font_family: "Inter",
            font_size_xs: 11.0,
            font_size_sm: 13.0,
            font_size_md: 14.0,
            font_size_lg: 16.0,
            font_size_xl: 18.0,
            font_size_xxl: 20.0,
            font_weight_bold: 600.0,
            ..Default::default()
        },
        shadows: ShadowScale {
            none: Shadow::none(),
            sm: Shadow::new(0.0, 1.0, 2.0, 0.0, vec4(0.0, 0.0, 0.0, 0.04)),
            md: Shadow::new(0.0, 2.0, 4.0, 0.0, vec4(0.0, 0.0, 0.0, 0.08)),
            lg: Shadow::new(0.0, 4.0, 8.0, 0.0, vec4(0.0, 0.0, 0.0, 0.1)),
            xl: Shadow::new(0.0, 8.0, 16.0, 0.0, vec4(0.0, 0.0, 0.0, 0.12)),
        },
        elevation: ElevationScale {
            level0: Shadow::none(),
            level1: Shadow::new(0.0, 1.0, 2.0, 0.0, vec4(0.0, 0.0, 0.0, 0.06)),
            level2: Shadow::new(0.0, 2.0, 6.0, -1.0, vec4(0.0, 0.0, 0.0, 0.08)),
            level3: Shadow::new(0.0, 6.0, 16.0, -3.0, vec4(0.0, 0.0, 0.0, 0.10)),
            level4: Shadow::new(0.0, 12.0, 32.0, -6.0, vec4(0.0, 0.0, 0.0, 0.14)),
        },
        motion: MotionScale::default(),
        opacity: OpacityScale::default(),
        focus_ring: FocusRingTokens {
            color: h("#346CA3"),
            width: 2.0,
            offset: 2.0,
        },
    }
}

/// Returns the Notion-inspired dark theme.
pub fn notion_dark_theme() -> Theme {
    Theme {
        name: "Notion Dark",
        mode: ThemeMode::Dark,
        colors: SemanticColors {
            surface_primary: h("#191919"),
            surface_secondary: h("#202020"),
            surface_tertiary: h("#2F2F2F"),
            surface_inverse: h("#EBEBEA"),
            text_primary: h("#EBEBEA"),
            text_secondary: h("#9B9A97"),
            text_tertiary: h("#6B6B6B"),
            text_disabled: h("#4B4B4B"),
            text_inverse: h("#191919"),
            text_link: h("#529CCA"),
            interactive_default: h("#529CCA"),
            interactive_hover: h("#4388B4"),
            interactive_pressed: h("#346CA3"),
            interactive_disabled: h("#3A3A3A"),
            border_default: h("#2F2F2F"),
            border_hover: h("#3A3A3A"),
            border_focus: h("#529CCA"),
            border_error: h("#EB5757"),
            feedback_success: h("#4ADE80"),
            feedback_warning: h("#FBBF24"),
            feedback_error: h("#EB5757"),
            feedback_info: h("#529CCA"),
        },
        spacing: SpacingScale::default(),
        radius: RadiusScale {
            none: 0.0,
            sm: 3.0,
            md: 4.0,
            lg: 6.0,
            xl: 8.0,
            full: 9999.0,
        },
        typography: TypographyScale {
            font_family: "Inter",
            font_size_xs: 11.0,
            font_size_sm: 13.0,
            font_size_md: 14.0,
            font_size_lg: 16.0,
            font_size_xl: 18.0,
            font_size_xxl: 20.0,
            font_weight_bold: 600.0,
            ..Default::default()
        },
        shadows: ShadowScale {
            none: Shadow::none(),
            sm: Shadow::new(0.0, 1.0, 2.0, 0.0, vec4(0.0, 0.0, 0.0, 0.20)),
            md: Shadow::new(0.0, 2.0, 4.0, 0.0, vec4(0.0, 0.0, 0.0, 0.28)),
            lg: Shadow::new(0.0, 4.0, 8.0, 0.0, vec4(0.0, 0.0, 0.0, 0.32)),
            xl: Shadow::new(0.0, 8.0, 16.0, 0.0, vec4(0.0, 0.0, 0.0, 0.38)),
        },
        elevation: ElevationScale {
            level0: Shadow::none(),
            level1: Shadow::new(0.0, 1.0, 2.0, 0.0, vec4(0.0, 0.0, 0.0, 0.24)),
            level2: Shadow::new(0.0, 2.0, 6.0, -1.0, vec4(0.0, 0.0, 0.0, 0.28)),
            level3: Shadow::new(0.0, 6.0, 16.0, -3.0, vec4(0.0, 0.0, 0.0, 0.32)),
            level4: Shadow::new(0.0, 12.0, 32.0, -6.0, vec4(0.0, 0.0, 0.0, 0.40)),
        },
        motion: MotionScale::default(),
        opacity: OpacityScale {
            disabled: 0.38,
            hover_overlay: 0.10,
            pressed_overlay: 0.16,
            backdrop: 0.6,
        },
        focus_ring: FocusRingTokens {
            color: h("#529CCA"),
            width: 2.0,
            offset: 2.0,
        },
    }
}
