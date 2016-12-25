//! Utilities for dealing with common YAML-processing tasks.

// Standard library
use std::fmt;

// Third party
use yaml_rust::yaml;

#[derive(Debug)]
pub enum Required {
    Yes,
    No,
}

impl fmt::Display for Required {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &Required::Yes => write!(f, "Required"),
            &Required::No => write!(f, "Optional"),
        }
    }
}

pub fn required_key(key: &str, yaml: &yaml::Hash) -> String {
    format!("Required key `{}` missing from {:?}", key, yaml)
}

pub fn key_of_type(key: &str,
                   required: Required,
                   yaml: &yaml::Hash,
                   required_type: &str)
                   -> String {
    format!("{} key `{}` in {:?} must be a {}",
            required,
            key,
            yaml,
            required_type)
}
