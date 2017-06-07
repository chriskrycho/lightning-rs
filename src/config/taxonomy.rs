
// Standard library
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::u8;

// Third party
use yaml_rust::{yaml, Yaml, YamlLoader};

// First party
use yaml_util::*;


#[derive(Debug, PartialEq)]
pub enum Taxonomy {
    Binary {
        name: String,
        templates: Templates,
        hierarchical: bool,
    },
    Singular {
        name: String,
        fields: Vec<String>,
        templates: Templates,
        default: Option<String>,
        required: bool,
        hierarchical: bool,
    },
    Multiple {
        name: String,
        fields: Vec<String>,
        templates: Templates,
        default: Option<String>,
        limit: Option<u8>,
        required: bool,
        hierarchical: bool,
    },
    Temporal {
        name: String,
        templates: Templates,
        required: bool,
    },
}


impl Taxonomy {
    pub fn from_yaml(hash: &yaml::Hash, name: &str) -> Result<Taxonomy, String> {
        const TYPE: &str = "type";
        const BINARY: &str = "binary";
        const SINGULAR: &str = "singular";
        const MULTIPLE: &str = "multiple";
        const TEMPORAL: &str = "temporal";

        let name = String::from(name);
        let templates = Templates::from_yaml(hash)?;

        // Name can't collide with keyword `type`.
        let taxonomy_type = hash.get(&Yaml::from_str(TYPE))
            .ok_or(key_of_type(TYPE, Required::Yes, hash, "string"))?
            .as_str()
            .ok_or(key_of_type(TYPE, Required::Yes, hash, "string"))?;

        match taxonomy_type {
            BINARY => {
                Ok(
                    Taxonomy::Binary {
                        name: name,
                        templates: templates,
                        hierarchical: Self::is_hierarchical(hash)?,
                    }
                )
            },
            SINGULAR => {
                Ok(
                    Taxonomy::Singular {
                        name: name,
                        templates: templates,
                        default: Self::default_value(hash)?,
                        hierarchical: Self::is_hierarchical(hash)?,
                        required: Self::is_required(hash)?,
                        fields: Vec::new(),
                    }
                )
            },
            MULTIPLE => {
                Ok(
                    Taxonomy::Multiple {
                        name: name,
                        templates: templates,
                        default: Self::default_value(hash)?,
                        hierarchical: Self::is_hierarchical(hash)?,
                        required: Self::is_required(hash)?,
                        limit: Self::limit(hash)?,
                        fields: Vec::new(),
                    }
                )
            },
            TEMPORAL => {
                Ok(
                    Taxonomy::Temporal {
                        name: name,
                        templates: templates,
                        required: Self::is_required(hash)?,
                    }
                )
            },
            _ => Err(format!("Invalid taxonomy type `{:?}` in {:?}", taxonomy_type, hash)),
        }
    }

    fn default_value(hash: &yaml::Hash) -> Result<Option<String>, String> {
        let key = "default";
        match hash[&Yaml::from_str(key)] {
            Yaml::Null => Ok(None),
            Yaml::String(ref string) => Ok(Some(string.clone())),
            _ => Err(key_of_type(key, Required::No, hash, "string")),
        }
    }

    fn is_hierarchical(hash: &yaml::Hash) -> Result<bool, String> {
        let key = "hierarchical";
        match hash[&Yaml::from_str(key)] {
            Yaml::Boolean(boolean_value) => Ok(boolean_value),
            _ => Err(key_of_type(key, Required::Yes, hash, "bool")),
        }
    }

    fn is_required(hash: &yaml::Hash) -> Result<bool, String> {
        let key = "required";
        match hash[&Yaml::from_str(key)] {
            Yaml::Boolean(bool_value) => Ok(bool_value),
            _ => Err(key_of_type(key, Required::No, hash, "bool")),
        }
    }

