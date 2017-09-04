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
        for (key, config) in configs {
            if let Some(value) = metadata.get(&Yaml::from_str(&key)) {
                if let Some(taxonomy) = Taxonomy::from_entry(value, config) {
                    taxonomies.insert(key.clone(), taxonomy);
                }
            };
        }

        taxonomies
    }

    fn from_entry(yaml: &Yaml, config: &config::taxonomy::Taxonomy) -> Option<Taxonomy> {
        unimplemented!()
    }
}
