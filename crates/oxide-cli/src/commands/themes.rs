//! List and show info about built-in themes

use clap::Args;

#[derive(Args)]
pub struct ThemesArgs {
    #[command(subcommand)]
    pub subcommand: Option<ThemesSubcommand>,
}

#[derive(clap::Subcommand)]
pub enum ThemesSubcommand {
    /// List all built-in themes
    List,
    /// Show details for a theme
    Info {
        /// Theme name (openai, airbnb, notion)
        #[arg(value_name = "NAME")]
        name: String,
    },
}

const THEMES: &[(&str, &str)] = &[
    ("OpenAI", "Clean, professional green accent (#10A37F)"),
    ("Airbnb", "Warm, inviting coral accent (#FF5A5F)"),
    ("Notion", "Calm, focused blue accent (#346CA3)"),
];

pub fn run(args: ThemesArgs) -> Result<(), Box<dyn std::error::Error>> {
    match args.subcommand {
        Some(ThemesSubcommand::List) | None => {
            println!("Built-in OxideUI themes:\n");
            for (name, desc) in THEMES {
                println!("  {} - {}", name, desc);
            }
        }
        Some(ThemesSubcommand::Info { name }) => {
            let name_lower = name.to_lowercase();
            let found = THEMES.iter().find(|(n, _)| n.to_lowercase() == name_lower);
            if let Some((n, d)) = found {
                println!("Theme: {}", n);
                println!("Description: {}", d);
                println!("\nUse ThemeEngine::switch_by_name(\"{}\") to activate.", n);
            } else {
                eprintln!(
                    "Unknown theme: {}. Use 'oxide themes list' to see available themes.",
                    name
                );
            }
        }
    }
    Ok(())
}
