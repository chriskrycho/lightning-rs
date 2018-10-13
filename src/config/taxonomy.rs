
// Standard library

// Third party
use yaml_rust::{yaml, Yaml};

// First party
use yaml_util::*;
use config::templates::Templates;


/// A `config::Taxonomy` represents a way of *defining* taxonomies for a site.
///
/// Each taxonomy may be one of "boolean", "categorical", "taglike", or
/// "temporal". These options are mutually exclusive: a taxonomy may use time
/// *or* it may define its own structure, but sad to say, we don't get to define
/// the structure of time itself (unless you're a [Time Lord], in which case I
/// would like to borrow your TARDIS).
///
/// [Time Lord]: https://en.wikipedia.org/wiki/Time_Lord
#[derive(Debug, PartialEq)]
pub enum Taxonomy {
    /// A taxonomy to which an item either belongs or does not.
    ///
    /// For example, in many CMS setups, an item is either a standalone "page"
    /// or *not*.
    ///
    /// Boolean taxonomies do not have multiple variants; so e.g. in this setup
    /// an item is a "page" or it is not---unlike a `Multiple` taxonomy, where
    /// an item does not belong to e.g. the "category" taxonomy so much as to
    /// one of the variants *within* the taxonomy.
    Boolean { name: String, templates: Templates },

    /// "TagLike"-type taxonomies have as many variants as you define. You
    /// might have both "Tech" and "Art" as categories, for example. Then an
    /// item could belong to both of them, if it were about digital art
    /// creation.
    TagLike {
        /// The name of the taxonomy.
        name: String,

        /// The `fields` are used to specify a taxonomy when it includes more
        /// data than simply the name. For example, in the case of a "series,"
        /// the series has a name, but each item in it also includes a "part",
        /// so that an item is ultimately taxonomized as being something like
        /// "Series Foo, Part 2" from input data like this:
        ///
        /// ```yaml
        /// Series:
        ///   Name: Foo
        ///   Part: 2
        /// ```
        ///
        /// `fields` may be `required` (the item will fail to build and an error
        /// will be logged if the taxonomy is specified but not all its required
        /// fields are set) or `optional` (the item will build fine).
        fields: Vec<String>,

        /// The templates to use in generating this taxonomy.
        templates: Templates,

        /// The "default" key allows you to specify a default value for items.
        default: Option<String>,

        /// Use the "limit" field to specify whether a multiple-variant taxonomy
        /// may have more than one field applied. The field may be implicitly
        /// set to null by leaving it out, or explicitly set to null with `~`.
        /// If set, it must be a number greater than 0. (Other values will be
        /// treated as errors.) See examples below.
        limit: Option<usize>,

        /// The "required" key may be set for any field which is required for
        /// all pieces of content (e.g. on many sites, "author" might be
        /// required).
        required: bool,

        /// Use the `hierarchical` field to specify (`true` or `false`) whether
        /// a taxonomy may be nested, e.g. "Parent Category -> Child Category".
        /// Taxonomies are implicitly defined with `hierarchical: false` if the
        /// field is not included explicitly.
        hierarchical: bool,
    },

    /// `Temporal` taxonomies represent time -- usually as a *date*.
    ///
    /// Note: the `date` taxonomy is normally implicit, and automatically
    /// associated with any piece of content stamped with a `date` field. You
    /// only need to define it explicitly if you want to customize the
    /// associated templates, or if you want to use something besides `date` to
    /// specify the time stamp for a given item. The `limit` field here is
    /// *always* ignored.
    Temporal {
        /// The name of the taxonomy.
        name: String,

        /// The templates to use in generating this taxonomy.
        templates: Templates,

        /// The "required" key may be set for any field which is required for
        /// all pieces of content (e.g. on many sites, "author" might be
        /// required).
        required: bool,
    },
}


