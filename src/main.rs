/// Run the static site generator.
#[macro_use]
extern crate clap;
extern crate glob;
extern crate pandoc;
extern crate syntect;

mod cli;

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use clap::{Arg, App};
use glob::glob;
use pandoc::{Pandoc,PandocOption,InputFormat,OutputFormat,OutputKind};
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
  //   0. Determine command.
  //   1. Load config.
  //   2. Execute command. So: make functions to dispatch for commands!
  //       - generate:
  //           - get list of docs in source directory (from config)
  //           - do the below!
  //       - new
  //           - get list of templates in source directory (from config)
  //           - match to template specified or default (from config)
  //           - generate the new file

  // In the vein of "MVP": let's start by just loading all the files. We'll
  // extract this all into standalone functions as necessary later.

  // TODO: load this from the configuration file.
  let directory = Path::new("tests/data");

  // TODO: instead of unwrapping the directory and the glob result, we'll
  //   actually check both.
  let dir_str = format!("{}/**/*.md", directory.to_str().unwrap());
  let mut markdown_files = glob(&dir_str).unwrap();

  // TODO: we'll repeat this process on *all* of them instead of just one.
  //   Eventually we'll do that iteration with `rayon::par_iter::for_each()`.
  if let Some(first_file) = markdown_files.next() /* -> Option<Path> */ {
    // TODO: extract this into a nice function to call in a for loop/foreach.
    // Need to make item live long enough after unwrapping.
    let first_file = first_file.unwrap();
    let first_file = first_file.to_str().unwrap();

    let mut pandoc = Pandoc::new();
    pandoc.set_input_format(InputFormat::Markdown)
      .set_output_format(OutputFormat::Html5)
      .add_options(&[
        PandocOption::Smart,
        PandocOption::NoHighlight,
      ])
      .add_input(first_file)
      .set_output(OutputKind::Pipe);

    // TODO: don't panic, return a Result. That can then be a `try!` and
    //   eventually even a `?`.
    let output = match pandoc.execute_with_output() {
      Ok(output) => output,
      Err(err) => panic!("Failed pandoc-ing {}:\n{:?}", first_file, err),
    };

    // TODO: extract this as part of the writing it out process.
    let ff_path = Path::new(first_file);
    let dest = Path::new("/Users/chris/Desktop")
      .join(ff_path.file_name().unwrap())
      .with_extension("html");

    // TODO: syntect here for code snippets in the file!
    let mut fd = match OpenOptions::new()
        .write(true)
        .create(true)
        .open(dest.clone()) {

      Ok(fd) => fd,
      Err(why) =>
        panic!("Could not open {} for write: {}", dest.to_string_lossy(), why),
    };

    match write!(fd, "{}", output) {
      Ok(_) => println!("BOOM."),
      Err(why) => panic!("... the other kind of BOOM. Alas.\n{}", why),
    }
  }
}
