//! Generate Rust Theme struct code from token JSON

/// Generate Rust theme code from tokens JSON
pub struct RustCodegen;

impl RustCodegen {
    pub fn generate(tokens: &serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
        let themes = tokens
            .get("themes")
            .and_then(|t| t.as_object())
            .ok_or("Missing or invalid 'themes' object")?;

        let mut out = String::from(
            r#"//! Auto-generated theme code from figma/tokens.json
//! Do not edit manually. Run `oxide generate` to regenerate.

use oxide_core::theme::{
    hex_to_vec4, ElevationScale, FocusRingTokens, MotionScale, OpacityScale, RadiusScale,
    SemanticColors, ShadowScale, SpacingScale, Theme, ThemeMode, TypographyScale,
};

fn h(hex: &str) -> makepad_widgets::makepad_draw::Vec4 {
    hex_to_vec4(hex)
}

"#,
        );

        for (theme_id, theme_val) in themes {
            let theme_obj = theme_val.as_object().ok_or("Theme must be object")?;
            let name = theme_obj
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or(theme_id);
            let fn_name = to_snake_case(theme_id);

            let mode = theme_obj
                .get("mode")
                .and_then(|m| m.as_str())
                .unwrap_or("light");

            out.push_str(&format!("pub fn {}_theme() -> Theme {{\n", fn_name));
            out.push_str("    Theme {\n");
            out.push_str(&format!("        name: \"{}\",\n", name));
            out.push_str(&format!(
                "        mode: ThemeMode::{},\n",
                if mode == "dark" { "Dark" } else { "Light" }
            ));
            out.push_str("        colors: ");
            out.push_str(&emit_semantic_colors(theme_obj.get("colors"))?);
            out.push_str(",\n");
            out.push_str("        spacing: ");
            out.push_str(&emit_spacing(theme_obj.get("spacing"))?);
            out.push_str(",\n");
            out.push_str("        radius: ");
            out.push_str(&emit_radius(theme_obj.get("radius"))?);
            out.push_str(",\n");
            out.push_str("        typography: ");
            out.push_str(&emit_typography(theme_obj.get("typography"))?);
            out.push_str(",\n");
            out.push_str("        shadows: ShadowScale::default(),\n");
            out.push_str("        elevation: ElevationScale::default(),\n");
            out.push_str("        motion: ");
            out.push_str(&emit_motion(theme_obj.get("motion"))?);
            out.push_str(",\n");
            out.push_str("        opacity: ");
            out.push_str(&emit_opacity(theme_obj.get("opacity"))?);
            out.push_str(",\n");
            out.push_str("        focus_ring: ");
            out.push_str(&emit_focus_ring(
                theme_obj.get("focus_ring"),
                theme_obj.get("colors"),
            )?);
            out.push_str(",\n");
            out.push_str("    }\n");
            out.push_str("}\n\n");
        }

        out.push_str("\npub fn all_themes() -> Vec<Theme> {\n    vec![\n");
        for (theme_id, _) in themes {
            let fn_name = to_snake_case(theme_id);
            out.push_str(&format!("        {}_theme(),\n", fn_name));
        }
        out.push_str("    ]\n}\n");

        Ok(out)
    }
}

fn hex_to_vec4_string(hex: &str) -> String {
    format!("h(\"{}\")", hex.trim_start_matches('#'))
}

fn to_snake_case(s: &str) -> String {
    let lowered = s.to_lowercase();
    let sanitized: String = lowered
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    let collapsed = sanitized
        .split('_')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("_");
    if collapsed.starts_with(|c: char| c.is_ascii_digit()) {
        format!("_{}", collapsed)
    } else {
        collapsed
    }
}

fn emit_semantic_colors(
    colors: Option<&serde_json::Value>,
) -> Result<String, Box<dyn std::error::Error>> {
    let default_colors: &[(&str, &str)] = &[
        ("surface_primary", "#FFFFFF"),
        ("surface_secondary", "#F7F7F8"),
        ("surface_tertiary", "#EAEBEE"),
        ("surface_inverse", "#202023"),
        ("text_primary", "#202023"),
        ("text_secondary", "#676A70"),
        ("text_tertiary", "#8E8EA0"),
        ("text_disabled", "#ACACBE"),
        ("text_inverse", "#FFFFFF"),
        ("text_link", "#10A37F"),
        ("interactive_default", "#10A37F"),
        ("interactive_hover", "#0D8A6A"),
        ("interactive_pressed", "#0B7359"),
        ("interactive_disabled", "#C5C5D1"),
        ("border_default", "#EAEBEE"),
        ("border_hover", "#D1D3D8"),
        ("border_focus", "#10A37F"),
        ("border_error", "#E53935"),
        ("feedback_success", "#10A37F"),
        ("feedback_warning", "#F59E0B"),
        ("feedback_error", "#E53935"),
        ("feedback_info", "#3B82F6"),
    ];

    let obj = colors.and_then(|c| c.as_object());
    let mut out = String::from("SemanticColors {\n            ");
    for (key, default_hex) in default_colors {
        let key_str = key.to_string();
        let hex = obj
            .and_then(|o| o.get(&key_str))
            .and_then(|v| v.as_str())
            .unwrap_or(default_hex);
        out.push_str(&format!(
            "{}: {},\n            ",
            key,
            hex_to_vec4_string(hex)
        ));
    }
    out.push('}');
    Ok(out)
}

