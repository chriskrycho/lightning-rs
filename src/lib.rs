//! Generate web sites from Markdown content and TOML configuration.

extern crate glob;
extern crate pandoc;
extern crate quick_xml;
extern crate syntect;
extern crate yaml_rust;

mod initializer;
mod config;
mod creator;
mod builder;
mod server;
mod syntax_highlighting;

pub use initializer::init;
pub use builder::build;
pub use creator::create;
pub use server::serve;
