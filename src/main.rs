/// Run the static site generator.
#[macro_use]
extern crate clap;

mod cli;

use clap::{Arg, App};


fn main() {
  let args: Vec<Arg> = vec![];
  let commands: Vec<App> = vec![];
  let matches = cli::cli(&args, &commands);
}
