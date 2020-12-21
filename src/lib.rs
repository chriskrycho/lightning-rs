//! Generate web sites from Markdown content and YAML configuration.

mod build;
mod config;
mod metadata;

pub use build::build;
