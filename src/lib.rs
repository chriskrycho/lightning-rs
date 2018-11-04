//! Generate web sites from Markdown content and YAML configuration.

extern crate glob;
extern crate pandoc;
extern crate quick_xml;
extern crate syntect;
extern crate yaml_rust;

mod builder;
mod config;
mod creator;
mod initializer;
mod server;
mod syntax_highlighting;
mod validated_types;
mod yaml_util;

pub use crate::builder::build;
pub use crate::creator::create;
pub use crate::initializer::init;
pub use crate::server::serve;
