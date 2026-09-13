use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    version,
    about = "Create and validate portable Markdown article packages"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Allocate an unfinished draft without overwriting existing content.
    Create {
        #[arg(long)]
        root: PathBuf,
        #[arg(long)]
        slug: String,
        #[arg(long)]
        title: String,
        #[arg(long, default_value = "en")]
        language: String,
    },
    /// Check package structure and local resources without modifying files.
    Validate { folder: PathBuf },
    /// Install the three named agents, preserving differing existing files.
    SetupAgents {
        #[arg(long)]
        project: PathBuf,
    },
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    match Cli::parse().command {
        Command::Create {
            root,
            slug,
            title,
            language,
        } => {
            println!(
                "{}",
                codex_blogger::create(&root, &slug, &title, &language)?.display()
            );
        }
        Command::Validate { folder } => {
            let errors = codex_blogger::validate(&folder)?;
            if !errors.is_empty() {
                return Err(errors.join("\n").into());
            }
            println!("Article package checks passed.");
        }
        Command::SetupAgents { project } => {
            println!("{}", codex_blogger::setup_agents(&project)?.display());
        }
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}
