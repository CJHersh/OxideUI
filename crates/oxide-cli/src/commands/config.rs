//! Configuration management for Figma token and settings

use clap::Args;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub subcommand: ConfigSubcommand,
}

#[derive(clap::Subcommand)]
pub enum ConfigSubcommand {
    /// Set Figma personal access token
    Set {
        /// Figma personal access token
        #[arg(value_name = "TOKEN")]
        token: String,
    },
    /// Show current config (token masked)
    Show,
    /// Clear stored config
    Clear,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub figma_token: Option<String>,
}

fn config_path() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home)
        .join(".config")
        .join("oxide-ui")
        .join("config.json")
}

pub fn load_config() -> Config {
    let path = config_path();
    if path.exists() {
        if let Ok(data) = fs::read_to_string(&path) {
            if let Ok(cfg) = serde_json::from_str(&data) {
                return cfg;
            }
        }
    }
    Config::default()
}

fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, serde_json::to_string_pretty(config)?)?;
    Ok(())
}

pub fn run(args: ConfigArgs) -> Result<(), Box<dyn std::error::Error>> {
    match args.subcommand {
        ConfigSubcommand::Set { token } => {
            let mut config = load_config();
            config.figma_token = Some(token);
            save_config(&config)?;
            println!("Figma token saved to {}", config_path().display());
        }
        ConfigSubcommand::Show => {
            let config = load_config();
            if let Some(t) = &config.figma_token {
                let masked = if t.len() > 8 {
                    format!("{}...{}", &t[..4], &t[t.len() - 4..])
                } else {
                    "****".to_string()
                };
                println!("figma_token: {}", masked);
            } else {
                println!("No config set. Use 'oxide config set <TOKEN>' to add Figma token.");
            }
        }
        ConfigSubcommand::Clear => {
            let path = config_path();
            if path.exists() {
                fs::remove_file(path)?;
                println!("Config cleared.");
            } else {
                println!("No config file found.");
            }
        }
    }
    Ok(())
}
