/// Run the static site generator.
#[macro_use]
extern crate clap;

use clap::{Arg, ArgMatches, App, AppSettings, SubCommand};


fn cli<'a>() -> ArgMatches<'a> {
  App::new("SSG(rs)")
    .setting(AppSettings::ArgRequiredElseHelp)
    .version("0.1.0")
    .author(crate_authors!())
    .about("Meets my peculiar needs for generating static sites... *fast*.")
    .subcommand(SubCommand::with_name("generate")
      .about("Generate the site")
      .arg(Arg::with_name("local")
        .long("--local")
        .help("Use local paths to resources")))
    .get_matches()
}


fn main() {
  let matches = cli();
}
