/// Run the static site generator.

#[macro_use]
extern crate clap;
extern crate glob;
extern crate pandoc;  // TODO: extract to `lib.rs`.
extern crate lightning;

mod cli;

// Standard library
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

// Third party
use clap::{Arg, App};
use glob::glob;
use pandoc::{Pandoc, PandocOption, InputFormat, OutputFormat, OutputKind};

// First party
use cli::{cli, Commands};
use lightning::syntax_highlighting::syntax_highlight;


fn main() {
  let extra_args: Vec<Arg> = vec![];
  let subcommands: Vec<App> = vec![];
  let commands = cli(&extra_args, &subcommands);

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

  // In the vein of "MVP": let's start by just loading all the files. We'll
  // extract this all into standalone functions as necessary later.

  // TODO: load this from the configuration file.
  let directory = Path::new("tests/data");

  // TODO: instead of unwrapping the directory and the glob result, we'll
  //   actually check both.
  let dir_str = format!("{}/**/*.md", directory.to_str().unwrap());
  let markdown_files = glob(&dir_str).unwrap();

  // TODO: we'll repeat this process on *all* of them instead of just one.
  //   Eventually we'll do that iteration with `rayon::par_iter::for_each()`.
  for file in markdown_files {  // -> Option<Path>
    // TODO: extract this into a nice function to call in a for loop/foreach.
    // Need to make item live long enough after unwrapping.
    let file = file.unwrap();
    let file = file.to_str().unwrap();

    let mut pandoc = Pandoc::new();
    pandoc.set_input_format(InputFormat::Markdown)
      .set_output_format(OutputFormat::Html5)
      .add_options(&[PandocOption::Smart, PandocOption::NoHighlight])
      .add_input(file)
      .set_output(OutputKind::Pipe);

    // TODO: don't panic, return a Result. That can then be a `?`.
    let output = match pandoc.execute_with_output() {
      Ok(output) => output,
      Err(err) => panic!("Failed pandoc-ing {}:\n{:?}", file, err),
    };

    // TODO: syntect (#1)
    let highlighted = syntax_highlight(output);

    // TODO: extract this as part of the writing it out process.
    let ff_path = Path::new(file);
    let dest = Path::new("./tests/output")
      .join(ff_path.file_name().unwrap())
      .with_extension("html");

    let mut fd = match OpenOptions::new()
      .write(true)
      .create(true)
      .open(dest.clone()) {

      Ok(fd) => fd,
      Err(why) => {
        panic!("Could not open {} for write: {}",
               dest.to_string_lossy(),
               why)
      }
    };

    match write!(fd, "{}", highlighted) {
      Ok(_) => println!("wrote {}", dest.to_string_lossy()),
      Err(why) => panic!("failed to write: {}", why),
    }
  }
}
