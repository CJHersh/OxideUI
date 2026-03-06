//! Oxide CLI - Figma integration and code generation for OxideUI

use clap::{Parser, Subcommand};

mod codegen;
mod commands;
mod figma;
mod parser;

use commands::{config, generate, init, new_project, sync, themes, validate, watch};

#[derive(Parser)]
#[command(name = "oxide")]
#[command(about = "CLI tool for OxideUI Figma integration and code generation", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize OxideUI in the current project
    Init(init::InitArgs),

    /// Manage configuration (Figma token, etc.)
    Config(config::ConfigArgs),

    /// Sync variables from Figma to tokens.json
    Sync(sync::SyncArgs),

    /// Generate Rust theme code from tokens.json
    Generate(generate::GenerateArgs),

    /// Watch Figma for changes and auto-sync
    Watch(watch::WatchArgs),

    /// Validate tokens.json structure
    Validate(validate::ValidateArgs),

    /// List or show info about built-in themes
    Themes(themes::ThemesArgs),

    /// Create a new project from template
    New(new_project::NewProjectArgs),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => init::run(args)?,
        Commands::Config(args) => config::run(args)?,
        Commands::Sync(args) => sync::run(args).await?,
        Commands::Generate(args) => generate::run(args)?,
        Commands::Watch(args) => watch::run(args).await?,
        Commands::Validate(args) => validate::run(args)?,
        Commands::Themes(args) => themes::run(args)?,
        Commands::New(args) => new_project::run(args)?,
    }

    Ok(())
}
