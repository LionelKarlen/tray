use clap::Parser;
use clap::Subcommand;

/// Tray toolkit
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Command to execute
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
#[command()]
enum Command {
    /// Format Tray kdl file
    #[command(visible_alias = "fmt")]
    Format,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Format => todo!(),
    }
}
