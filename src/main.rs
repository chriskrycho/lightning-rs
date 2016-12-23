//! Run the static site generator.
#[macro_use]
extern crate clap;
extern crate lightning;

mod cli;

// Standard library
use std::io::prelude::*;

// First party
use cli::{cli, Command};


fn main() {
    if let Err(reason) = run() {
        // if this fails, we literally can't do a thing except panic.
        write!(std::io::stderr(), "failure: {}", reason).unwrap();
        std::process::exit(1);
    }
}


/// Define a `Result`-returning function to run the app.
///
/// (This is a standard Rust pattern to support the use of `try~`/`?`. We're
/// not doing that yet, but I expect we might eventually; this is convenient.)
fn run() -> Result<(), String> {
    match cli() {
        Command::Generate(site) => lightning::generate(site),
        Command::Create => lightning::create(),
        Command::Serve => lightning::serve(),
        Command::Unspecified => Err(format!("Failed to parse command line.")),
    }
}
