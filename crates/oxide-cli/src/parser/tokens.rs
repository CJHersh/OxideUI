#![allow(non_snake_case)]
//! Convert Figma VariablesResponse to OxideUI tokens JSON format

use crate::figma::types::VariablesResponse;

/// Convert Figma 0.0–1.0 RGBA to a hex string (#RRGGBB or #RRGGBBAA).
pub fn rgba_to_hex(r: f64, g: f64, b: f64, a: f64) -> String {
    let ri = (r.clamp(0.0, 1.0) * 255.0).round() as u8;
    let gi = (g.clamp(0.0, 1.0) * 255.0).round() as u8;
    let bi = (b.clamp(0.0, 1.0) * 255.0).round() as u8;
    let ai = (a.clamp(0.0, 1.0) * 255.0).round() as u8;
    if ai == 255 {
        format!("#{:02X}{:02X}{:02X}", ri, gi, bi)
    } else {
        format!("#{:02X}{:02X}{:02X}{:02X}", ri, gi, bi, ai)
    }
}

/// Resolve a value that may be a VARIABLE_ALIAS by following the alias chain.
///
/// Figma aliases have the shape `{"type": "VARIABLE_ALIAS", "id": "VariableID:..."}`.
/// This function follows up to 10 levels of indirection to prevent infinite loops.
fn resolve_value(
    value: &serde_json::Value,
    mode_id: &str,
    variables: &std::collections::HashMap<String, crate::figma::types::Variable>,
) -> serde_json::Value {
    let mut current = value.clone();
    for _ in 0..10 {
        let is_alias = current
            .as_object()
            .and_then(|obj| obj.get("type"))
            .and_then(|t| t.as_str())
            == Some("VARIABLE_ALIAS");
        if !is_alias {
            return current;
        }
        let alias_id = match current
            .as_object()
            .and_then(|obj| obj.get("id"))
            .and_then(|v| v.as_str())
        {
            Some(id) => id.to_string(),
            None => return current,
        };
        match variables.get(&alias_id) {
            Some(target_var) => match target_var.valuesByMode.get(mode_id) {
                Some(v) => current = v.clone(),
                None => return current,
            },
            None => return current,
        }
    }
    current
}

