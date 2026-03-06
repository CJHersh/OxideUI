use makepad_widgets::makepad_draw::*;

/// Whether the theme targets a light or dark background.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}

/// Complete theme definition containing all token scales.
#[derive(Clone, Debug)]
pub struct Theme {
    pub name: &'static str,
    pub mode: ThemeMode,
    pub colors: SemanticColors,
    pub spacing: SpacingScale,
    pub radius: RadiusScale,
    pub typography: TypographyScale,
    pub shadows: ShadowScale,
    pub elevation: ElevationScale,
    pub motion: MotionScale,
    pub opacity: OpacityScale,
    pub focus_ring: FocusRingTokens,
}

/// Semantic color tokens for surfaces, text, interactive states, borders, and feedback.
#[derive(Clone, Debug)]
pub struct SemanticColors {
    pub surface_primary: Vec4,
    pub surface_secondary: Vec4,
    pub surface_tertiary: Vec4,
    pub surface_inverse: Vec4,
    pub text_primary: Vec4,
    pub text_secondary: Vec4,
    pub text_tertiary: Vec4,
    pub text_disabled: Vec4,
    pub text_inverse: Vec4,
    pub text_link: Vec4,
    pub interactive_default: Vec4,
    pub interactive_hover: Vec4,
    pub interactive_pressed: Vec4,
    pub interactive_disabled: Vec4,
    pub border_default: Vec4,
    pub border_hover: Vec4,
    pub border_focus: Vec4,
    pub border_error: Vec4,
    pub feedback_success: Vec4,
    pub feedback_warning: Vec4,
    pub feedback_error: Vec4,
    pub feedback_info: Vec4,
}

impl Default for SemanticColors {
    fn default() -> Self {
        Self {
            surface_primary: hex_to_vec4("#FFFFFF"),
            surface_secondary: hex_to_vec4("#F7F7F8"),
            surface_tertiary: hex_to_vec4("#EAEBEE"),
            surface_inverse: hex_to_vec4("#202023"),
            text_primary: hex_to_vec4("#202023"),
            text_secondary: hex_to_vec4("#676A70"),
            text_tertiary: hex_to_vec4("#8E8EA0"),
            text_disabled: hex_to_vec4("#ACACBE"),
            text_inverse: hex_to_vec4("#FFFFFF"),
            text_link: hex_to_vec4("#10A37F"),
            interactive_default: hex_to_vec4("#10A37F"),
            interactive_hover: hex_to_vec4("#0D8A6A"),
            interactive_pressed: hex_to_vec4("#0B7359"),
            interactive_disabled: hex_to_vec4("#C5C5D1"),
            border_default: hex_to_vec4("#EAEBEE"),
            border_hover: hex_to_vec4("#D1D3D8"),
            border_focus: hex_to_vec4("#10A37F"),
            border_error: hex_to_vec4("#E53935"),
            feedback_success: hex_to_vec4("#10A37F"),
            feedback_warning: hex_to_vec4("#F59E0B"),
            feedback_error: hex_to_vec4("#E53935"),
            feedback_info: hex_to_vec4("#3B82F6"),
        }
    }
}

/// Spacing scale based on a 4px grid.
///
/// **Status**: Token values are defined and available via `Theme::spacing`,
/// but are not yet applied at runtime by `apply_*_theme` functions. Spacing
/// is currently set in the widget DSL. Runtime spacing application via
/// `apply_over!` is planned for a future release.
#[derive(Clone, Debug)]
pub struct SpacingScale {
    pub none: f64,
    pub xs: f64,
    pub sm: f64,
    pub md: f64,
    pub lg: f64,
    pub xl: f64,
    pub xxl: f64,
}

impl Default for SpacingScale {
    fn default() -> Self {
        Self {
            none: 0.0,
            xs: 4.0,
            sm: 8.0,
            md: 16.0,
            lg: 24.0,
            xl: 32.0,
            xxl: 48.0,
        }
    }
}

