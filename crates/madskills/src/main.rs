//! Madskills CLI - tools for madskilling
#![forbid(unsafe_code)]

use clap::Parser;
use madskills::{Cli, Commands, commands};

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