/// Convert Figma VariablesResponse to serde_json::Value (tokens format).
///
/// When `response.meta` contains real variable data (a non-empty `variables`
/// HashMap), the function resolves COLOR values to hex strings and NUMBER
/// values to f64, grouping them by name prefix and building one theme per
/// collection/mode combination.
///
/// When no real data is present it falls back to the top-level
/// `variableCollections` vec (legacy path) and finally to hardcoded default
/// themes.
pub fn figma_to_tokens(response: &VariablesResponse) -> serde_json::Value {
    let mut themes = serde_json::Map::new();

    let has_real_data = response
        .meta
        .as_ref()
        .map(|m| !m.variables.is_empty())
        .unwrap_or(false);

    if has_real_data {
        let meta = response.meta.as_ref().unwrap();

        for collection in meta.variableCollections.values() {
            for mode in &collection.modes {
                let theme_id = if collection.modes.len() == 1 {
                    collection.name.to_lowercase().replace(' ', "_")
                } else {
                    format!(
                        "{}_{}",
                        collection.name.to_lowercase().replace(' ', "_"),
                        mode.name.to_lowercase().replace(' ', "_")
                    )
                };

                let theme_label = if collection.modes.len() == 1 {
                    collection.name.clone()
                } else {
                    format!("{} ({})", collection.name, mode.name)
                };

                let mut colors = serde_json::Map::new();
                let mut spacing = serde_json::Map::new();
                let mut radius_tokens = serde_json::Map::new();
                let mut typography_tokens = serde_json::Map::new();

                for var_id in &collection.variableIds {
                    if let Some(variable) = meta.variables.get(var_id) {
                        if let Some(value) = variable.valuesByMode.get(&mode.modeId) {
                            let parts: Vec<&str> = variable.name.split('/').collect();
                            let (category, token_name) = if parts.len() >= 2 {
                                (
                                    parts[0].to_lowercase(),
                                    parts[1..].join("_").replace(['-', ' '], "_"),
                                )
                            } else {
                                (
                                    "other".to_string(),
                                    variable.name.replace(['/', '-', ' '], "_"),
                                )
                            };

                            let resolved_value =
                                resolve_value(value, &mode.modeId, &meta.variables);

                            match variable.resolvedType.as_str() {
                                "COLOR" => {
                                    if let Some(obj) = resolved_value.as_object() {
                                        let r =
                                            obj.get("r").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let g =
                                            obj.get("g").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let b =
                                            obj.get("b").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let a =
                                            obj.get("a").and_then(|v| v.as_f64()).unwrap_or(1.0);
                                        let hex = rgba_to_hex(r, g, b, a);
                                        colors.insert(token_name, serde_json::json!(hex));
                                    }
                                }
                                "FLOAT" | "NUMBER" => {
                                    if let Some(n) = resolved_value.as_f64() {
                                        match category.as_str() {
                                            "spacing" => {
                                                spacing.insert(token_name, serde_json::json!(n));
                                            }
                                            "radius" => {
                                                radius_tokens
                                                    .insert(token_name, serde_json::json!(n));
                                            }
                                            "typography" | "font" | "text" => {
                                                typography_tokens
                                                    .insert(token_name, serde_json::json!(n));
                                            }
                                            _ => {
                                                spacing.insert(
                                                    format!("{}_{}", category, token_name),
                                                    serde_json::json!(n),
                                                );
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }

                let mut theme = serde_json::Map::new();
                theme.insert("name".to_string(), serde_json::json!(theme_label));
                theme.insert("colors".to_string(), serde_json::Value::Object(colors));
                theme.insert(
                    "spacing".to_string(),
                    if spacing.is_empty() {
                        default_spacing()
                    } else {
                        serde_json::Value::Object(spacing)
                    },
                );
                theme.insert(
                    "radius".to_string(),
                    if radius_tokens.is_empty() {
                        default_radius()
                    } else {
                        serde_json::Value::Object(radius_tokens)
                    },
                );
                theme.insert(
                    "typography".to_string(),
                    if typography_tokens.is_empty() {
                        default_typography()
                    } else {
                        serde_json::Value::Object(typography_tokens)
                    },
                );

                themes.insert(theme_id, serde_json::Value::Object(theme));
            }
        }
    }

    // Legacy path: top-level variableCollections vec (no resolved variable data)
    if themes.is_empty() {
        for collection in &response.variableCollections {
            let theme_id = collection.name.to_lowercase().replace(' ', "_");
            let mut theme = serde_json::Map::new();
            theme.insert("name".to_string(), serde_json::json!(collection.name));

            let colors = default_colors_for_theme(&collection.name);
            theme.insert("colors".to_string(), colors);
            theme.insert("spacing".to_string(), default_spacing());
            theme.insert("radius".to_string(), default_radius());
            theme.insert("typography".to_string(), default_typography());

            themes.insert(theme_id, serde_json::Value::Object(theme));
        }
    }

    // Hardcoded fallback when the API returned nothing useful
    if themes.is_empty() {
        themes.insert(
            "openai".to_string(),
            build_theme_json("OpenAI", openai_colors()),
        );
        themes.insert(
            "airbnb".to_string(),
            build_theme_json("Airbnb", airbnb_colors()),
        );
        themes.insert(
            "notion".to_string(),
            build_theme_json("Notion", notion_colors()),
        );
    }

    serde_json::json!({
        "version": "1.0",
        "themes": themes
    })
}

fn default_colors_for_theme(name: &str) -> serde_json::Value {
    let colors = match name.to_lowercase().as_str() {
        "airbnb" => airbnb_colors(),
        "notion" => notion_colors(),
        _ => openai_colors(),
    };
    serde_json::to_value(colors).unwrap_or(serde_json::json!({}))
}

fn default_spacing() -> serde_json::Value {
    serde_json::json!({
        "none": 0,
        "xs": 4,
        "sm": 8,
        "md": 16,
        "lg": 24,
        "xl": 32,
        "xxl": 48
    })
}

fn default_radius() -> serde_json::Value {
    serde_json::json!({
        "none": 0,
        "sm": 4,
        "md": 8,
        "lg": 12,
        "xl": 16,
        "full": 9999
    })
}

fn default_typography() -> serde_json::Value {
    serde_json::json!({
        "font_family": "Inter",
        "font_size_xs": 12,
        "font_size_sm": 14,
        "font_size_md": 16,
        "font_size_lg": 18,
        "font_size_xl": 20,
        "font_size_xxl": 24,
        "line_height_tight": 1.25,
        "line_height_normal": 1.5,
        "line_height_relaxed": 1.75,
        "font_weight_normal": 400,
        "font_weight_medium": 500,
        "font_weight_bold": 600
    })
}

fn build_theme_json(name: &str, colors: serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "name": name,
        "colors": colors,
        "spacing": default_spacing(),
        "radius": default_radius(),
        "typography": default_typography()
    })
}

fn openai_colors() -> serde_json::Value {
    serde_json::json!({
        "surface_primary": "#FFFFFF",
        "surface_secondary": "#F7F7F8",
        "surface_tertiary": "#EAEBEE",
        "surface_inverse": "#202023",
        "text_primary": "#202023",
        "text_secondary": "#676A70",
        "text_tertiary": "#8E8EA0",
        "text_disabled": "#ACACBE",
        "text_inverse": "#FFFFFF",
        "text_link": "#10A37F",
        "interactive_default": "#10A37F",
        "interactive_hover": "#0D8A6A",
        "interactive_pressed": "#0B7359",
        "interactive_disabled": "#C5C5D1",
        "border_default": "#EAEBEE",
        "border_hover": "#D1D3D8",
        "border_focus": "#10A37F",
        "border_error": "#E53935",
        "feedback_success": "#10A37F",
        "feedback_warning": "#F59E0B",
        "feedback_error": "#E53935",
        "feedback_info": "#3B82F6"
    })
}

fn airbnb_colors() -> serde_json::Value {
    serde_json::json!({
        "surface_primary": "#FFFFFF",
        "surface_secondary": "#F7F7F7",
        "surface_tertiary": "#EFEFEF",
        "surface_inverse": "#222222",
        "text_primary": "#222222",
        "text_secondary": "#717171",
        "text_tertiary": "#999999",
        "text_disabled": "#B0B0B0",
        "text_inverse": "#FFFFFF",
        "text_link": "#FF5A5F",
        "interactive_default": "#FF5A5F",
        "interactive_hover": "#E84E53",
        "interactive_pressed": "#D9454A",
        "interactive_disabled": "#CCCCCC",
        "border_default": "#DDDDDD",
        "border_hover": "#CCCCCC",
        "border_focus": "#FF5A5F",
        "border_error": "#C13515",
        "feedback_success": "#008A05",
        "feedback_warning": "#FFB400",
        "feedback_error": "#C13515",
        "feedback_info": "#00A699"
    })
}

fn notion_colors() -> serde_json::Value {
    serde_json::json!({
        "surface_primary": "#FFFFFF",
        "surface_secondary": "#F9F9F8",
        "surface_tertiary": "#EFEFED",
        "surface_inverse": "#252525",
        "text_primary": "#252525",
        "text_secondary": "#737373",
        "text_tertiary": "#9B9A97",
        "text_disabled": "#B3B2AF",
        "text_inverse": "#FFFFFF",
        "text_link": "#346CA3",
        "interactive_default": "#346CA3",
        "interactive_hover": "#2B5A8A",
        "interactive_pressed": "#244A73",
        "interactive_disabled": "#D3D3D1",
        "border_default": "#E3E2E0",
        "border_hover": "#D3D3D1",
        "border_focus": "#346CA3",
        "border_error": "#EB5757",
        "feedback_success": "#0F9D58",
        "feedback_warning": "#F2994A",
        "feedback_error": "#EB5757",
        "feedback_info": "#346CA3"
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::figma::types::VariablesResponse;

    fn empty_response() -> VariablesResponse {
        VariablesResponse {
            meta: None,
            variableCollections: vec![],
        }
    }

    #[test]
    fn empty_response_produces_default_themes() {
        let result = figma_to_tokens(&empty_response());
        let themes = result.get("themes").unwrap().as_object().unwrap();
        assert!(themes.contains_key("openai"));
        assert!(themes.contains_key("airbnb"));
        assert!(themes.contains_key("notion"));
    }

    #[test]
    fn empty_response_has_version() {
        let result = figma_to_tokens(&empty_response());
        assert_eq!(result.get("version").unwrap().as_str().unwrap(), "1.0");
    }

    #[test]
    fn default_themes_have_required_structure() {
        let result = figma_to_tokens(&empty_response());
        let themes = result.get("themes").unwrap().as_object().unwrap();
        for (_id, theme) in themes {
            let obj = theme.as_object().unwrap();
            assert!(obj.contains_key("name"));
            assert!(obj.contains_key("colors"));
            assert!(obj.contains_key("spacing"));
            assert!(obj.contains_key("radius"));
            assert!(obj.contains_key("typography"));
        }
    }

    #[test]
    fn collection_creates_theme_entry() {
        let response = VariablesResponse {
            meta: None,
            variableCollections: vec![crate::figma::types::VariableCollection {
                name: "Custom Brand".to_string(),
                modes: vec![],
                variableIds: vec![],
            }],
        };
        let result = figma_to_tokens(&response);
        let themes = result.get("themes").unwrap().as_object().unwrap();
        assert!(themes.contains_key("custom_brand"));
        let custom = themes.get("custom_brand").unwrap().as_object().unwrap();
        assert_eq!(
            custom.get("name").unwrap().as_str().unwrap(),
            "Custom Brand"
        );
    }

    #[test]
    fn openai_colors_have_all_fields() {
        let colors = openai_colors();
        let obj = colors.as_object().unwrap();
        assert!(obj.contains_key("surface_primary"));
        assert!(obj.contains_key("interactive_default"));
        assert!(obj.contains_key("feedback_error"));
        assert!(obj.contains_key("border_default"));
    }

    #[test]
    fn airbnb_colors_differ_from_openai() {
        let oc = openai_colors();
        let ac = airbnb_colors();
        assert_ne!(
            oc.get("interactive_default").unwrap().as_str().unwrap(),
            ac.get("interactive_default").unwrap().as_str().unwrap()
        );
    }

    #[test]
    fn rgba_to_hex_opaque() {
        assert_eq!(rgba_to_hex(1.0, 0.0, 0.0, 1.0), "#FF0000");
        assert_eq!(rgba_to_hex(0.0, 1.0, 0.0, 1.0), "#00FF00");
        assert_eq!(rgba_to_hex(0.0, 0.0, 1.0, 1.0), "#0000FF");
    }

    #[test]
    fn rgba_to_hex_with_alpha() {
        assert_eq!(rgba_to_hex(1.0, 1.0, 1.0, 0.5), "#FFFFFF80");
    }

    #[test]
    fn rgba_to_hex_clamps() {
        assert_eq!(rgba_to_hex(2.0, -0.5, 0.0, 1.0), "#FF0000");
    }

    #[test]
    fn real_variables_resolve_colors() {
        use crate::figma::types::{Mode, Variable, VariableCollection, VariablesMeta};
        use std::collections::HashMap;

        let mut variables = HashMap::new();
        let mut values_by_mode = serde_json::Map::new();
        values_by_mode.insert(
            "mode1".to_string(),
            serde_json::json!({"r": 0.063, "g": 0.639, "b": 0.498, "a": 1.0}),
        );
        variables.insert(
            "var1".to_string(),
            Variable {
                name: "color/brand_500".to_string(),
                resolvedType: "COLOR".to_string(),
                valuesByMode: values_by_mode,
            },
        );

        let mut collections = HashMap::new();
        collections.insert(
            "coll1".to_string(),
            VariableCollection {
                name: "primitives".to_string(),
                modes: vec![Mode {
                    modeId: "mode1".to_string(),
                    name: "default".to_string(),
                }],
                variableIds: vec!["var1".to_string()],
            },
        );

        let response = VariablesResponse {
            meta: Some(VariablesMeta {
                variableCollections: collections,
                variables,
            }),
            variableCollections: vec![],
        };

        let result = figma_to_tokens(&response);
        let themes = result.get("themes").unwrap().as_object().unwrap();
        assert!(themes.contains_key("primitives"));
        let prim = themes.get("primitives").unwrap().as_object().unwrap();
        let colors = prim.get("colors").unwrap().as_object().unwrap();
        assert_eq!(
            colors.get("brand_500").unwrap().as_str().unwrap(),
            "#10A37F"
        );
    }
}
