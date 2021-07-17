//! Generate web sites from Markdown content and YAML configuration.

pub mod build;
pub mod collection;
pub mod config;
pub mod feed;
pub mod page;

pub use build::build;
