///

// Standard library
use std::collections::HashMap;

// Third party
use yaml_rust::{yaml, Yaml};

// First party
use config;
use config::Config;

/// PathSegments: a list of "paths" which comprise a hierarchical taxonomy.
///
/// If there is only one segment, i.e. the taxonomy is not hierarchical, this
/// will simply be a single-item `Vec`.
pub type PathSegments = Vec<String>;

/// An `item::taxonomy::Taxonomy` is a taxonomy *value* for an item.
pub enum Taxonomy {
    Boolean { name: String, value: bool },
    TagLike {
        name: String,
        values: Vec<PathSegments>,
    },
    Temporal { name: String, value: String }, // TODO: `String` is wrong for Temporal
}

impl Taxonomy {
    /// Given a hash representing a single item taxonomy, attempt to parse it.
    ///
    /// The result is either a valid taxonomy with its name, or a list of the
    /// reason the taxonomy entry is not valid.
    pub fn from_yaml_hash(
        metadata: &yaml::Hash,
        config: &Config,
    ) -> Result<HashMap<String, Taxonomy>, String> {
        let mut taxonomies = HashMap::new();
        let mut errs = HashMap::new();

        for (name, taxonomy) in &config.taxonomies {
            match metadata.get(&Yaml::from_str(&name)) {
                None => if taxonomy.is_required() {
                    errs.insert(name.clone(), String::from("is required but not present"));
                },
                Some(value) => {
                    match Taxonomy::from_entry(value, name, taxonomy, config.rules.commasAsLists) {
                        Ok(Some(taxonomy)) => {
                            taxonomies.insert(name.clone(), taxonomy);
                        }
                        Ok(None) => { /* we can just skip these */ }
                        Err(reason) => {
                            errs.insert(name.clone(), reason);
                        }
                    }
                }
            }
        }

        if errs.len() == 0 {
            Ok(taxonomies)
        } else {
            let mut merged_errs = String::from("");
            for (name, reason) in errs {
                let err = format!("\n\t'{}': {}", name, reason);
                merged_errs.push_str(&err);
            }

            Err(merged_errs)
        }
    }

    // TODO: this is *crazy* nested. Seems like a sign that perhaps the data
    // structure should be rethought. Also an opportunity to extract some
    // functions, I think.
    /// Return the `Taxonomy` or a description of the reason it's invalid.
    ///
    /// Validity is defined in terms of whether the specified item matches the
    /// corresponding configuration rule for the taxonomy of that name.
    fn from_entry(
        entry: &Yaml,
        name: &str,
        config_taxonomy: &config::taxonomy::Taxonomy,
        commas_as_lists: bool,
    ) -> Result<Option<Taxonomy>, String> {
        match config_taxonomy {
            &config::taxonomy::Taxonomy::Boolean { .. } => match entry {
                &Yaml::Boolean(value) => Ok(Some(Taxonomy::Boolean {
                    name: name.into(),
                    value,
                })),
                _ => Err(format!("must be `true`, `false`, or left off entirely")),
            },

            &config::taxonomy::Taxonomy::TagLike {
                required,
                hierarchical,
                limit,
                ..
            } => match entry {
                &Yaml::String(ref taxonomy_string) => {
                    let taxonomy_values = if commas_as_lists {
                        vec![taxonomy_string.clone()]
                    } else {
                        taxonomy_string.split(',').map(String::from).collect()
                    };

                    let taxonomy_values = if hierarchical {
                        taxonomy_values
                            .iter()
                            .map(|tv| tv.split('/').map(String::from).collect())
                            .collect()
                    } else {
                        vec![taxonomy_values]
                    };

                    match limit {
                        Some(limit_value) if taxonomy_values.len() > limit_value => {
                            Err(format!("only {} values allowed", limit_value))
                        }
                        Some(..) | None => Ok(Some(Taxonomy::TagLike {
                            name: name.into(),
                            values: taxonomy_values,
                        })),
                    }
                }
                &Yaml::Hash(ref hash) => unimplemented!(),
                &Yaml::Array(ref values) => unimplemented!(),
                &Yaml::Null => if required {
                    Err("is required".into())
                } else {
                    Ok(None)
                },
                _ => Err("".into()),
            },

            &config::taxonomy::Taxonomy::Temporal { required, .. } => {
                unimplemented!("can't yet parse Temporal item configs")
            }
        }
    }
}
