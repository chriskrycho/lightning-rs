//! Generate web sites from Markdown content and TOML configuration.

extern crate glob;
extern crate pandoc;
extern crate quick_xml;
extern crate syntect;
extern crate yaml_rust;

mod init;
mod creator;
mod generator;
mod server;
mod syntax_highlighting;

pub use init::init;
pub use creator::create;
pub use generator::{generate, Site};
pub use server::serve;
