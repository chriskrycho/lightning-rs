//! Supply the command line.

// Standard library
use std::path::PathBuf;

// Third party
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Lightning (lx)",
    about = "A fast, reliable, configurable static site generator",
    raw(setting = "structopt::clap::AppSettings::ArgRequiredElseHelp"),
    raw(setting = "structopt::clap::AppSettings::ColoredHelp")
)]
pub enum Command {
    /// Create a new Lightning site.
    #[structopt(name = "init")]
    Init {
        /// The folder to create the site in. If no argument is supplied, the
        /// current directory will be used instead.
        #[structopt(short = "p", long = "path", parse(from_os_str))]
        site_directory: PathBuf,
    },

    /// Build the site.
    #[structopt(name = "build")]
    Build {
        /// The root of the site (if different from the current directory).
        #[structopt(short = "p", long = "path", parse(from_os_str))]
        site_directory: PathBuf,

        /// Use local paths to resources.
        #[structopt(short = "l")]
        local: bool,
    },

    /// Create an item from a template.
    #[structopt(name = "create")]
    Create {
        /// The name of the template to generate, e.g. 'post'. Must be the stem
        /// of a file in your `<site>/templates` directory.
        template: Vec<String>,
    },

    /// Serve the site.
    #[structopt(name = "serve")]
    Serve {
        // TODO: make this basically the same as `Build`, but with reload.
    },

    /// Run the UI for the site.
    #[structopt(name = "run")]
    Run {
        // TODO: this is super aspirational.
    },
}

// TODO: figure out a way, eventually, to customize arguments based on whatever
//       external tools are supplied---without requiring a rebuild. (Compare
//       what Cargo does.)
/// Get arguments from the command line.
pub fn cli() -> Command {
    Command::from_args()
}
