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
    pub feed_item: Option<PathBuf>,
    pub feed_list: Option<PathBuf>,
}

impl Templates {
    pub fn from_yaml(yaml: &yaml::Hash) -> Result<Templates, String> {
        let key = "templates";
        let template_yaml = yaml[&Yaml::from_str(key)]
            .as_hash()
            .ok_or_else(|| key_of_type(key, Required::Yes, yaml, "hash"))?;

        let item = Self::item_from_yaml(template_yaml)?;
        let list = Self::object_from_yaml("list", template_yaml)?;
        let feed_list = Self::object_from_yaml("feed_list", template_yaml)?;
        let feed_item = Self::object_from_yaml("feed_item", template_yaml)?;

        Ok(Templates {
            item,
            list,
            feed_item,
            feed_list,
        })
    }

    /// Get the `item` value for a taxonomy's templates.
    fn item_from_yaml(yaml: &yaml::Hash) -> Result<PathBuf, String> {
        let key = "item";
        Ok(yaml[&Yaml::from_str(key)]
            .as_str()
            .ok_or_else(|| key_of_type(key, Required::Yes, yaml, "string"))?
            .into())
    }

    /// Get a value for a taxonomy's templates.
    ///
    /// This return type isn't as crazy as it looks. An entry is allowed
    /// to be explicitly `null`/`~` or simply unset, but if the key is
    /// included, it is not allowed to be anything other than a `string` or
    /// explicitly set to `null`.
    fn object_from_yaml(key: &str, yaml: &yaml::Hash) -> Result<Option<PathBuf>, String> {
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
