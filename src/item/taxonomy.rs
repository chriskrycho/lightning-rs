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
    pub fn from_yaml_hash(
        metadata: &yaml::Hash,
        configs: &config::Taxonomies,
    ) -> HashMap<String, Taxonomy> {
        let mut taxonomies = HashMap::new();
        for key in configs.keys() {
            if let Some(value) = metadata.get(&Yaml::from_str(&key)) {
                if let Some(taxonomy) = Taxonomy::from_entry(value) {
                    taxonomies.insert(key.clone(), taxonomy);
                }
            };
        }

        taxonomies
    }

    fn from_entry(yaml: &Yaml) -> Option<Taxonomy> {
        unimplemented!("Need to parse each YAML hash item into a `Taxonomy`");
    }
}
