//! Supply the command line.

// Standard library
use std::path::PathBuf;

// Third party
use clap::Clap;

#[derive(Clap)]
#[clap(
    name = "Lightning (lx)",
    about = "A very fast, very opinionated static site generator",
    version = "1.0",
    author = "Chris Krycho <hello@@chriskrycho.com>",
    setting = clap::AppSettings::ArgRequiredElseHelp,
    global_setting = clap::AppSettings::ColoredHelp,
    global_setting = clap::AppSettings::ColorAuto
)]
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
