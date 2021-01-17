//! Run the static site generator.

mod cli;

use crate::cli::Command;

fn main() -> Result<(), String> {
    let cwd = std::env::current_dir()
        .expect("Something is suuuuper borked: I cannot even get the current working directory!");

    match Command::cli() {
        Command::Build { site_directory } => lightning::build(site_directory.unwrap_or(cwd)),
    }
}
