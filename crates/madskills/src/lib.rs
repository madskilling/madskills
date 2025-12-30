//! Library interface for madskills CLI - used for documentation generation

pub mod commands;

use clap::{CommandFactory, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "madskills")]
#[command(about = "Tools for madskilling", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Run as if started in DIR
    #[arg(short = 'C', long, global = true)]
    pub chdir: Option<PathBuf>,

    /// Only print errors (suppresses warnings/info)
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// More detail (repeatable; e.g. -vv)
    #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Colorize output: auto|always|never
    #[arg(long, global = true, default_value = "auto")]
    pub color: String,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Lint/validate skills (spec + markdown)
    Lint(commands::lint::LintArgs),

    /// Normalize skill files (safe rewrites)
    Fmt(commands::fmt::FmtArgs),

    /// List discovered skills and metadata
    List(commands::list::ListArgs),

    /// Scaffold a new skill directory with SKILL.md
    Init(commands::init::InitArgs),
}

/// Returns the clap command for documentation generation
pub fn command() -> clap::Command {
    Cli::command()
}
