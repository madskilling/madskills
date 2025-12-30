//! Madskills CLI - tools for madskilling
#![forbid(unsafe_code)]

mod commands;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "madskills")]
#[command(about = "Tools for madskilling", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Run as if started in DIR
    #[arg(short = 'C', long, global = true)]
    chdir: Option<PathBuf>,

    /// Only print errors (suppresses warnings/info)
    #[arg(short, long, global = true)]
    quiet: bool,

    /// More detail (repeatable; e.g. -vv)
    #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Colorize output: auto|always|never
    #[arg(long, global = true, default_value = "auto")]
    color: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Lint/validate skills (spec + markdown)
    Lint(commands::lint::LintArgs),

    /// Normalize skill files (safe rewrites)
    Fmt(commands::fmt::FmtArgs),

    /// List discovered skills and metadata
    List(commands::list::ListArgs),

    /// Scaffold a new skill directory with SKILL.md
    Init(commands::init::InitArgs),
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Change directory if requested
    if let Some(ref dir) = cli.chdir {
        std::env::set_current_dir(dir)?;
    }

    // Execute command
    match cli.command {
        Commands::Lint(args) => commands::lint::cmd_lint(args, cli.quiet),
        Commands::Fmt(args) => commands::fmt::cmd_fmt(args, cli.quiet),
        Commands::List(args) => commands::list::cmd_list(args, cli.quiet),
        Commands::Init(args) => commands::init::cmd_init(args, cli.quiet),
    }
}
