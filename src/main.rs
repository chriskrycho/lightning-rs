/// Run the static site generator.
#[macro_use]
extern crate clap;

mod cli;

use clap::{Arg, App};

use cli::Commands;


fn main() {
  let extra_args: Vec<Arg> = vec![];
  let subcommands: Vec<App> = vec![];
  let commands = cli::cli(&extra_args, &subcommands);

  if let Some(command_name) = commands.subcommand_name() {
    match Commands::from(command_name) {
      Commands::Generate => {},
      Commands::New => {},
      Commands::Unspecified => {/* FAIL SOMEHOW */}
    }
  }


  // TODO:
  // 0. Determine command.
  // 1. Load config.
  // 2. Execute command. So: make functions to dispatch for commands!
  //     - generate:
  //         - get list of docs in source directory (from config)
  //         -
}
