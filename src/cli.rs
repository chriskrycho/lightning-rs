//! Supply the command line.

// Standard library
use std::env;
use std::fmt;
use std::path::{PathBuf};

// Third party
use clap::{Arg, App, AppSettings, SubCommand};

// First party
use lightning::Site;


const GENERATE: &'static str = "generate";
const CREATE: &'static str = "create";
const SERVE: &'static str = "serve";


/// Commands which can be called, mapped from strings of the same name.
pub enum Command {
    /// Generate the site
    Generate(Site),
    Create,
    Serve,
    Unspecified,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::Generate(_) => write!(f, "{}", GENERATE),
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
pub fn cli<'a, 'b>(additional_args: &[Arg<'a, 'b>],
                   additional_subcommands: &[App<'a, 'b>])
                   -> Command {

    let mut args = vec![];
    args.extend_from_slice(additional_args);

    let mut subcommands = vec![
        generate(),
        new(),
    ];

    subcommands.extend_from_slice(additional_subcommands);

    let matches = App::new("Lightning")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.1.0")
        .author(crate_authors!())
        .about("A fast, reliable, configurable static site generator.")
        .subcommands(subcommands)
        .args(&args)
        .get_matches();

    // Since a subcommand is required, if this fails it's clap's fault.
    // `unwrap()` at will, commander!
    match matches.subcommand_name().unwrap() {
        GENERATE => {
            let generate_args = matches.subcommand_matches(GENERATE).unwrap();
            Command::Generate(Site {
                source_directory: match generate_args.value_of("site_directory") {
                    Some(path_str) => PathBuf::from(path_str),
                    None => env::current_dir().unwrap(),
                },
                template_directory: match generate_args.value_of("template_directory") {
                    Some(path_str) => Some(PathBuf::from(path_str)),
                    None => None,
                }
            })
        },
        CREATE => Command::Create,
        SERVE => Command::Serve,
        _ => Command::Unspecified,

    }
}


/// Generate the site.
fn generate<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(GENERATE)
        .about("Generate the site")
        .arg(Arg::with_name("site_directory")
            .help("the root of the site (if different from the current directory)")
            .takes_value(true)
            .visible_alias("site"))
        .arg(Arg::with_name("template_directory")
            .help("the template directory, if not `{site_directory}/layout`")
            .takes_value(true)
            .visible_alias("templates"))
        .arg(Arg::with_name("local")
            .help("Use local paths to resources")
            .long("local"))
        .arg(Arg::with_name("watch")
            .short("w")
            .long("watch"))
}


/// Generate a new item from a template.
fn new<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CREATE)
        .about("Create an item from a template")
        .arg(Arg::with_name("template")
            .help("The name of the template to generate, e.g. `post`")
            .index(1)
            .required(true)
            .possible_values(&["post"]))
        .arg(Arg::with_name("title")
            .help("The title to use in the template")
            .index(2)
            .required(true)
            .takes_value(true))
}
