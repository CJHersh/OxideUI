//! Watch Figma for changes and auto-sync

use clap::Args;
use std::path::PathBuf;
use std::time::Duration;

use crate::commands::sync;

#[derive(Args)]
pub struct WatchArgs {
    /// Figma file key. Can also set via FIGMA_FILE_KEY env var.
    #[arg(short, long)]
    pub file_key: Option<String>,

    /// Poll interval in seconds
    #[arg(short, long, default_value = "30")]
    pub interval: u64,

    /// Output path for tokens.json
    #[arg(short, long, default_value = "figma/tokens.json")]
    pub output: PathBuf,
}

pub async fn run(args: WatchArgs) -> Result<(), Box<dyn std::error::Error>> {
    let file_key: String = args
        .file_key
        .or_else(|| std::env::var("FIGMA_FILE_KEY").ok())
        .ok_or("Missing --file-key. Set FIGMA_FILE_KEY or pass --file-key")?;

    println!("Watching Figma file {} every {}s", file_key, args.interval);
    println!("Press Ctrl+C to stop");

    let sync_args = sync::SyncArgs {
        file_key: Some(file_key.clone()),
        output: args.output.clone(),
        fallback: true,
    };

    loop {
        if let Err(e) = sync::run(sync_args.clone()).await {
            eprintln!("Sync error: {}", e);
        }
        tokio::time::sleep(Duration::from_secs(args.interval)).await;
    }
}
