//! Supply the command line.

// Standard library
use std::env;
use std::fmt;
use std::path::{PathBuf};

// Third party
use clap::{App, ArgMatches};

// First party
use lightning::Site;


const INIT: &'static str = "init";
const GENERATE: &'static str = "generate";
const CREATE: &'static str = "create";
const SERVE: &'static str = "serve";


/// Commands which can be called, mapped from strings of the same name.
pub enum Command {
    /// Generate the site
    Generate(Site),
    /// Create a new site.
    Init,
    Create,
    Serve,
    Unspecified,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::Generate(_) => write!(f, "{}", GENERATE),
            Command::Init => write!(f, "{}", INIT),
            Command::Create => write!(f, "{}", CREATE),
            Command::Serve => write!(f, "{}", SERVE),
            _ => write!(f, "error!!!"),  // TODO: something else!
        }
    }
}


// TODO: figure out a way, eventually, to customize arguments based on whatever
//       external tools are supplied---without requiring a rebuild. (Compare
//       what Cargo does.)
/// Get arguments from the command line.
pub fn cli() -> Command  {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Since a subcommand is required, if this fails it's clap's fault.
    // `unwrap()` at will, commander!
    match matches.subcommand_name().unwrap() {
        INIT => Command::Init,
        GENERATE => generate_from_matches(matches.subcommand_matches(GENERATE).unwrap()),
        CREATE => Command::Create,
        SERVE => Command::Serve,
        _ => Command::Unspecified,
    }
}

fn generate_from_matches<'m>(matches: &'m ArgMatches) -> Command {
    Command::Generate(Site {
        source_directory: match matches.value_of("site_directory") {
            Some(path_str) => PathBuf::from(path_str),
            None => env::current_dir().unwrap(),
        },
        template_directory: match matches.value_of("template_directory") {
            Some(path_str) => Some(PathBuf::from(path_str)),
            None => None,
        }
    })
}
