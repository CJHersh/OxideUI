//! Validate tokens.json structure

use clap::Args;
use std::fs;
use std::path::PathBuf;

#[derive(Args)]
pub struct ValidateArgs {
    /// Path to tokens.json
    #[arg(short, long, default_value = "figma/tokens.json")]
    pub input: PathBuf,
}

fn validate_tokens(value: &serde_json::Value) -> Vec<String> {
    let mut errors = Vec::new();

    let themes = match value.get("themes") {
        Some(t) => t,
        None => {
            errors.push("Missing 'themes' field".to_string());
            return errors;
        }
    };

    let themes_obj = match themes.as_object() {
        Some(o) => o,
        None => {
            errors.push("'themes' must be an object".to_string());
            return errors;
        }
    };

    if themes_obj.is_empty() {
        errors.push("'themes' must have at least one theme".to_string());
    }

    let required_color_fields = [
        "surface_primary",
        "surface_secondary",
        "text_primary",
        "text_secondary",
        "interactive_default",
        "border_default",
        "feedback_success",
        "feedback_error",
    ];

    for (theme_id, theme_val) in themes_obj {
        let theme_obj = match theme_val.as_object() {
            Some(o) => o,
            None => {
                errors.push(format!("Theme '{}' must be an object", theme_id));
                continue;
            }
        };

        if !theme_obj.contains_key("name") {
            errors.push(format!("Theme '{}' missing 'name'", theme_id));
        }

        let colors = theme_obj.get("colors");
        match colors {
            Some(c) if c.is_object() => {
                let colors_obj = c.as_object().unwrap();
                for field in &required_color_fields {
                    if !colors_obj.contains_key(*field) {
                        errors.push(format!(
                            "Theme '{}' colors missing required field '{}'",
                            theme_id, field
                        ));
                    }
                }
            }
            Some(_) => errors.push(format!("Theme '{}' 'colors' must be an object", theme_id)),
            None => errors.push(format!("Theme '{}' missing 'colors'", theme_id)),
        }

        if !theme_obj.contains_key("spacing") {
            errors.push(format!("Theme '{}' missing 'spacing'", theme_id));
        }
        if !theme_obj.contains_key("radius") {
            errors.push(format!("Theme '{}' missing 'radius'", theme_id));
        }
        if !theme_obj.contains_key("typography") {
            errors.push(format!("Theme '{}' missing 'typography'", theme_id));
        }
    }

    errors
}

pub fn run(args: ValidateArgs) -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string(&args.input)?;
    let value: serde_json::Value = serde_json::from_str(&data)?;

    let errors = validate_tokens(&value);

    if errors.is_empty() {
        println!("Validation passed: {}", args.input.display());
        Ok(())
    } else {
        for e in &errors {
            eprintln!("  - {}", e);
        }
        Err(format!("Validation failed with {} error(s)", errors.len()).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_tokens() -> serde_json::Value {
        serde_json::json!({
            "themes": {
                "test": {
                    "name": "Test",
                    "colors": {
                        "surface_primary": "#FFFFFF",
                        "surface_secondary": "#F7F7F8",
                        "text_primary": "#202023",
                        "text_secondary": "#676A70",
                        "interactive_default": "#10A37F",
                        "border_default": "#EAEBEE",
                        "feedback_success": "#10A37F",
                        "feedback_error": "#E53935"
                    },
                    "spacing": {},
                    "radius": {},
                    "typography": {}
                }
            }
        })
    }

    #[test]
    fn valid_tokens_pass() {
        let errors = validate_tokens(&valid_tokens());
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn missing_themes_field() {
        let errors = validate_tokens(&serde_json::json!({}));
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("Missing 'themes'"));
    }

    #[test]
    fn themes_not_object() {
        let errors = validate_tokens(&serde_json::json!({"themes": "bad"}));
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("must be an object"));
    }

    #[test]
    fn empty_themes() {
        let errors = validate_tokens(&serde_json::json!({"themes": {}}));
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("at least one theme"));
    }

    #[test]
    fn missing_name_field() {
        let v = serde_json::json!({
            "themes": {
                "t": {
                    "colors": {
                        "surface_primary": "#FFF",
                        "surface_secondary": "#FFF",
                        "text_primary": "#000",
                        "text_secondary": "#000",
                        "interactive_default": "#000",
                        "border_default": "#000",
                        "feedback_success": "#000",
                        "feedback_error": "#000"
                    },
                    "spacing": {},
                    "radius": {},
                    "typography": {}
                }
            }
        });
        let errors = validate_tokens(&v);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("missing 'name'"));
    }

    #[test]
    fn missing_required_color_field() {
        let v = serde_json::json!({
            "themes": {
                "t": {
                    "name": "T",
                    "colors": {
                        "surface_primary": "#FFF"
                    },
                    "spacing": {},
                    "radius": {},
                    "typography": {}
                }
            }
        });
        let errors = validate_tokens(&v);
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.contains("missing required field")));
    }

    #[test]
    fn missing_spacing_radius_typography() {
        let v = serde_json::json!({
            "themes": {
                "t": {
                    "name": "T",
                    "colors": {
                        "surface_primary": "#FFF",
                        "surface_secondary": "#FFF",
                        "text_primary": "#000",
                        "text_secondary": "#000",
                        "interactive_default": "#000",
                        "border_default": "#000",
                        "feedback_success": "#000",
                        "feedback_error": "#000"
                    }
                }
            }
        });
        let errors = validate_tokens(&v);
        assert_eq!(errors.len(), 3);
        assert!(errors.iter().any(|e| e.contains("missing 'spacing'")));
        assert!(errors.iter().any(|e| e.contains("missing 'radius'")));
        assert!(errors.iter().any(|e| e.contains("missing 'typography'")));
    }
}