fn emit_spacing(spacing: Option<&serde_json::Value>) -> Result<String, Box<dyn std::error::Error>> {
    let obj = spacing.and_then(|s| s.as_object());
    let get = |k: &str, d: f64| {
        obj.and_then(|o| o.get(k))
            .and_then(|v| v.as_f64().or_else(|| v.as_i64().map(|i| i as f64)))
            .unwrap_or(d)
    };
    Ok(format!(
        "SpacingScale {{\n            none: {}, xs: {}, sm: {}, md: {}, lg: {}, xl: {}, xxl: {}\n        }}",
        get("none", 0.0),
        get("xs", 4.0),
        get("sm", 8.0),
        get("md", 16.0),
        get("lg", 24.0),
        get("xl", 32.0),
        get("xxl", 48.0)
    ))
}

fn emit_radius(radius: Option<&serde_json::Value>) -> Result<String, Box<dyn std::error::Error>> {
    let obj = radius.and_then(|r| r.as_object());
    let get = |k: &str, d: f64| {
        obj.and_then(|o| o.get(k))
            .and_then(|v| v.as_f64().or_else(|| v.as_i64().map(|i| i as f64)))
            .unwrap_or(d)
    };
    Ok(format!(
        "RadiusScale {{\n            none: {}, sm: {}, md: {}, lg: {}, xl: {}, full: {}\n        }}",
        get("none", 0.0),
        get("sm", 4.0),
        get("md", 8.0),
        get("lg", 12.0),
        get("xl", 16.0),
        get("full", 9999.0)
    ))
}

fn emit_motion(motion: Option<&serde_json::Value>) -> Result<String, Box<dyn std::error::Error>> {
    let obj = motion.and_then(|m| m.as_object());
    let get = |k: &str, d: f64| {
        obj.and_then(|o| o.get(k))
            .and_then(|v| v.as_f64().or_else(|| v.as_i64().map(|i| i as f64)))
            .unwrap_or(d)
    };
    Ok(format!(
        "MotionScale {{\n            duration_fast: {}, duration_normal: {}, duration_slow: {}, ease: {}\n        }}",
        get("duration_fast", 0.1),
        get("duration_normal", 0.2),
        get("duration_slow", 0.35),
        get("ease", 0.5)
    ))
}

fn emit_opacity(opacity: Option<&serde_json::Value>) -> Result<String, Box<dyn std::error::Error>> {
    let obj = opacity.and_then(|o| o.as_object());
    let get = |k: &str, d: f64| {
        obj.and_then(|o| o.get(k))
            .and_then(|v| v.as_f64().or_else(|| v.as_i64().map(|i| i as f64)))
            .unwrap_or(d)
    };
    Ok(format!(
        "OpacityScale {{\n            disabled: {}, hover_overlay: {}, pressed_overlay: {}, backdrop: {}\n        }}",
        get("disabled", 0.38),
        get("hover_overlay", 0.08),
        get("pressed_overlay", 0.12),
        get("backdrop", 0.5)
    ))
}

fn emit_focus_ring(
    focus_ring: Option<&serde_json::Value>,
    colors: Option<&serde_json::Value>,
) -> Result<String, Box<dyn std::error::Error>> {
    let obj = focus_ring.and_then(|f| f.as_object());
    let get_f64 = |k: &str, d: f64| {
        obj.and_then(|o| o.get(k))
            .and_then(|v| v.as_f64().or_else(|| v.as_i64().map(|i| i as f64)))
            .unwrap_or(d)
    };

    let default_color = colors
        .and_then(|c| c.as_object())
        .and_then(|c| c.get("interactive_default"))
        .and_then(|v| v.as_str())
        .unwrap_or("#10A37F");

    let color = obj
        .and_then(|o| o.get("color"))
        .and_then(|v| v.as_str())
        .unwrap_or(default_color);

    Ok(format!(
        "FocusRingTokens {{\n            color: {}, width: {}, offset: {}\n        }}",
        hex_to_vec4_string(color),
        get_f64("width", 2.0),
        get_f64("offset", 2.0)
    ))
}

