//! Utilities for dealing with common YAML-processing tasks.

// Standard library
use std::fmt;
use std::ops;
use std::u8;

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

pub fn required_key<Y: fmt::Debug>(key: &str, yaml: &Y) -> String {
    format!("Required key `{}` missing from {:?}", key, yaml)
}

pub fn key_of_type<Y: fmt::Debug>(key: &str,
                                  required: Required,
                                  yaml: Y,
                                  required_type: &str)
                                  -> String {
    format!("{} key `{}` in {:?} must be a {}",
            required,
            key,
            yaml,
            required_type)
}

pub fn bad_value<V: fmt::Debug, Y: fmt::Debug>(value: V, key: &str, context: &Y) -> String {
    format!("Invalid value {:?} for key `{}` in hash {:?}",
            value,
            key,
            context)
}

pub fn ridiculous_number<V: fmt::Display + ops::Add>(value: V,
                                                     key: &str,
                                                     context: &yaml::Hash)
                                                     -> String {
    format!("Seriously? You set the value of `{}` to {}? (The max is {}.)\nContext: {:?}",
            key,
            value,
            u8::MAX,
            context)
}
