/// Supply the command line.

use clap::{Arg, ArgMatches, App, AppSettings, SubCommand};


pub fn cli<'a, 'b>(additional_args: &[Arg<'a, 'b>]) -> ArgMatches<'a> {
  App::new("SSG(rs)")
    .setting(AppSettings::ArgRequiredElseHelp)
    .version("0.1.0")
    .author(crate_authors!())
    .about("Meets my peculiar needs for generating static sites... *fast*.")

    .subcommand(SubCommand::with_name("generate")
      .about("Generate the site")
      .arg(Arg::with_name("local")
        .help("Use local paths to resources")
        .long("local"))
      .arg(Arg::with_name("serve")
        .short("s")
        .long("serve")
        .alias("server")))

    // TODO: figure out a way, eventually, to customize arguments.
    //
    // I can probably just take a list of `Arg`s to `cli()`, and then do
    // `.args()` with the list.
    .subcommand(SubCommand::with_name("new")
      .about("Create an item from a template")
      .arg(Arg::with_name("template")
        .help("The name of the template to generate, e.g. `post`")
        .index(1)
        .required(true)
        .possible_values(&["post"])
      )
      .arg(Arg::with_name("title")
        .help("The title to use in the template")
        .index(2)
        .required(true)
        .takes_value(true)
      )
    )
    .args(additional_args)
    .get_matches()
}
