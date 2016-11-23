/// Supply the command line.

use std::fmt;

use clap::{Arg, App, AppSettings, SubCommand};

// TODO: define this as the thing that `cli` actually returns (instead of
// TODO: `ArgMatches`.
pub struct Args {
    pub sub_command: Command
}


// TODO: figure out a way, eventually, to customize arguments based on whatever
//       external tools are supplied---without requiring a rebuild. (Compare
//       what Cargo does.)
/// Get arguments from the command line.
pub fn cli<'a, 'b>(additional_args: &[Arg<'a, 'b>],
                   additional_subcommands: &[App<'a, 'b>])
                   -> Result<Args, String> {

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
    let sub_command_name = matches.subcommand_name().unwrap();

    Ok(Args {
        sub_command: Command::from(sub_command_name)
    })
}


pub enum Command {
    Generate,
    New,
    Unspecified,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::Generate => write!(f, "generate"),
            Command::New => write!(f, "new"),
            _ => write!(f, "error!!!"),  // TODO: something else!
        }
    }
}

impl<'a> From<&'a str> for Command {
    fn from(s: &str) -> Command {
        match s {
            "generate" => Command::Generate,
            "new" => Command::New,
            _ => Command::Unspecified,
        }
    }
}


/// Generate the site.
fn generate<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("generate")
               .about("Generate the site")
               .arg(Arg::with_name("local")
                        .help("Use local paths to resources")
                        .long("local"))
               .arg(Arg::with_name("serve")
                        .short("s")
                        .long("serve")
                        .alias("server"))
}


/// Generate a new item from a template.
fn new<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("new")
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
