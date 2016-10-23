/// Run the static site generator.
#[macro_use]
extern crate clap;

mod cli;


fn main() {
  let matches = cli::cli(&[]);
}
