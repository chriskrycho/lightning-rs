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
use yaml_util::get_hash;

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
pub struct IndexTemplates {
    pub index: PathBuf,
    pub item: PathBuf,
}

#[derive(Debug, PartialEq)]
pub struct Rules {
    pub commas_as_lists: bool,
}
#[derive(Debug, PartialEq)]
pub struct OtherContent {
    pub copy_paths: Vec<PathBuf>,
    pub exclude: Vec<PathBuf>,
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub site: SiteInfo,
    pub directories: Directories,
    pub taxonomies: Taxonomies,
    pub templates: IndexTemplates,
    pub rules: Rules,
    pub other_content: OtherContent,
}

impl Config {
    pub fn load(directory: &PathBuf) -> Result<Config, String> {
        const INDEX: &str = "index";
        const ITEM: &str = "item";

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

        let layout = get_hash("layout", config_map)?;
        let templates_yaml = get_hash("templates", &layout)?;
        let rules = get_hash("taxonomy_rules", &layout)?;

        let index_yaml = templates_yaml
            .get(&Yaml::from_str(INDEX))
            .ok_or_else(|| required_key(INDEX, templates_yaml))?;

        let item_yaml = templates_yaml
            .get(&Yaml::from_str(ITEM))
            .ok_or_else(|| required_key(ITEM, templates_yaml))?;

        let templates = IndexTemplates {
            index: Directories::path_buf_from_yaml(&index_yaml, "index", &config_path)?,
            item: Directories::path_buf_from_yaml(&item_yaml, "index", &config_path)?,
        };

        let other_content_yaml = get_hash("other_content", &layout)?;

        let other_content = OtherContent {
            copy_paths: Self::get_other_content("copy", &other_content_yaml)?,
            exclude: Self::get_other_content("exclude", &other_content_yaml)?,
        };

        Ok(Config {
            site: Self::parse_site_meta(config_map)?,
            directories: Directories::from_yaml(config_map, &config_path, &layout)?,
            taxonomies: Self::parse_taxonomies(&layout, &config_path)?,
            templates,
            rules: Self::parse_rules(&rules)?,
            other_content,
        })
    }

    fn get_other_content(key: &str, map: &BTreeMap<Yaml, Yaml>) -> Result<Vec<PathBuf>, String> {
        match map.get(&Yaml::from_str(key)) {
            None | Some(Yaml::Null) => Ok(Vec::new()),
            Some(Yaml::String(ref value)) => Ok(vec![value.into()]),
            Some(Yaml::Array(value)) => Ok(value
                .iter()
                .map(|v| PathBuf::from(v.as_str().unwrap()))
                .collect()),
            _ => Err(key_of_type(key, Required::No, map, "string")),
        }
    }

    /// Load the site data from the configuration file.
    fn parse_site_meta(config_map: &BTreeMap<Yaml, Yaml>) -> Result<SiteInfo, String> {
        const SITE_INFO: &str = "site_info";
        let site_info_yaml = get_hash(SITE_INFO, config_map)?;
        SiteInfo::from_yaml(&site_info_yaml)
    }

    /// Load the taxonomies from the configuration file.
    fn parse_taxonomies(
        structure: &BTreeMap<Yaml, Yaml>,
        config_path: &PathBuf,
    ) -> Result<Taxonomies, String> {
        const TAXONOMIES: &str = "taxonomies";

        let taxonomies_yaml = get_hash(TAXONOMIES, structure)?;

        let mut taxonomies = Taxonomies::new();
        if taxonomies_yaml.is_empty() {
            return Ok(taxonomies);
        }

        for name in taxonomies_yaml.keys() {
            let key = name.as_str().expect("If this isn't here, YAML is broken.");
            let content = get_hash(key, taxonomies_yaml)?;
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
            fields_req: Vec::new(),
            fields_opt: Vec::new(),
            templates: Templates {
                item: "author.html".into(),
                list: Some("authors.html".into()),
                feed_item: Some("feed_template.rss".into()),
                feed_list: Some("feed_template.rss".into()),
            },
        };
        taxonomies.insert("author".into(), tax_author);

        let tax_category = Taxonomy::TagLike {
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
                feed_item: Some("feed_template.rss".into()),
                feed_list: Some("feed_template.rss".into()),
            },
        };
        taxonomies.insert("category".into(), tax_category);

        let tax_tag = Taxonomy::TagLike {
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
        taxonomies.insert("tag".into(), tax_tag);

        let tax_date = Taxonomy::Temporal {
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
        taxonomies.insert("date".into(), tax_date);

        let tax_page = Taxonomy::Boolean {
            name: "page".into(),
            templates: Templates {
                item: "page.html".into(),
                list: None,
                feed_item: None,
                feed_list: None,
            },
        };
        taxonomies.insert("page".into(), tax_page);

        let tax_series = Taxonomy::TagLike {
            name: "series".into(),
            default: None,
            limit: Some(1),
            required: false,
            hierarchical: false,
            fields_req: vec!["name".into(), "part".into()],
            fields_opt: Vec::new(),
            templates: Templates {
                item: "series.html".into(),
                list: Some("series-list.html".into()),
                feed_item: None,
                feed_list: None,
            },
        };
        taxonomies.insert("series".into(), tax_series);

        let templates = IndexTemplates {
            index: PathBuf::from("index.html"),
            item: PathBuf::from("item.html"),
        };

        let other_content = OtherContent {
            copy_paths: vec!["static".into(), "extra".into()],
            exclude: Vec::new(),
        };

        let expected = Config {
            site,
            directories,
            taxonomies,
            templates,
            rules: Rules {
                commas_as_lists: true,
            },
            other_content,
        };

        assert_eq!(expected, config);
    }
}
