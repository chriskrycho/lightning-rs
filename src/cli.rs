//! Supply the command line.

// Standard library
use std::env;
use std::fmt;
use std::path::PathBuf;

// Third party
use clap::{App, ArgMatches};

const INIT: &str = "init";
const BUILD: &str = "build";
const CREATE: &str = "create";
const SERVE: &str = "serve";

/// Commands which can be called, mapped from strings of the same name.
pub enum Command {
    /// Create a new site.
    Init {
        site: PathBuf,
    },
    /// Generate the site at `site`.
    Build {
        site: PathBuf,
    },
    Create,
    Serve,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::Init { ref site } => write!(f, "{} {}", INIT, site.to_string_lossy()),
            Command::Build { ref site } => write!(f, "{} {}", BUILD, site.to_string_lossy()),
            Command::Create => write!(f, "{}", CREATE),
            Command::Serve => write!(f, "{}", SERVE),
        }
    }
}

// TODO: figure out a way, eventually, to customize arguments based on whatever
//       external tools are supplied---without requiring a rebuild. (Compare
//       what Cargo does.)
/// Get arguments from the command line.
pub fn cli() -> Command {
    use self::Command::*;

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Since a subcommand is required, if this fails it's clap's (or, more
    // likely, our *configuration* of clap's) fault. In any case... `unwrap()`
    // at will, commander!
    match matches.subcommand_name().unwrap() {
        INIT => Init {
            site: site_directory(matches.subcommand_matches(INIT).unwrap()),
        },
        BUILD => Build {
            site: site_directory(matches.subcommand_matches(BUILD).unwrap()),
        },
        CREATE => Create,
        SERVE => Serve,
        _ => panic!("ERROR: `clap.rs` is configured wrong somehow, kids."),
    }
}

fn site_directory(matches: &ArgMatches) -> PathBuf {
    match matches.value_of("site_directory") {
        Some(path_str) => PathBuf::from(path_str),
        None => env::current_dir().unwrap(),
    }
}
