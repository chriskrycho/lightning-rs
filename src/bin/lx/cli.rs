//! Supply the command line.

// Standard library
use std::path::PathBuf;

// Third party
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = "Lightning (lx)",
    about = "A very fast, very opinionated static site generator",
    version = "1.0",
    author = "Chris Krycho <hello@@chriskrycho.com>"
)]
#[command(arg_required_else_help(true))]
pub(crate) enum Command {
    /// Build the site.
    #[clap(name = "build")]
    Build {
        /// The root of the site (if different from the current directory).
        site_directory: Option<PathBuf>,
    },
}

impl Command {
    pub(crate) fn cli() -> Self {
        Self::parse()
    }
}
