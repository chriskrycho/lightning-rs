//! Handle configuration for templates.

// Standard library
use std::path::PathBuf;

// Third party
use yaml_rust::{yaml, Yaml};

// First party
use yaml_util::*;

#[derive(Debug, PartialEq)]
pub struct Templates {
    pub item: PathBuf,
    pub list: Option<PathBuf>,
}

impl Templates {
    pub fn from_yaml(yaml: &yaml::Hash) -> Result<Templates, String> {
        let key = "templates";
        let template_yaml = yaml[&Yaml::from_str(key)].as_hash().ok_or(key_of_type(
            key,
            Required::Yes,
            yaml,
            "hash",
        ))?;

        let item = Self::item_from_yaml(template_yaml)?;
        let list = Self::list_from_yaml(template_yaml)?;

        Ok(Templates {
            item: item,
            list: list,
        })
    }

    /// Get the `item` value for a taxonomy's templates.
    fn item_from_yaml(yaml: &yaml::Hash) -> Result<PathBuf, String> {
        let key = "item";
        Ok(yaml[&Yaml::from_str(key)]
            .as_str()
            .ok_or(key_of_type(key, Required::Yes, yaml, "string"))?
            .into())
    }

    /// Get the `list` value for a taxonomy's templates.
    ///
    /// This return type isn't as crazy as it looks. A `list` entry is allowed
    /// to be explicitly `null`/`~` or simply unset, but if the key is
    /// included, it is not allowed to be anything other than a `string` or
    /// explicitly set to `null`.
    fn list_from_yaml(yaml: &yaml::Hash) -> Result<Option<PathBuf>, String> {
        let key = "list";
        if yaml.contains_key(&Yaml::from_str(key)) {
            match yaml[&Yaml::from_str(key)] {
                Yaml::Null => Ok(None),
                Yaml::String(ref string) => Ok(Some(string.into())),
                _ => Err(key_of_type(key, Required::No, yaml, "string")),
            }
        } else {
            Ok(None)
        }
    }
}
