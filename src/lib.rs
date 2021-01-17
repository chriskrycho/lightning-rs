//! Generate web sites from Markdown content and YAML configuration.

mod build;
pub mod config;
pub mod page;

pub use build::build;
