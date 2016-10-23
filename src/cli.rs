/// Supply the command line.

use clap::{Arg, ArgMatches, App, AppSettings};


// TODO: figure out a way, eventually, to customize arguments based on whatever
// external tools are supplied---without requiring a rebuild. (Compare what
// Cargo does.)
pub fn cli<'a, 'b>(additional_args: &[Arg<'a, 'b>]) -> ArgMatches<'a> {
  App::new("SSG(rs)")
    .setting(AppSettings::ArgRequiredElseHelp)
    .version("0.1.0")
    .author(crate_authors!())
    .about("Meets my peculiar needs for generating static sites... *fast*.")
    .subcommands(vec![subcommands::generate(), subcommands::new()])
    .args(additional_args)
    .get_matches()
}


// TODO: figure out what the best way to programmatically generate new options
// for these based on the state of the file system where it's run is, so that
// e.g. the user (*me*, initially!) can e.g. just specify a location for
// new templates in the configuration file, drop a template in there, and have
// the generator pick it up correctly as one of the options.
mod subcommands {
  use clap::{Arg, App, SubCommand};

  pub fn generate<'a, 'b>() -> App<'a, 'b> {
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

  // TODO: different templates will take different args, right?
  pub fn new<'a, 'b>() -> App<'a, 'b> {
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
}
