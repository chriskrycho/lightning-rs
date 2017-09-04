///

// Standard library
use std::collections::HashMap;

// Third party
use yaml_rust::{yaml, Yaml, YamlLoader};

// First party
use config;


pub enum Taxonomy {
    Single { name: String, value: String },
    Multiple { name: String, values: Vec<String> },
}

impl Taxonomy {
    /// Given a hash representing a single item taxonomy, attempt to parse it.
    ///
    /// The result is either a valid taxonomy with its name, or a list of the
    /// reason the taxonomy entry is not valid.
    pub fn from_yaml_hash(
        metadata: &yaml::Hash,
        configs: &config::Taxonomies,
    ) -> Result<HashMap<String, Taxonomy>, String> {
        let mut taxonomies = HashMap::new();
        let mut errs = HashMap::new();

        for (name, config) in configs {
            if let Some(value) = metadata.get(&Yaml::from_str(&name)) {
                match Taxonomy::from_entry(value, config) {
                    Ok(taxonomy) => {
                        taxonomies.insert(name.clone(), taxonomy);
                    }
                    Err(reason) => {
                        errs.insert(name.clone(), reason);
                    }
                }
            };
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

    /// Return the `Taxonomy` or a description of the reason it's invalid.
    ///
    /// Validity is defined in terms of whether the specified item matches the
    /// corresponding configuration rule for the taxonomy of that name.
    fn from_entry(yaml: &Yaml, config: &config::taxonomy::Taxonomy) -> Result<Taxonomy, String> {
        unimplemented!()
    }
}
