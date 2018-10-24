//! Process configurations for Lightning sites.

pub mod directories;
pub mod site_info;
pub mod taxonomy;
pub mod templates;

// First-party
use std::collections::{BTreeMap, HashMap};
use std::convert::From;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

// Third-party
use yaml_rust::{Yaml, YamlLoader};

// First-party
// use validated_types::Url as ValidatedUrl;
use yaml_util::*;

use self::directories::*;
use self::site_info::*;
use self::taxonomy::*;

const CONFIG_FILE_NAME: &str = "lightning.yaml";

pub type Name = String;
pub type Taxonomies = HashMap<Name, Taxonomy>;

#[derive(Debug, PartialEq)]
pub struct Rules {
    pub commas_as_lists: bool,
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub site: SiteInfo,
    pub directories: Directories,
    pub taxonomies: Taxonomies,
    pub rules: Rules,
}

impl Config {
    pub fn load(directory: &PathBuf) -> Result<Config, String> {
        let config_path = directory.join(CONFIG_FILE_NAME);
        if !config_path.exists() {
            return Err(format!(
                "The specified configuration path {:?} does not exist.",
                config_path.to_string_lossy()
            ));
        }

        let mut file = File::open(&config_path)
            .map_err(|reason| format!("Error reading {:?}: {:?}", config_path, reason))?;

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(err) => return Err(String::from(err.description())),
        };

        // We need all these intermediate bindings because the temporaries created
        // along the way don't live long enough otherwise.
        let load_result = YamlLoader::load_from_str(&contents)
            .map_err(|err| format!("{} ({:?})", err, &config_path))?;
        let yaml_config = load_result
            .into_iter()
            .next()
            .ok_or("Empty configuration file")?;
        let config_map = yaml_config.as_hash().ok_or("Configuration is not a map")?;

        let layout = Self::get_layout(config_map)?;
        let rules = Self::get_rules(&layout)?;

        Ok(Config {
            site: Self::parse_site_meta(config_map)?,
            directories: Directories::from_yaml(config_map, &config_path, &layout)?,
            taxonomies: Self::parse_taxonomies(&layout, &config_path)?,
            rules: Self::parse_rules(&rules)?,
        })
    }

    fn get_layout<'map>(
        config_map: &'map BTreeMap<Yaml, Yaml>,
    ) -> Result<&'map BTreeMap<Yaml, Yaml>, String> {
        const LAYOUT: &str = "layout";
        config_map
            .get(&Yaml::from_str(LAYOUT))
            .ok_or_else(|| required_key(LAYOUT, config_map))?
            .as_hash()
            .ok_or_else(|| key_of_type(LAYOUT, Required::Yes, config_map, "hash"))
    }

    fn get_rules<'l>(layout: &'l BTreeMap<Yaml, Yaml>) -> Result<&'l BTreeMap<Yaml, Yaml>, String> {
        const RULES: &str = "taxonomy_rules";
        layout
            .get(&Yaml::from_str(RULES))
            .ok_or_else(|| required_key(RULES, layout))?
            .as_hash()
            .ok_or_else(|| key_of_type(RULES, Required::Yes, layout, "hash"))
    }

    /// Load the site data from the configuration file.
    fn parse_site_meta(config_map: &BTreeMap<Yaml, Yaml>) -> Result<SiteInfo, String> {
        const SITE_INFO: &str = "site_info";
        let site_info_yaml = config_map
            .get(&Yaml::from_str(SITE_INFO))
            .ok_or_else(|| required_key(SITE_INFO, config_map))?
            .as_hash()
            .ok_or_else(|| key_of_type(SITE_INFO, Required::Yes, config_map, "hash"))?;

        SiteInfo::from_yaml(&site_info_yaml)
    }

    /// Load the taxonomies from the configuration file.
    fn parse_taxonomies(
        structure: &BTreeMap<Yaml, Yaml>,
        config_path: &PathBuf,
    ) -> Result<Taxonomies, String> {
        const TAXONOMIES: &str = "taxonomies";

        let taxonomies_yaml = structure
            .get(&Yaml::from_str(TAXONOMIES))
            .ok_or_else(|| format!("No `{}` key in {:?}", TAXONOMIES, config_path))?
            .as_hash()
            .ok_or_else(|| format!("`{}` is not a hash in {:?}", TAXONOMIES, config_path))?;

        let mut taxonomies = Taxonomies::new();
        if taxonomies_yaml.is_empty() {
            return Ok(taxonomies);
        }

        for name in taxonomies_yaml.keys() {
            let key = name.as_str().expect("If this isn't here, YAML is broken.");
            let content = taxonomies_yaml
                .get(name)
                .ok_or_else(|| required_key(key, taxonomies_yaml))?
                .as_hash()
                .ok_or_else(|| key_of_type(key, Required::Yes, taxonomies_yaml, "hash"))?;
            let taxonomy = Taxonomy::from_yaml(content, key)?;
            if taxonomies.insert(key.into(), taxonomy).is_some() {
                return Err(format!("duplicate key {}", key));
            }
        }

        Ok(taxonomies)
    }

    fn parse_rules(rules: &BTreeMap<Yaml, Yaml>) -> Result<Rules, String> {
        const COMMAS_AS_LISTS: &str = "commas_as_lists";
        let key = &Yaml::from_str(COMMAS_AS_LISTS);

        match rules.get(key) {
            None => Ok(Rules {
                commas_as_lists: false,
            }),
            Some(Yaml::Boolean(value)) => Ok(Rules {
                commas_as_lists: *value,
            }),
            _ => panic!("expected a bool and didn't get it"),
        }
    }
}