impl Taxonomy {
    pub fn from_yaml(hash: &yaml::Hash, name: &str) -> Result<Taxonomy, String> {
        const TYPE: &str = "type";
        const BOOLEAN: &str = "boolean";
        const SINGULAR: &str = "singular";
        const TAGLIKE: &str = "taglike";
        const TEMPORAL: &str = "temporal";

        let name = String::from(name);
        let templates = Templates::from_yaml(hash)?;

        // Name can't collide with keyword `type`.
        let taxonomy_type = hash.get(&Yaml::from_str(TYPE))
            .ok_or(key_of_type(TYPE, Required::Yes, hash, "string"))?
            .as_str()
            .ok_or(key_of_type(TYPE, Required::Yes, hash, "string"))?;

        match taxonomy_type {
            BOOLEAN => Ok(Taxonomy::Boolean {
                name: name,
                templates: templates,
            }),

            SINGULAR => Ok(Taxonomy::TagLike {
                name: name,
                templates: templates,
                default: Self::default_value(hash)?,
                hierarchical: Self::is_hierarchical(hash)?,
                required: Self::required_field_value(hash)?,
                limit: Some(1),
                fields: Vec::new(),
            }),

            TAGLIKE => {
                Ok(Taxonomy::TagLike {
                    name: name,
                    templates: templates,
                    default: Self::default_value(hash)?,
                    hierarchical: Self::is_hierarchical(hash)?,
                    required: Self::required_field_value(hash)?,
                    limit: Self::limit(hash)?,
                    fields: Vec::new(),
                })
            },

            TEMPORAL => Ok(Taxonomy::Temporal {
                name: name,
                templates: templates,
                required: Self::required_field_value(hash)?,
            }),

            _ => Err(format!(
                "Invalid taxonomy type `{:?}` in {:?}",
                taxonomy_type,
                hash
            )),
        }
    }

    pub fn is_required(&self) -> bool {
        match self {
            &Taxonomy::Boolean { .. } => false,
            &Taxonomy::TagLike { required, .. } => required,
            &Taxonomy::Temporal { required, .. } => required,
        }
    }

    fn default_value(hash: &yaml::Hash) -> Result<Option<String>, String> {
        let key = &Yaml::from_str("default");
        if hash.contains_key(key) {
            match hash[key] {
                Yaml::Null => Ok(None),
                Yaml::String(ref string) => Ok(Some(string.clone())),
                _ => Err(key_of_type("default", Required::No, hash, "string")),
            } 
        } else {
            Ok(None)
        }
    }

    fn is_hierarchical(hash: &yaml::Hash) -> Result<bool, String> {
        let key = "hierarchical";
        let yaml_key=&Yaml::from_str(key);
        if !hash.contains_key(yaml_key) {
            return Ok(false);
        }
        match hash[yaml_key] {
            Yaml::Boolean(boolean_value) => Ok(boolean_value),
            _ => Err(key_of_type(key, Required::Yes, hash, "bool")),
        }
    }

    fn required_field_value(hash: &yaml::Hash) -> Result<bool, String> {
        let key = "required";
        let yaml_key=&Yaml::from_str(key);
        if !hash.contains_key(yaml_key) {
            return Ok(false);
        }
        match hash[yaml_key] {
            Yaml::Boolean(bool_value) => Ok(bool_value),
            _ => Err(key_of_type(key, Required::No, hash, "bool")),
        }
    }

    fn limit(hash: &yaml::Hash) -> Result<Option<usize>, String> {
        let key = "limit";
        let yaml_key=&Yaml::from_str(key);
        const max:usize = usize::max_value();
        if !hash.contains_key(yaml_key) {
            return Ok(None);
        }
        match hash[yaml_key] {
            Yaml::Null => Ok(None),
            Yaml::Integer(i) => {
                match i as usize {
                    0 => Ok(None),
                    1 ... max => Ok(Some(i as usize)),
                    _ => Err(key_of_type(key, Required::No, hash, "integer")),
                }
            },
            _ => Err(key_of_type(key, Required::No, hash, "integer")),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use::config::taxonomy::Taxonomy;
    use std::collections::BTreeMap;
    use yaml_rust::YamlLoader;

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
    type: taglike
    required: true
    hierarchical: false
    templates:
        list: authors.html
        item: author.html
        ",
            taxonomy_name
        );

        let expected = Taxonomy::TagLike {
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
    type: taglike
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

        let expected = Taxonomy::TagLike {
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
    type: taglike
    limit: ~
    required: false
    hierarchical: false
    templates:
        list: tags.html
        item: tag.html
        ",
            taxonomy_name
        );

        let expected = Taxonomy::TagLike {
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
    fn parses_boolean() {
        let taxonomy_name = "page";
        let taxonomy = format!(
            "
{}:
    type: boolean
    hierarchical: true
    templates:
        item: page.html
        ",
            taxonomy_name
        );

        let expected = Taxonomy::Boolean {
            name: "page".into(),
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
