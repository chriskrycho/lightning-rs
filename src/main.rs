//! Run the static site generator.
#[macro_use]
extern crate clap;
extern crate lightning;

mod cli;

// Standard library
use std::io::prelude::*;

// Third party
use clap::{Arg, App};

// First party
use cli::{cli, Command};
use lightning::generate;


fn main() {
    if let Err(reason) = run() {
        // if this fails, we literally can't do a thing except panic.
        write!(std::io::stderr(), "failure: {}", reason).unwrap();
        std::process::exit(1);
    }
}


/// Define a `Result`-returning function to run the app.
///
/// (This is a standard Rust pattern to support the use of `try~`/`?`.)
fn run() -> Result<(), String> {
    let extra_args: Vec<Arg> = vec![];
    let sub_commands: Vec<App> = vec![];
    let args = cli(&extra_args, &sub_commands)?;

    match args.sub_command {
        Command::Generate => { lightning::generate() }
        Command::New => { new() }
        Command::Unspecified => {
            Err(format!("Failed to parse command line."))
        }
    }
}


fn new() -> Result<(), String> {
    unimplemented!()
}
