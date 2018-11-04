///
// Standard library
use std::collections::HashMap;

// Third party
use yaml_rust::{yaml, Yaml};
use chrono::prelude::*;

// First party
use config;
use config::Config;

/// PathSegments: a list of "paths" which comprise a hierarchical taxonomy.
///
/// If there is only one segment, i.e. the taxonomy is not hierarchical, this
/// will simply be a single-item `Vec`.
pub type PathSegments = Vec<String>; // SM - TagLike already includes the Vec so it isn't needed here?

/// An `item::taxonomy::Taxonomy` is a taxonomy *value* for an item.
#[derive(Debug, PartialEq)]
pub enum Taxonomy {
    Boolean {
        name: String, // SM - Do we need all these names? They are in the hash already.
        value: bool,
    },
    TagLike {
        name: String,
        values: Vec<PathSegments>,
        fields: Vec<(String, String)>,
    },
    Temporal {
        name: String,
        value: DateTime<Utc>,
    }, // TODO: `String` is wrong for Temporal
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
                    match Taxonomy::from_entry(value, name, taxonomy, config.rules.commas_as_lists)
                    {
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

        if errs.is_empty() {
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
        match *config_taxonomy {
            config::taxonomy::Taxonomy::Boolean { .. } => match *entry {
                Yaml::Boolean(value) => Ok(Some(Taxonomy::Boolean {
                    name: name.into(),
                    value,
                })),
                _ => Err("must be `true`, `false`, or left off entirely".to_string()),
            },

            config::taxonomy::Taxonomy::TagLike {
                required,
                hierarchical,
                limit: maybe_limit,
                ref fields_req,
                ref fields_opt,
                ..
            } => match *entry {
                Yaml::String(ref taxonomy_string) => {
                    let taxonomy_values = get_taxonomy_values(taxonomy_string, commas_as_lists);
                    match maybe_limit {
                        Some(limit) if taxonomy_values.len() > limit => {
                            Err(format!("only {} values allowed", limit))
                        }
                        Some(..) | None => Ok(Some(Taxonomy::TagLike {
                            name: name.into(),
                            values: get_split_taxonomy_values(&taxonomy_values, hierarchical),
                            fields: Vec::new(),
                        })),
                    }
                }

                Yaml::Hash(ref hash) => {
                    let mut fields: Vec<(String, String)> = Vec::new();

                    //check required fields - This seems a bit inside out, we should be checking that the required fields exist even if there isn't a Yaml::Hash
                    for field in fields_req {
                        match hash.get(&Yaml::from_str(&field)) {
                            None | Some(Yaml::Null) => {
                                return Err(format!(
                                    "Required field {:?} not found or empty.",
                                    field
                                ))
                            }
                            Some(Yaml::String(value)) => {
                                fields.push((field.to_string(), value.to_string()))
                            }
                            _ => {
                                return Err(format!(
                                    "Expected a string for {:?} and didn't find one.",
                                    field
                                ))
                            }
                        }
                    }

                    //check for optional fields
                    for field in fields_opt {
                        match hash.get(&Yaml::from_str(&field)) {
                            None | Some(Yaml::Null) => continue,
                            Some(Yaml::String(value)) => {
                                fields.push((field.to_string(), value.to_string()))
                            }
                            _ => {
                                return Err(format!(
                                    "Expected a string for {:?} and didn't find one.",
                                    field
                                ))
                            }
                        }
                    }

                    Ok(Some(Taxonomy::TagLike {
                        name: name.into(),
                        values: Vec::new(),
                        fields,
                    }))
                }

                Yaml::Array(ref values) => {
                    if all_of_same_yaml_type(values) {
                        Ok(Some(Taxonomy::TagLike {
                            name: name.into(),
                            values: extract_values(values),
                            fields: Vec::new(),
                        }))
                    } else {
                        Err("not all values were of the same type".into())
                    }
                }

                Yaml::Null => if required {
                    Err("is required".into())
                } else {
                    Ok(None)
                },
                _ => Err("".into()),
            },

            config::taxonomy::Taxonomy::Temporal { required, .. } =>  match *entry {
                Yaml::String(ref value) => Ok(Some(Taxonomy::Temporal {
                    name: name.into(),
                    value: Utc.datetime_from_str(value, "%Y-%m-%d %H:%M:%S").unwrap(),
                })),
                _ => Err("must be `true`, `false`, or left off entirely".to_string()),
            },
        }
    }
}

fn get_taxonomy_values(taxonomy_string: &str, commas_as_lists: bool) -> Vec<String> {
    if commas_as_lists {
        taxonomy_string.split(',').map(String::from).collect()
    } else {
        vec![taxonomy_string.into()]
    }
}

fn get_split_taxonomy_values(
    taxonomy_values: &Vec<String>,
    hierarchical: bool,
) -> Vec<PathSegments> {
    if hierarchical {
        taxonomy_values
            .iter()
            .map(|tv| tv.split('/').map(String::from).collect())
            .collect()
    } else {
        vec![taxonomy_values.clone()]
    }
}

fn all_of_same_yaml_type(values: &[yaml::Yaml]) -> bool {
    if values.is_empty() {
        return true;
    }

    let is_same_variant: Box<Fn(&Yaml) -> bool> = match *values.first().unwrap() {
        Yaml::Alias(..) => Box::new(|_v| false),
        Yaml::Array(..) => Box::new(|v| v.as_vec().is_some()),
        Yaml::BadValue => Box::new(|v| v.is_badvalue()),
        Yaml::Boolean(..) => Box::new(|v| v.as_bool().is_some()),
        Yaml::Hash(..) => Box::new(|v| v.as_hash().is_some()),
        Yaml::Integer(..) => Box::new(|v| v.as_i64().is_some()),
        Yaml::Null => Box::new(|v| v.is_null()),
        Yaml::Real(..) => Box::new(|v| v.as_f64().is_some()),
        Yaml::String(..) => Box::new(|v| v.as_str().is_some()),
    };

    values.iter().all(|v| is_same_variant(v))
}

fn extract_values(values: &[yaml::Yaml]) -> Vec<PathSegments> {
    vec![
        values
            .iter()
            .map(|v| match *v {
                Yaml::String(ref value) => value.clone(),
                _ => panic!("can only take strings!"),
            }).collect(),
    ]
}

#[cfg(test)]
mod tests {
    use config::Config;
    use item::taxonomy::Taxonomy;
    use std::collections::HashMap;
    use std::env;
    use std::path::PathBuf;
    use yaml_rust::YamlLoader;

    #[test]
    fn parses_metadata_from_post() {
        let mut site_directory: PathBuf = env::current_dir().unwrap();
        site_directory.push(r"tests/scenarios/pelican/");

        let config = Config::load(&PathBuf::from(&site_directory)).unwrap();

        let yaml_str = "
author: Steven, Chris
category: 
  - Test1
  - Test2
date: 2017-01-01 12:01 am
series:
    name: Testing
    part: One
";

        let yaml = YamlLoader::load_from_str(&yaml_str).unwrap();
        let yaml = yaml.into_iter().next().unwrap();
        let yaml = yaml.as_hash().unwrap();

        let taxonomy = Taxonomy::from_yaml_hash(yaml, &config);

        let expected = Ok(HashMap::new());
        assert_eq!(expected, taxonomy);
    }
}