/// Border radius scale.
#[derive(Clone, Debug)]
pub struct RadiusScale {
    pub none: f64,
    pub sm: f64,
    pub md: f64,
    pub lg: f64,
    pub xl: f64,
    pub full: f64,
}

impl Default for RadiusScale {
    fn default() -> Self {
        Self {
            none: 0.0,
            sm: 4.0,
            md: 8.0,
            lg: 12.0,
            xl: 16.0,
            full: 9999.0,
        }
    }
}

/// Typography tokens including font sizes, line heights, and weights.
///
/// **Status**: Token values are defined and available via `Theme::typography`,
/// but are not yet applied at runtime by `apply_*_theme` functions. Font sizes
/// and families are currently set in the widget DSL. Runtime typography
/// application via `apply_over!` is planned for a future release.
#[derive(Clone, Debug)]
pub struct TypographyScale {
    pub font_family: &'static str,
    pub font_size_xs: f64,
    pub font_size_sm: f64,
    pub font_size_md: f64,
    pub font_size_lg: f64,
    pub font_size_xl: f64,
    pub font_size_xxl: f64,
    pub line_height_tight: f64,
    pub line_height_normal: f64,
    pub line_height_relaxed: f64,
    pub font_weight_normal: f64,
    pub font_weight_medium: f64,
    pub font_weight_bold: f64,
}

impl Default for TypographyScale {
    fn default() -> Self {
        Self {
            font_family: "Inter",
            font_size_xs: 12.0,
            font_size_sm: 14.0,
            font_size_md: 16.0,
            font_size_lg: 18.0,
            font_size_xl: 20.0,
            font_size_xxl: 24.0,
            line_height_tight: 1.25,
            line_height_normal: 1.5,
            line_height_relaxed: 1.75,
            font_weight_normal: 400.0,
            font_weight_medium: 500.0,
            font_weight_bold: 600.0,
        }
    }
}

/// Shadow elevation scale.
///
/// **Status**: Token values are defined and available via `Theme::shadows`,
/// but no widget shader currently renders shadows from these tokens. Shadow
/// rendering in component shaders (Card, Popover, etc.) is planned for a
/// future release.
#[derive(Clone, Debug)]
pub struct ShadowScale {
    pub none: Shadow,
    pub sm: Shadow,
    pub md: Shadow,
    pub lg: Shadow,
    pub xl: Shadow,
}

impl Default for ShadowScale {
    fn default() -> Self {
        Self {
            none: Shadow::none(),
            sm: Shadow::new(0.0, 1.0, 2.0, 0.0, vec4(0.0, 0.0, 0.0, 0.05)),
            md: Shadow::new(0.0, 2.0, 4.0, 0.0, vec4(0.0, 0.0, 0.0, 0.1)),
            lg: Shadow::new(0.0, 4.0, 8.0, 0.0, vec4(0.0, 0.0, 0.0, 0.12)),
            xl: Shadow::new(0.0, 8.0, 16.0, 0.0, vec4(0.0, 0.0, 0.0, 0.15)),
        }
    }
}

/// Individual shadow definition with offset, blur, spread, and color.
#[derive(Clone, Debug)]
pub struct Shadow {
    pub offset_x: f64,
    pub offset_y: f64,
    pub blur: f64,
    pub spread: f64,
    pub color: Vec4,
}

impl Shadow {
    /// Creates a shadow with no visible effect.
    pub fn none() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
            blur: 0.0,
            spread: 0.0,
            color: vec4(0.0, 0.0, 0.0, 0.0),
        }
    }

    /// Creates a shadow with the given offset, blur, spread, and color.
    pub fn new(offset_x: f64, offset_y: f64, blur: f64, spread: f64, color: Vec4) -> Self {
        Self {
            offset_x,
            offset_y,
            blur,
            spread,
            color,
        }
    }
}

/// Layered elevation scale mapping UI depth to shadow presets.
///
/// Each level combines a shadow offset, blur, and opacity appropriate for its
/// visual depth. Components reference levels (e.g. `level1` for cards,
/// `level3` for modals) rather than raw shadow values.
///
/// **Status**: Token values are defined and available via `Theme::elevation`,
/// but no widget shader currently renders elevation from these tokens. This
/// is planned for a future release alongside `ShadowScale` integration.
#[derive(Clone, Debug)]
pub struct ElevationScale {
    pub level0: Shadow,
    pub level1: Shadow,
    pub level2: Shadow,
    pub level3: Shadow,
    pub level4: Shadow,
}

