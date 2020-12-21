//! Generate web sites from Markdown content and YAML configuration.

mod build;
mod config;
mod page;

pub use build::build;
