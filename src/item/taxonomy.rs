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
                limit: maybe_limit,
                ..
            } => match entry {
                &Yaml::String(ref taxonomy_string) => {
                    let taxonomy_values = get_taxonomy_values(taxonomy_string, commas_as_lists);

                    match maybe_limit {
                        Some(limit) if taxonomy_values.len() > limit => {
                            Err(format!("only {} values allowed", limit))
                        }
                        Some(..) | None => Ok(Some(Taxonomy::TagLike {
                            name: name.into(),
                            values: get_split_taxonomy_values(&taxonomy_values, hierarchical),
                        })),
                    }
                }

                // TODO: e.g. series with fields.
                &Yaml::Hash(ref hash) => {
                    // TODO: Do fields match? If they don't match, how to handle
                    // them: ignore, or error, or warn?
                    unimplemented!()
                }

                &Yaml::Array(ref values) => if all_of_same_yaml_type(values) {
                    Ok(Some(Taxonomy::TagLike {
                        name: name.into(),
                        values: split_tags(values),
                    }))
                } else {
                    Err("not all values were of the same type".into())
                },

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
) -> Vec<Vec<String>> {
    if hierarchical {
        taxonomy_values
            .iter()
            .map(|tv| tv.split('/').map(String::from).collect())
            .collect()
    } else {
        vec![taxonomy_values.clone()]
    }
}


fn all_of_same_yaml_type(values: &Vec<yaml::Yaml>) -> bool {
    if values.len() == 0 {
        return true;
    }

    let is_same_variant: Box<Fn(&Yaml) -> bool> = match values.first().unwrap() {
        &Yaml::Alias(..) => Box::new(|_v| false),
        &Yaml::Array(..) => Box::new(|v| v.as_vec().is_some()),
        &Yaml::BadValue => Box::new(|v| v.is_badvalue()),
        &Yaml::Boolean(..) => Box::new(|v| v.as_bool().is_some()),
        &Yaml::Hash(..) => Box::new(|v| v.as_hash().is_some()),
        &Yaml::Integer(..) => Box::new(|v| v.as_i64().is_some()),
        &Yaml::Null => Box::new(|v| v.is_null()),
        &Yaml::Real(..) => Box::new(|v| v.as_f64().is_some()),
        &Yaml::String(..) => Box::new(|v| v.as_str().is_some()),
    };

    values.iter().all(|v| is_same_variant(v))
}

/// Split a
/// - `values` is a
fn split_tags(values: &Vec<yaml::Yaml>) -> Vec<PathSegments> {
    let result = values
        .into_iter()
        .map(|yaml_entry| match yaml_entry {
            &Yaml::String(ref s) => vec![s.clone()],
            &Yaml::Hash(ref _h) => panic!("hash"),
            &Yaml::Array(ref _a) => panic!("array"),
            _ => vec!["".into()],
        })
        .collect();

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_tags_single_level() {
        let alpha = "alpha";
        let beta = "beta";
        let charlie = "charlie";

        let the_yaml = vec![
            Yaml::from_str(alpha),
            Yaml::from_str(beta),
            Yaml::from_str(charlie),
        ];

        let tags = split_tags(&the_yaml);
        let expected: Vec<PathSegments> =
            vec![vec![alpha.into()], vec![beta.into()], vec![charlie.into()]];

        assert_eq!(tags, expected);
    }

    #[test]
    fn split_tags_nested() {
        let alpha = "alpha";
        let beta = "beta";
        let charlie = "charlie";

        let the_yaml = Yaml::from_str(&format!(
            "
- {}:
  - {}
  - {}
            ",
            alpha,
            beta,
            charlie
        ));

        let the_yaml = the_yaml.as_vec().expect("badly formed test data");
        let expected: Vec<PathSegments> = vec![
            vec![
                alpha.into(),
                format!("{}/{}", alpha, beta),
                format!("{}/{}", alpha, charlie),
            ],
        ];

        assert_eq!(split_tags(&the_yaml), expected);
    }
}
