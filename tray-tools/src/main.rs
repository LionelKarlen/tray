use clap::Parser;
use miette::Result;
use tray_tools::commands::{Command, Executable};

/// Tray toolkit
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Command to execute
    #[command(subcommand)]
    command: Command,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.command.execute()?;
    Ok(())
}
