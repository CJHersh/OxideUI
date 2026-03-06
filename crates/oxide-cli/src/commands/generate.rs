//! Generate Rust theme code from tokens.json

use clap::Args;
use std::fs;
use std::path::PathBuf;

use crate::codegen::rust::RustCodegen;

#[derive(Args)]
pub struct GenerateArgs {
    /// Input tokens.json path
    #[arg(short, long, default_value = "figma/tokens.json")]
    pub input: PathBuf,

    /// Output .rs file path
    #[arg(short, long, default_value = "src/theme_generated.rs")]
    pub output: PathBuf,

    /// Module name for the generated code
    #[arg(long, default_value = "theme_generated")]
    pub module: String,
}

pub fn run(args: GenerateArgs) -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string(&args.input)?;
    let tokens: serde_json::Value = serde_json::from_str(&data)?;

    let code = RustCodegen::generate(&tokens)?;

    if let Some(parent) = args.output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&args.output, code)?;

    println!("Generated theme code at {}", args.output.display());
    Ok(())
}
