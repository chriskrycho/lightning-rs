//! Generate web sites from Markdown content and TOML configuration.

extern crate glob;
extern crate pandoc;
extern crate quick_xml;
extern crate syntect;

mod creator;
mod generator;
mod server;
mod syntax_highlighting;

pub use creator::create;
pub use generator::{generate, Site};
pub use server::serve;
