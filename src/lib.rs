//! Generate web sites from Markdown content and TOML configuration.

extern crate glob;
extern crate pandoc;
extern crate quick_xml;
extern crate rayon;
extern crate syntect;

pub mod generator;
pub mod syntax_highlighting;

pub use generator::generate;
