//! Run the static site generator.
#[macro_use]
extern crate clap;
extern crate lightning;

mod cli;

// Standard library

// First party
use cli::{cli, Command};

fn main() {
    if let Err(reason) = run() {
        // if this fails, we literally can't do a thing except panic.
        eprint!("failure: {}", reason);
        std::process::exit(1);
    }
}

/// Define a `Result`-returning function to run the app.
///
/// (This is a standard Rust pattern to support the use of `try~`/`?`. We're
/// not doing that yet, but I expect we might eventually; this is convenient.)
fn run() -> Result<(), String> {
    match cli() {
        Command::Init { site } => lightning::init(site),
        Command::Build { site } => lightning::build(site),
        Command::Create => lightning::create(),
        Command::Serve => lightning::serve(),
    }
}
