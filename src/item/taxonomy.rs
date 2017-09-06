///

// Standard library
use std::collections::HashMap;

// Third party
use yaml_rust::{yaml, Yaml, YamlLoader};

// First party
use config;
use config::Config;


pub enum Taxonomy {
    Boolean { name: String, value: bool },
    Single { name: String, value: String },
    TagLike { name: String, values: Vec<String> },
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
                hierarchical, // TODO: what should we do with this?
                limit,        // TODO: and this
                ..
            } => match entry {
                &Yaml::String(ref value) => {
                    let values = if commas_as_lists {
                        vec![]
                    } else {
                        value.split(',').collect()
                    };

                    match limit {
                        Some(limit_value) => if values.len() > limit_value {
                            Err(format!("only {} values allowed", limit_value))
                        } else if limit_value == 1 {
                            Ok(Some(Taxonomy::Single {
                                name: name.into(),
                                value: value.clone(),  // just use the base value
                            }))
                        } else {
                            Ok(Some(Taxonomy::TagLike {
                                name: name.into(),
                                values,
                            }))
                        },
                        None => unimplemented!(),
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