    fn limit(hash: &yaml::Hash) -> Result<Option<u8>, String> {
        let key = "limit";
        let max = u8::MAX as i64;
        match hash[&Yaml::from_str(key)] {
            Yaml::Null => Ok(None),
            Yaml::Integer(i) if i < 0 => Err(bad_value(i, key, hash)),
            Yaml::Integer(i) if i == 0 => Ok(None),
            Yaml::Integer(i) if i > 0 && i < max => Ok(Some(i as u8)),
            Yaml::Integer(i) if i > max as i64 => Err(ridiculous_number(i, key, hash)),
            _ => Err(key_of_type(key, Required::No, hash, "integer")),
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct Templates {
    pub item: PathBuf,
    pub list: Option<PathBuf>,
}


impl Templates {
    fn from_yaml(yaml: &yaml::Hash) -> Result<Templates, String> {
        let key = "templates";
        let template_yaml = yaml[&Yaml::from_str(key)]
            .as_hash()
            .ok_or(key_of_type(key, Required::Yes, yaml, "hash"))?;

        let item = Self::item_from_yaml(template_yaml)?;
        let list = Self::list_from_yaml(template_yaml)?;

        Ok(
            Templates {
                item: item,
                list: list,
            }
        )
    }

    /// Get the `item` value for a taxonomy's templates.
    fn item_from_yaml(yaml: &yaml::Hash) -> Result<PathBuf, String> {
        let key = "item";
        Ok(
            yaml[&Yaml::from_str(key)]
                .as_str()
                .ok_or(key_of_type(key, Required::Yes, yaml, "string"))?
                .into()
        )
    }

    /// Get the `list` value for a taxonomy's templates.
    ///
    /// This return type isn't as crazy as it looks. A `list` entry is allowed
    /// to be explicitly `null`/`~` or simply unset, but if the key is
    /// included, it is not allowed to be anything other than a `string` or
    /// explicitly set to `null`.
    fn list_from_yaml(yaml: &yaml::Hash) -> Result<Option<PathBuf>, String> {
        let key = "list";
        match yaml[&Yaml::from_str(key)] {
            Yaml::Null => Ok(None),
            Yaml::String(ref string) => Ok(Some(string.into())),
            _ => Err(key_of_type(key, Required::No, yaml, "string")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_taxonomy_at_key(taxonomy: &str, key: &str) -> BTreeMap<Yaml, Yaml> {
        let mut loaded = YamlLoader::load_from_str(&taxonomy).unwrap();
        let first = loaded.pop().unwrap();
        first.as_hash().unwrap()[&Yaml::from_str(key)]
            .as_hash()
            .unwrap()
            .clone()
    }

    #[test]
    fn parses_hierarchical_multiple() {
        let taxonomy_name = "author";
        let taxonomy = format!(
            "
{}:
    type: multiple
    required: true
    hierarchical: false
    templates:
        list: authors.html
        item: author.html
        ",
            taxonomy_name
        );

        let expected = Taxonomy::Multiple {
            name: "author".into(),
            default: None,
            limit: None,
            required: true,
            hierarchical: false,
            fields: Vec::new(),
            templates: Templates {
                item: "author.html".into(),
                list: Some("authors.html".into()),
            },
        };

        let taxonomy_yaml = load_taxonomy_at_key(&taxonomy, taxonomy_name);
        assert_eq!(
            Ok(expected),
            Taxonomy::from_yaml(&taxonomy_yaml, taxonomy_name)
        );
    }

    #[test]
    fn parses_nonhierarchical_multiple() {
        let taxonomy_name = "category";
        let taxonomy = format!(
            "
{}:
    type: multiple
    default: Blog
    limit: 1
    required: false
    hierarchical: false
    templates:
        list: categories.html
        item: category.html
        ",
            taxonomy_name
        );

        let expected = Taxonomy::Multiple {
            name: "category".into(),
            default: Some("Blog".into()),
            limit: Some(1),
            required: false,
            hierarchical: false,
            fields: Vec::new(),
            templates: Templates {
                item: "category.html".into(),
                list: Some("categories.html".into()),
            },
        };

        let taxonomy_yaml = load_taxonomy_at_key(&taxonomy, taxonomy_name);
        assert_eq!(
            Ok(expected),
            Taxonomy::from_yaml(&taxonomy_yaml, taxonomy_name)
        );
    }

    #[test]
    fn parses_nonhierarchical_multiple_without_default() {
        let taxonomy_name = "tag";
        let taxonomy = format!(
            "
{}:
    type: multiple
    limit: ~
    required: false
    hierarchical: false
    templates:
        list: tags.html
        item: tag.html
        ",
            taxonomy_name
        );

        let expected = Taxonomy::Multiple {
            name: "tag".into(),
            default: None,
            limit: None,
            required: false,
            hierarchical: false,
            fields: Vec::new(),
            templates: Templates {
                item: "tag.html".into(),
                list: Some("tags.html".into()),
            },
        };

        let taxonomy_yaml = load_taxonomy_at_key(&taxonomy, taxonomy_name);
        assert_eq!(
            Ok(expected),
            Taxonomy::from_yaml(&taxonomy_yaml, taxonomy_name)
        );
    }

    #[test]
    fn parses_temporal() {
        let taxonomy_name = "date";
        let taxonomy = format!(
            "
{}:
    type: temporal
    required: false
    templates:
        list: period_archives.html
        item: archives.html
        ",
            taxonomy_name
        );

        let expected = Taxonomy::Temporal {
            name: "date".into(),
            required: false,
            templates: Templates {
                item: "archives.html".into(),
                list: Some("period_archives.html".into()),
            },
        };

        let taxonomy_yaml = load_taxonomy_at_key(&taxonomy, taxonomy_name);
        assert_eq!(
            Ok(expected),
            Taxonomy::from_yaml(&taxonomy_yaml, taxonomy_name)
        );
    }

    #[test]
    fn parses_binary() {
        let taxonomy_name = "page";
        let taxonomy = format!(
            "
{}:
    type: binary
    hierarchical: true
    templates:
        item: page.html
        ",
            taxonomy_name
        );

        let expected = Taxonomy::Binary {
            name: "page".into(),
            hierarchical: true,
            templates: Templates {
                item: "page.html".into(),
                list: None,
            },
        };

        let taxonomy_yaml = load_taxonomy_at_key(&taxonomy, taxonomy_name);
        assert_eq!(
            Ok(expected),
            Taxonomy::from_yaml(&taxonomy_yaml, taxonomy_name)
        );
    }
}