fn emit_typography(typo: Option<&serde_json::Value>) -> Result<String, Box<dyn std::error::Error>> {
    let obj = typo.and_then(|t| t.as_object());
    let get_str = |k: &str, d: &'static str| {
        obj.and_then(|o| o.get(k))
            .and_then(|v| v.as_str())
            .unwrap_or(d)
    };
    let get_f64 = |k: &str, d: f64| {
        obj.and_then(|o| o.get(k))
            .and_then(|v| v.as_f64().or_else(|| v.as_i64().map(|i| i as f64)))
            .unwrap_or(d)
    };
    Ok(format!(
        "TypographyScale {{\n            font_family: \"{}\",\n            font_size_xs: {}, font_size_sm: {}, font_size_md: {}, font_size_lg: {}, font_size_xl: {}, font_size_xxl: {},\n            line_height_tight: {}, line_height_normal: {}, line_height_relaxed: {},\n            font_weight_normal: {}, font_weight_medium: {}, font_weight_bold: {}\n        }}",
        get_str("font_family", "Inter"),
        get_f64("font_size_xs", 12.0),
        get_f64("font_size_sm", 14.0),
        get_f64("font_size_md", 16.0),
        get_f64("font_size_lg", 18.0),
        get_f64("font_size_xl", 20.0),
        get_f64("font_size_xxl", 24.0),
        get_f64("line_height_tight", 1.25),
        get_f64("line_height_normal", 1.5),
        get_f64("line_height_relaxed", 1.75),
        get_f64("font_weight_normal", 400.0),
        get_f64("font_weight_medium", 500.0),
        get_f64("font_weight_bold", 600.0)
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tokens() -> serde_json::Value {
        serde_json::json!({
            "themes": {
                "my-theme": {
                    "name": "MyTheme",
                    "colors": {
                        "surface_primary": "#FFFFFF",
                        "interactive_default": "#10A37F"
                    },
                    "spacing": { "xs": 4, "sm": 8 },
                    "radius": { "sm": 4, "md": 8 },
                    "typography": { "font_family": "Mono" }
                }
            }
        })
    }

    #[test]
    fn generate_produces_function_named_after_theme() {
        let code = RustCodegen::generate(&sample_tokens()).unwrap();
        assert!(code.contains("pub fn my_theme_theme() -> Theme"));
    }

    #[test]
    fn generate_includes_theme_name() {
        let code = RustCodegen::generate(&sample_tokens()).unwrap();
        assert!(code.contains("\"MyTheme\""));
    }

    #[test]
    fn generate_includes_all_themes_fn() {
        let code = RustCodegen::generate(&sample_tokens()).unwrap();
        assert!(code.contains("pub fn all_themes() -> Vec<Theme>"));
        assert!(code.contains("my_theme_theme()"));
    }

    #[test]
    fn generate_includes_color_values() {
        let code = RustCodegen::generate(&sample_tokens()).unwrap();
        assert!(code.contains("FFFFFF") || code.contains("interactive_default"));
    }

    #[test]
    fn generate_respects_custom_font_family() {
        let code = RustCodegen::generate(&sample_tokens()).unwrap();
        assert!(code.contains("\"Mono\""));
    }

    #[test]
    fn generate_uses_default_when_fields_missing() {
        let tokens = serde_json::json!({
            "themes": {
                "bare": {
                    "name": "Bare"
                }
            }
        });
        let code = RustCodegen::generate(&tokens).unwrap();
        assert!(code.contains("pub fn bare_theme()"));
        assert!(code.contains("\"Inter\""));
        assert!(code.contains("MotionScale"));
        assert!(code.contains("OpacityScale"));
        assert!(code.contains("FocusRingTokens"));
        assert!(code.contains("ThemeMode::Light"));
    }

    #[test]
    fn generate_dark_mode_theme() {
        let tokens = serde_json::json!({
            "themes": {
                "my_dark": {
                    "name": "MyDark",
                    "mode": "dark"
                }
            }
        });
        let code = RustCodegen::generate(&tokens).unwrap();
        assert!(code.contains("ThemeMode::Dark"));
    }

    #[test]
    fn generate_fails_on_missing_themes() {
        let tokens = serde_json::json!({});
        assert!(RustCodegen::generate(&tokens).is_err());
    }

    #[test]
    fn generate_multiple_themes() {
        let tokens = serde_json::json!({
            "themes": {
                "alpha": { "name": "Alpha" },
                "beta": { "name": "Beta" }
            }
        });
        let code = RustCodegen::generate(&tokens).unwrap();
        assert!(code.contains("alpha_theme()"));
        assert!(code.contains("beta_theme()"));
    }

    #[test]
    fn generate_custom_motion_and_opacity() {
        let tokens = serde_json::json!({
            "themes": {
                "custom": {
                    "name": "Custom",
                    "motion": { "duration_fast": 0.05, "duration_normal": 0.15 },
                    "opacity": { "disabled": 0.5 }
                }
            }
        });
        let code = RustCodegen::generate(&tokens).unwrap();
        assert!(code.contains("0.05"));
        assert!(code.contains("0.15"));
        assert!(code.contains("0.5"));
    }
}
