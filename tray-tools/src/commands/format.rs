use std::{
    fs::{self},
    io::{self, Read, Write},
    path::PathBuf,
};

use clap::Args;
use miette::{IntoDiagnostic, Result};
use tray_core::{
    bridge::{FromKDL, ToKDL},
    nodes::document::Document,
};

use crate::commands::Executable;

#[derive(Args, Debug)]
#[command(group(
    clap::ArgGroup::new("input")
        .args(["file", "stdio"])
        .multiple(false)
        .required(true)
))]
pub struct FormatArgs {
    /// File to format. Writes in place.
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,
    /// Read File from stdin and write to stdout.
    #[arg(long)]
    stdio: bool,
}

impl Executable for FormatArgs {
    fn execute(&self) -> Result<()> {
        let mut input = String::new();

        if self.stdio {
            io::stdin().read_to_string(&mut input).into_diagnostic()?;
        } else if let Some(ref path) = self.file {
            input = fs::read_to_string(path).into_diagnostic()?;
        }

        let doc = Document::from_kdl(&input).into_diagnostic()?;
        let out = doc.to_kdl().into_diagnostic()?;

        if self.stdio {
            write!(io::stdout(), "{out}").into_diagnostic()?;
        } else if let Some(ref path) = self.file {
            fs::write(path, out).into_diagnostic()?;
        }
        Ok(())
    }
}
