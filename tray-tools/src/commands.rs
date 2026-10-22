use clap::Subcommand;
use miette::Result;
pub mod format;

#[derive(Subcommand, Debug)]
#[command()]
pub enum Command {
    /// Format Tray kdl file
    #[command(visible_alias = "fmt")]
    Format(format::FormatArgs),
}

pub trait Executable {
    fn execute(&self) -> Result<()>;
}

impl Executable for Command {
    fn execute(&self) -> Result<()> {
        match self {
            Command::Format(format_args) => format_args.execute(),
        }
    }
}
