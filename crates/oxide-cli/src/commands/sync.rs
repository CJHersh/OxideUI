//! Sync variables from Figma to tokens.json

use clap::Args;
use std::path::PathBuf;

use crate::commands::config;
use crate::figma::client::FigmaClient;
use crate::parser::tokens::figma_to_tokens;

#[derive(Args, Clone)]
pub struct SyncArgs {
    /// Figma file key (from URL). Can also set via FIGMA_FILE_KEY env var.
    #[arg(short, long)]
    pub file_key: Option<String>,

    /// Output path for tokens.json
    #[arg(short, long, default_value = "figma/tokens.json")]
    pub output: PathBuf,

    /// Use local tokens.json as fallback if API fails
    #[arg(long, default_value = "true")]
    pub fallback: bool,
}

pub async fn run(args: SyncArgs) -> Result<(), Box<dyn std::error::Error>> {
    let file_key: String = args
        .file_key
        .or_else(|| std::env::var("FIGMA_FILE_KEY").ok())
        .ok_or("Missing --file-key. Set FIGMA_FILE_KEY or pass --file-key")?;

    let cfg = config::load_config();
    let token = cfg
        .figma_token
        .as_deref()
        .ok_or("No Figma token. Run 'oxide config set <TOKEN>' first.")?;

    let client = FigmaClient::new(token);
    match client.get_local_variables(&file_key).await {
        Ok(variables) => {
            let tokens_value = figma_to_tokens(&variables);
            if let Some(parent) = args.output.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&args.output, serde_json::to_string_pretty(&tokens_value)?)?;
            println!("Synced to {}", args.output.display());
        }
        Err(e) => {
            if args.fallback && args.output.exists() {
                eprintln!(
                    "Warning: Figma API failed ({}), using existing {}",
                    e,
                    args.output.display()
                );
            } else {
                return Err(e.into());
            }
        }
    }

    Ok(())
}