#[cfg(test)]
mod tests {
    use self::templates::Templates;
    use super::*;
    use config::Config;
    use std::env;
    use std::path::PathBuf;

    #[test]
    fn parses_full_config() {
        let mut site_directory: PathBuf = env::current_dir().unwrap();
        site_directory.push(r"tests/scenarios/pelican/");

        let config = Config::load(&PathBuf::from(&site_directory)).unwrap();

        let site = SiteInfo {
            title: "lx (lightning)".into(),
            url: ValidatedUrl::new("https://lightning.rs").unwrap(),
            description: Some("A ridiculously fast site generator and engine.\n".into()),
            default_timezone: None,
            metadata: HashMap::new(),
        };

        let directories = Directories {
            content: PathBuf::from("content"),
            output: PathBuf::from("output"),
            template: PathBuf::from("layout"),
        };

        let mut taxonomies = HashMap::new();
        let tax_author = Taxonomy::TagLike {
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
        taxonomies.insert("author".into(), tax_author);

        let tax_category = Taxonomy::TagLike {
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
        taxonomies.insert("category".into(), tax_category);

        let tax_tag = Taxonomy::TagLike {
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
        taxonomies.insert("tag".into(), tax_tag);

        let tax_date = Taxonomy::Temporal {
            name: "date".into(),
            required: false,
            templates: Templates {
                item: "archives.html".into(),
                list: Some("period_archives.html".into()),
            },
        };
        taxonomies.insert("date".into(), tax_date);

        let tax_page = Taxonomy::Boolean {
            name: "page".into(),
            templates: Templates {
                item: "page.html".into(),
                list: None,
            },
        };
        taxonomies.insert("page".into(), tax_page);

        let tax_series = Taxonomy::TagLike {
            name: "series".into(),
            default: None,
            limit: Some(1),
            required: false,
            hierarchical: false,
            fields: Vec::new(),
            templates: Templates {
                item: "series.html".into(),
                list: Some("series-list.html".into()),
            },
        };
        taxonomies.insert("series".into(), tax_series);

        let expected = Config {
            site,
            directories,
            taxonomies,
            rules: Rules {
                commas_as_lists: true,
            },
        };

        assert_eq!(expected, config);
    }
}
