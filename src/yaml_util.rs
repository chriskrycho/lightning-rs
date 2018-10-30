//! Utilities for dealing with common YAML-processing tasks.

// Standard library
use std::collections::BTreeMap;
use std::fmt;
use std::ops;

// Third party
use yaml_rust::yaml::{Hash, Yaml};

#[derive(Debug)]
pub enum Required {
    Yes,
    No,
}

impl fmt::Display for Required {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Required::Yes => write!(f, "Required"),
            Required::No => write!(f, "Optional"),
        }
    }
}

/// Extract a key irrespective of the case the user supplied it in.
///
/// - `value`: the title-case or lower-case version of the string to match. The
///   YAML will be checked against both this and the Title-cased version. E.g.
///   if the value passed is `"hello"`, both `"hello"` and `"Hello"` will be
///   checked. If the value passed is `"Ahoy"`, both `"ahoy"` and `"Ahoy"` will
///   be
/// - `yaml`: a `yaml::Hash` against which to check the value.
/// - `required`: determines whether to return an *error* if the key is missing,
///   or to return `Ok(None)`. This lets the caller determine how to handle the
///   missing-value type as it deems appropriate, including e.g. simply by
///   letting it be passed up with `?` if `Required::Yes`.
pub fn case_insensitive_string(
    value: &str,
    yaml: &Hash,
    required: Required,
) -> Result<Option<String>, String> {
    let mut chars = value.chars();
    let first_char = chars.next().ok_or("no key passed")?;

    // TODO: can I avoid allocating a new `String` for each of these?
    let upper = format!("{}{}", first_char.to_uppercase(), chars.as_str());
    let lower = format!("{}{}", first_char.to_lowercase(), chars.as_str());
    let upper_key = Yaml::from_str(&upper);
    let lower_key = Yaml::from_str(&lower);

    match yaml.get(&upper_key).or_else(|| yaml.get(&lower_key)) {
        Some(&Yaml::String(ref value)) => Ok(Some(value.clone())),
        _ => match required {
            Required::No => Ok(None),
            Required::Yes => Err(key_of_type(
                format!("{} (case insensitive)", value).as_str(),
                required,
                yaml,
                "string",
            )),
        },
    }
}

pub fn required_key<Y: fmt::Debug>(key: &str, yaml: &Y) -> String {
    format!("Required key `{}` missing from {:?}", key, yaml)
}

pub fn key_of_type<Y: fmt::Debug>(
    key: &str,
    required: Required,
    yaml: Y,
    required_type: &str,
) -> String {
    format!(
        "{} key `{}` in {:?} must be a {}",
        required, key, yaml, required_type
    )
}

pub fn bad_value<V: fmt::Debug, Y: fmt::Debug>(value: V, key: &str, context: &Y) -> String {
    format!(
        "Invalid value {:?} for key `{}` in hash {:?}",
        value, key, context
    )
}

pub fn ridiculous_number<V: fmt::Display + ops::Add>(
    value: V,
    key: &str,
    max: usize,
    context: &Hash,
) -> String {
    format!(
        "Seriously? You set the value of `{}` to {}? (The max is {}.)\nContext: {:?}",
        key, value, max, context
    )
}

pub fn get_hash<'l>(
    key: &str,
    map: &'l BTreeMap<Yaml, Yaml>,
) -> Result<&'l BTreeMap<Yaml, Yaml>, String> {
    map.get(&Yaml::from_str(key))
        .ok_or_else(|| required_key(key, map))?
        .as_hash()
        .ok_or_else(|| key_of_type(key, Required::Yes, map, "hash"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_insensitive_string_handles_title_case() {
        let value = "the value";
        let mut yaml = Hash::new();
        yaml.insert(Yaml::from_str("Title"), Yaml::from_str(value));

        let lower = case_insensitive_string("title", &yaml, Required::No);
        assert_eq!(lower, Ok(Some(value.to_string())));

        let title = case_insensitive_string("Title", &yaml, Required::No);
        assert_eq!(title, Ok(Some(value.to_string())));
    }

    #[test]
    fn case_insensitive_string_handles_lower_case() {
        let value = "the value";
        let mut yaml = Hash::new();
        yaml.insert(Yaml::from_str("title"), Yaml::from_str(value));

        let lower = case_insensitive_string("title", &yaml, Required::No);
        assert_eq!(lower, Ok(Some(value.to_string())));

        let title = case_insensitive_string("Title", &yaml, Required::No);
        assert_eq!(title, Ok(Some(value.to_string())));
    }
}