impl Default for ElevationScale {
    fn default() -> Self {
        Self {
            level0: Shadow::none(),
            level1: Shadow::new(0.0, 1.0, 3.0, 0.0, vec4(0.0, 0.0, 0.0, 0.08)),
            level2: Shadow::new(0.0, 3.0, 8.0, -1.0, vec4(0.0, 0.0, 0.0, 0.10)),
            level3: Shadow::new(0.0, 8.0, 24.0, -4.0, vec4(0.0, 0.0, 0.0, 0.12)),
            level4: Shadow::new(0.0, 16.0, 48.0, -8.0, vec4(0.0, 0.0, 0.0, 0.16)),
        }
    }
}

/// Animation duration and easing tokens.
///
/// **Status**: Token values are defined and available via `Theme::motion`,
/// but are not yet consumed by widget animations. Integration with Makepad's
/// animator system is planned for a future release.
#[derive(Clone, Debug)]
pub struct MotionScale {
    /// Fast micro-interactions (button press, checkbox tick) in seconds.
    pub duration_fast: f64,
    /// Normal transitions (hover states, focus rings) in seconds.
    pub duration_normal: f64,
    /// Slow transitions (page fades, drawer slides) in seconds.
    pub duration_slow: f64,
    /// Default easing function factor (0.0 = linear, higher = more ease).
    pub ease: f64,
}

impl Default for MotionScale {
    fn default() -> Self {
        Self {
            duration_fast: 0.1,
            duration_normal: 0.2,
            duration_slow: 0.35,
            ease: 0.5,
        }
    }
}

/// Opacity tokens for interactive overlays and disabled states.
#[derive(Clone, Debug)]
pub struct OpacityScale {
    /// Opacity for disabled elements (text, icons, surfaces).
    pub disabled: f64,
    /// Background overlay opacity on hover (e.g. 8% tint).
    pub hover_overlay: f64,
    /// Background overlay opacity on press (e.g. 12% tint).
    pub pressed_overlay: f64,
    /// Backdrop overlay for modals / drawers.
    pub backdrop: f64,
}

impl Default for OpacityScale {
    fn default() -> Self {
        Self {
            disabled: 0.38,
            hover_overlay: 0.08,
            pressed_overlay: 0.12,
            backdrop: 0.5,
        }
    }
}

/// Focus ring visual tokens for keyboard navigation indicators.
#[derive(Clone, Debug)]
pub struct FocusRingTokens {
    pub color: Vec4,
    pub width: f64,
    pub offset: f64,
}

impl Default for FocusRingTokens {
    fn default() -> Self {
        Self {
            color: hex_to_vec4("#10A37F"),
            width: 2.0,
            offset: 2.0,
        }
    }
}

/// Converts a hex color string (#RRGGBB or #RRGGBBAA) to a Vec4.
pub fn hex_to_vec4(hex: &str) -> Vec4 {
    let hex = hex.trim_start_matches('#');
    let len = hex.len();
    let v = u64::from_str_radix(hex, 16).unwrap_or(0);
    if len == 6 {
        let r = ((v >> 16) & 0xFF) as f32 / 255.0;
        let g = ((v >> 8) & 0xFF) as f32 / 255.0;
        let b = (v & 0xFF) as f32 / 255.0;
        vec4(r, g, b, 1.0)
    } else if len == 8 {
        let r = ((v >> 24) & 0xFF) as f32 / 255.0;
        let g = ((v >> 16) & 0xFF) as f32 / 255.0;
        let b = ((v >> 8) & 0xFF) as f32 / 255.0;
        let a = (v & 0xFF) as f32 / 255.0;
        vec4(r, g, b, a)
    } else {
        vec4(0.0, 0.0, 0.0, 1.0)
    }
}
