/// Run the static site generator.
#[macro_use]
extern crate clap;
extern crate glob;
extern crate syntect;

mod cli;

use std::path::Path;

use clap::{Arg, App};
use glob::glob;
use syntect::easy::HighlightLines;

use cli::Commands;


fn main() {
  let extra_args: Vec<Arg> = vec![];
  let subcommands: Vec<App> = vec![];
  let commands = cli::cli(&extra_args, &subcommands);

  // TODO: actually use those matches.
  if let Some(command_name) = commands.subcommand_name() {
    match Commands::from(command_name) {
      Commands::Generate => {}
      Commands::New => {}
      Commands::Unspecified => {
        // FAIL SOMEHOW
      }
    }
  }

  // TODO:
  // 0. Determine command.
  // 1. Load config.
  // 2. Execute command. So: make functions to dispatch for commands!
  //     - generate:
  //         - get list of docs in source directory (from config)
  //         -

  // In the vein of "MVP": let's start by just loading all the files. We'll
  // extract this all into standalone functions as necessary later.

  // TODO: load this from the configuration file.
  let directory = Path::new("/Users/chris/Sites/chriskrycho.com/current/content");

  // TODO: instead of unwrapping the directory and the glob result, we'll
  // actually check both.
  let dir_str = format!("{}/**/*.md", directory.to_str().unwrap());
  let markdown_files = glob(&dir_str).unwrap();

  // TODO: we'll repeat this process on *all* of them instead of just one.
  if let Some(first_file) = markdown_files.peekable().next() {
    // TODO:
    // pandoc::string_from_file(
    //   first_file,
    //   [ PandocOption::To(OutputFormat::html5)
    //   , PandocOption::Smart
    //   , PandocOption::NoHighlight
    //   ])
  }
}
