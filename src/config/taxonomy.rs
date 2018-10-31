// Standard library

// Third party
use yaml_rust::{yaml, Yaml};

// First party
use config::templates::Templates;
use yaml_util::*;

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
        fields_req: Vec<String>,
        fields_opt: Vec<String>,

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

        date_format: String,
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
        let taxonomy_type = hash
            .get(&Yaml::from_str(TYPE))
            .ok_or_else(|| key_of_type(TYPE, Required::Yes, hash, "string"))?
            .as_str()
            .ok_or_else(|| key_of_type(TYPE, Required::Yes, hash, "string"))?;

        match taxonomy_type {
            BOOLEAN => Ok(Taxonomy::Boolean { name, templates }),

            SINGULAR => Ok(Taxonomy::TagLike {
                name,
                templates,
                default: Self::default_value(hash)?,
                hierarchical: Self::is_hierarchical(hash)?,
                required: Self::required_field_value(hash)?,
                limit: Some(1),
                fields_req: Self::get_fields("required".into(), hash)?,
                fields_opt: Self::get_fields("optional".into(), hash)?,
            }),

            TAGLIKE => Ok(Taxonomy::TagLike {
                name,
                templates,
                default: Self::default_value(hash)?,
                hierarchical: Self::is_hierarchical(hash)?,
                required: Self::required_field_value(hash)?,
                limit: Self::limit(hash)?,
                fields_req: Self::get_fields("required".into(), hash)?,
                fields_opt: Self::get_fields("optional".into(), hash)?,
            }),

            TEMPORAL => Ok(Taxonomy::Temporal {
                name,
                templates,
                required: Self::required_field_value(hash)?,
                date_format: Self::date_format_value(hash)?,
            }),

            _ => Err(format!(
                "Invalid taxonomy type `{:?}` in {:?}",
                taxonomy_type, hash
            )),
        }
    }

    pub fn is_required(&self) -> bool {
        match *self {
            Taxonomy::Boolean { .. } => false,
            Taxonomy::TagLike { required, .. } => required,
            Taxonomy::Temporal { required, .. } => required,
        }
    }

    fn default_value(hash: &yaml::Hash) -> Result<Option<String>, String> {
        match hash.get(&Yaml::from_str("default")) {
            None | Some(Yaml::Null) => Ok(None),
            Some(Yaml::String(ref string)) => Ok(Some(string.clone())),
            _ => Err(key_of_type("default", Required::No, hash, "string")),
        }
    }

    fn is_hierarchical(hash: &yaml::Hash) -> Result<bool, String> {
        match hash.get(&Yaml::from_str("hierarchical")) {
            None => Ok(false),
            Some(Yaml::Boolean(boolean_value)) => Ok(*boolean_value),
            _ => Err(key_of_type("hierarchical", Required::Yes, hash, "bool")),
        }
    }

    fn required_field_value(hash: &yaml::Hash) -> Result<bool, String> {
        match hash.get(&Yaml::from_str("required")) {
            None => Ok(false),
            Some(Yaml::Boolean(bool_value)) => Ok(*bool_value),
            _ => Err(key_of_type("required", Required::No, hash, "bool")),
        }
    }

    fn limit(hash: &yaml::Hash) -> Result<Option<usize>, String> {
        let key = "limit";
        match hash.get(&Yaml::from_str(key)) {
            None | Some(Yaml::Null) => Ok(None),
            Some(Yaml::Integer(0)) => Ok(None),
            Some(Yaml::Integer(1)) => Ok(Some(1)),
            Some(Yaml::Integer(i)) if *i < 0 => Err(bad_value(i, key, hash)),
            Some(Yaml::Integer(i)) if (*i as i32) > i32::max_value() => {
                Err(ridiculous_number(*i, key, usize::max_value(), hash))
            }
            Some(Yaml::Integer(i)) => Ok(Some(*i as usize)),
            _ => Err(key_of_type("limit", Required::No, hash, "integer")),
        }
    }

    fn get_fields(req: String, hash: &yaml::Hash) -> Result<Vec<String>, String> {
        match hash.get(&Yaml::from_str("fields")) {
            None => Ok(Vec::new()),
            Some(Yaml::Hash(field_hash)) => {
                match field_hash.get(&Yaml::from_str(&req)) {
                    None | Some(Yaml::Null) => Ok(Vec::new()),
                    Some(Yaml::String(ref name)) => {
                        let mut field_vec = Vec::new();
                        field_vec.push(name.clone());
                        Ok(field_vec)
                    }
                    Some(Yaml::Array(values)) => {
                        Ok(values
                            .iter()
                            .map(|v| match *v {
                                Yaml::String(ref value) => value.clone(),
                                _ => panic!("can only take strings!"), // SM - TODO: need to change to return an error rather than panic but at least it builds for now
                            }).collect())
                    }
                    _ => Err(key_of_type(
                        "Fields",
                        Required::No,
                        field_hash,
                        "Array or String",
                    )),
                }
            }
            _ => Err(key_of_type("fields", Required::No, hash, "hash")),
        }
    }

    fn date_format_value(hash: &yaml::Hash) -> Result<String, String> {
        match hash.get(&Yaml::from_str("format")) {
            None | Some(Yaml::Null) => Ok("%Y-%m-%d %H:%M %P".into()), //default
            Some(Yaml::String(ref value)) => Ok(value.clone()), //should this be validated somehow or just leave it until it fails when building a page?
            _ => Err(key_of_type("Date/Format", Required::No, hash, "String")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::taxonomy::Taxonomy;
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
            fields_req: Vec::new(),
            fields_opt: Vec::new(),
            templates: Templates {
                item: "author.html".into(),
                list: Some("authors.html".into()),
                feed_item: None,
                feed_list: None,
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
            fields_req: Vec::new(),
            fields_opt: Vec::new(),
            templates: Templates {
                item: "category.html".into(),
                list: Some("categories.html".into()),
                feed_item: None,
                feed_list: None,
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
            fields_req: Vec::new(),
            fields_opt: Vec::new(),
            templates: Templates {
                item: "tag.html".into(),
                list: Some("tags.html".into()),
                feed_item: None,
                feed_list: None,
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
                feed_item: None,
                feed_list: None,
            },
            date_format: "%Y-%m-%d %H:%M %P".into(),
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
                feed_item: None,
                feed_list: None,
            },
        };

        let taxonomy_yaml = load_taxonomy_at_key(&taxonomy, taxonomy_name);
        assert_eq!(
            Ok(expected),
            Taxonomy::from_yaml(&taxonomy_yaml, taxonomy_name)
        );
    }
}
