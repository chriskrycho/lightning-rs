//! Run the static site generator.

mod cli;

// Standard library
use std::io::prelude::*;

// First party
use crate::cli::{cli, Command};

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
        Command::Init { site_directory } => lightning::init(site_directory),
        Command::Build {
            site_directory,
            local: _,
        } => lightning::build(site_directory),
        Command::Create { template: _ } => lightning::create(),
        Command::Serve {} => lightning::serve(),
        Command::Run {} => unimplemented!(),
    }
}
