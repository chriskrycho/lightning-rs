//! Process configurations for Lightning sites.


// First-party
use std::collections::{BTreeMap, HashMap};
use std::convert::From;
use std::error::Error;
use std::io::Read;
use std::fs::File;
use std::path::PathBuf;
use std::u8;

// Third-party
use yaml_rust::{yaml, Yaml, YamlLoader};

// First-party
pub use validated_types::Url as ValidatedUrl;
use yaml_util::*;


const CONFIG_FILE_NAME: &'static str = "lightning.yaml";


#[derive(Debug, PartialEq)]
pub struct Config {
    pub site: SiteInfo,
    pub directories: Directories,
    pub taxonomies: Vec<Taxonomy>,
}


impl Config {
    pub fn load(directory: &PathBuf) -> Result<Config, String> {
        let config_path = directory.join(CONFIG_FILE_NAME);
        if !config_path.exists() {
            return Err(format!("The specified configuration path {:?} does not exist.",
                               config_path.to_string_lossy()));
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
        let yaml_config = load_result.into_iter().next().ok_or("Empty configuration file")?;
        let config_map = yaml_config.as_hash().ok_or("Configuration is not a map")?;

        let structure = Self::get_structure(config_map)?;

        Ok(Config {
            site: Self::parse_site_meta(config_map)?,
            directories: Directories::from_yaml(config_map, &config_path, &structure)?,
            taxonomies: Self::parse_taxonomies(&structure, &config_path)?,
        })
    }

    fn get_structure<'map>(config_map: &'map BTreeMap<Yaml, Yaml>)
                           -> Result<&'map BTreeMap<Yaml, Yaml>, String> {
        const STRUCTURE: &'static str = "structure";
        config_map.get(&Yaml::from_str(STRUCTURE))
            .ok_or(required_key(STRUCTURE, config_map))?
            .as_hash()
            .ok_or(key_of_type(STRUCTURE, Required::Yes, config_map, "hash"))
    }


    /// Load the site data from the configuration file.
    fn parse_site_meta(config_map: &BTreeMap<Yaml, Yaml>) -> Result<SiteInfo, String> {
        const SITE_INFO: &'static str = "site_info";
        let site_info_yaml = config_map.get(&Yaml::from_str(SITE_INFO))
            .ok_or(required_key(SITE_INFO, config_map))?
            .as_hash()
            .ok_or(key_of_type(SITE_INFO, Required::Yes, config_map, "hash"))?;

        SiteInfo::from_yaml(&site_info_yaml)
    }


    /// Load the taxonomies from the configuration file.
    fn parse_taxonomies(structure: &BTreeMap<Yaml, Yaml>,
                        config_path: &PathBuf)
                        -> Result<Vec<Taxonomy>, String> {
        const KEY: &'static str = "taxonomies";

        let taxonomies_yaml = structure.get(&Yaml::from_str(KEY))
            .ok_or(format!("No `{}` key in {:?}", KEY, config_path))?
            .as_vec()
            .ok_or(format!("`{}` is not an array in {:?}", KEY, config_path))?;

        let mut taxonomies = Vec::new();
        if taxonomies_yaml.len() == 0 {
            return Ok(taxonomies);
        }

        for taxonomy_yaml in taxonomies_yaml {
            let wrapper = taxonomy_yaml.as_hash()
                .ok_or(key_of_type(KEY, Required::Yes, taxonomy_yaml, "hash"))?;
            let key = wrapper.keys()
                .next()
                .ok_or(key_of_type("first key", Required::Yes, wrapper, "hash"))?;
            let key_string = key.as_str()
                .ok_or(key_of_type("first key name", Required::Yes, wrapper, "string"))?;
            let content = wrapper.get(key)
                .ok_or(required_key(key_string, wrapper))?
                .as_hash()
                .ok_or(key_of_type(key_string, Required::Yes, wrapper, "hash"))?;
            let taxonomy = Taxonomy::from_yaml(content, key_string)?;
            taxonomies.push(taxonomy);
        }

        Ok(taxonomies)
    }
}


#[derive(Debug, PartialEq)]
pub struct Directories {
    pub content: PathBuf,
    pub output: PathBuf,
    pub template: PathBuf,
}


impl Directories {
    fn from_yaml(config_map: &BTreeMap<Yaml, Yaml>,
                 config_path: &PathBuf,
                 structure: &BTreeMap<Yaml, Yaml>)
                 -> Result<Directories, String> {
        const CONTENT_DIRECTORY: &'static str = "content_directory";
        const OUTPUT_DIRECTORY: &'static str = "output_directory";
        const TEMPLATE_DIRECTORY: &'static str = "directory";

        let content_directory_yaml = config_map.get(&Yaml::from_str(CONTENT_DIRECTORY))
            .ok_or(required_key(CONTENT_DIRECTORY, config_map))?;

        let content_directory =
            Self::path_buf_from_yaml(&content_directory_yaml, CONTENT_DIRECTORY, &config_path)?;

        let output_directory_yaml = config_map.get(&Yaml::from_str(OUTPUT_DIRECTORY))
            .ok_or(required_key(OUTPUT_DIRECTORY, config_map))?;

        let output_directory =
            Self::path_buf_from_yaml(output_directory_yaml, OUTPUT_DIRECTORY, &config_path)?;

        let template_directory_yaml =
            structure.get(&Yaml::from_str(TEMPLATE_DIRECTORY))
                .ok_or(required_key(TEMPLATE_DIRECTORY, structure) +
                       &format!(" in {:?}", config_path))?;

        let template_directory =
            Self::path_buf_from_yaml(&template_directory_yaml, TEMPLATE_DIRECTORY, &config_path)?;

        Ok(Directories {
            content: content_directory,
            output: output_directory,
            template: template_directory,
        })
    }

    fn path_buf_from_yaml(yaml: &Yaml,
                          key: &str,
                          config_path: &PathBuf)
                          -> Result<PathBuf, String> {
        match yaml {
            &Yaml::String(ref path_str) => Ok(PathBuf::from(path_str)),
            value => Err(bad_value(value, key, yaml) + &format!(" in {:?}", config_path)),
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum Taxonomy {
    Binary {
        name: String,
        templates: Templates,
        hierarchical: bool,
    },
    Multiple {
        name: String,
        templates: Templates,
        default: Option<String>,
        limit: Option<u8>,
        required: bool,
        hierarchical: bool,
    },
    Temporal {
        name: String,
        templates: Templates,
        required: bool,
    },
}


impl Taxonomy {
    fn from_yaml(hash: &yaml::Hash, name: &str) -> Result<Taxonomy, String> {
        const TYPE: &'static str = "type";
        const BINARY: &'static str = "binary";
        const MULTIPLE: &'static str = "multiple";
        const TEMPORAL: &'static str = "temporal";

        let name = String::from(name);

        // Name can't collide with keyword `type`.
        let taxonomy_type = hash.get(&Yaml::from_str(TYPE))
            .ok_or(required_key(TYPE, hash))?
            .as_str()
            .ok_or(key_of_type(TYPE, Required::Yes, hash, "string"))?;

        let templates = Templates::from_yaml(hash)?;

        match taxonomy_type {
            BINARY => {
                Ok(Taxonomy::Binary {
                    name: name,
                    templates: templates,
                    hierarchical: Self::is_hierarchical(hash)?,
                })
            }
            MULTIPLE => {
                Ok(Taxonomy::Multiple {
                    name: name,
                    templates: templates,
                    default: Self::default_value(hash)?,
                    hierarchical: Self::is_hierarchical(hash)?,
                    required: Self::is_required(hash)?,
                    limit: Self::limit(hash)?,
                })
            }
            TEMPORAL => {
                Ok(Taxonomy::Temporal {
                    name: name,
                    templates: templates,
                    required: Self::is_required(hash)?,
                })
            }
            _ => Err(format!("Invalid taxonomy type `{:?}` in {:?}", taxonomy_type, hash)),
        }
    }

    fn default_value(hash: &yaml::Hash) -> Result<Option<String>, String> {
        const DEFAULT: &'static str = "default";

        match hash.get(&Yaml::from_str(DEFAULT)) {
            None |
            Some(&Yaml::Null) => Ok(None),
            Some(&Yaml::String(ref string)) => Ok(Some(string.clone())),
            _ => Err(key_of_type(DEFAULT, Required::No, hash, "string")),
        }
    }

    fn is_hierarchical(hash: &yaml::Hash) -> Result<bool, String> {
        const HIERARCHICAL: &'static str = "hierarchical";

        match hash.get(&Yaml::from_str(HIERARCHICAL)) {
            None |
            Some(&Yaml::Boolean(false)) => Ok(false),
            Some(&Yaml::Boolean(true)) => Ok(true),
            _ => Err(key_of_type(HIERARCHICAL, Required::Yes, hash, "bool")),
        }
    }

    fn is_required(hash: &yaml::Hash) -> Result<bool, String> {
        const REQUIRED: &'static str = "required";

        match hash.get(&Yaml::from_str(REQUIRED)) {
            None |
            Some(&Yaml::Boolean(false)) => Ok(false),
            Some(&Yaml::Boolean(true)) => Ok(true),
            _ => Err(key_of_type(REQUIRED, Required::No, hash, "bool")),
        }
    }

    fn limit(hash: &yaml::Hash) -> Result<Option<u8>, String> {
        const LIMIT: &'static str = "limit";
        let max = u8::MAX as i64;

        match hash.get(&Yaml::from_str(LIMIT)) {
            None |
            Some(&Yaml::Null) => Ok(None),
            Some(&Yaml::Integer(i)) if i < 0 => Err(bad_value(i, LIMIT, hash)),
            Some(&Yaml::Integer(i)) if i == 0 => Ok(None),
            Some(&Yaml::Integer(i)) if i > 0 && i < max => Ok(Some(i as u8)),
            Some(&Yaml::Integer(i)) if i > max as i64 => Err(ridiculous_number(i, LIMIT, hash)),
            _ => Err(key_of_type(LIMIT, Required::No, hash, "integer")),
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct SiteInfo {
    /// The name of the site. Required.
    pub title: String,

    /// The canonical URL for the root of the site. Required.
    pub url: ValidatedUrl,

    /// The description of the site. Optional.
    pub description: Option<String>,

    /// Arbitrary metadata associated with the site. Optional.
    pub metadata: HashMap<String, Yaml>,
}


impl SiteInfo {
    fn from_yaml(yaml: &yaml::Hash) -> Result<SiteInfo, String> {
        let title = Self::parse_title(yaml)?;
        let url = Self::parse_url(yaml)?;
        let description = Self::parse_description(yaml)?;
        let metadata = Self::parse_metadata(yaml)?;
        Ok(SiteInfo {
            title: title,
            url: url,
            description: description,
            metadata: metadata,
        })
    }

    fn parse_title(yaml: &yaml::Hash) -> Result<String, String> {
        const TITLE: &'static str = "title";
        match yaml[&Yaml::from_str(TITLE)] {
            Yaml::Null | Yaml::BadValue => Err(required_key(TITLE, yaml)),
            Yaml::String(ref string) => Ok(string.clone()),
            _ => Err(key_of_type(TITLE, Required::Yes, yaml, "string")),
        }
    }

    fn parse_url(yaml: &yaml::Hash) -> Result<ValidatedUrl, String> {
        const URL: &'static str = "url";
        match yaml[&Yaml::from_str(URL)] {
            Yaml::Null | Yaml::BadValue => Err(required_key(URL, yaml)),
            Yaml::String(ref string) => ValidatedUrl::new(&string),
            _ => Err(key_of_type(URL, Required::Yes, yaml, "string")),
        }
    }

    fn parse_description(yaml: &yaml::Hash) -> Result<Option<String>, String> {
        const DESCRIPTION: &'static str = "description";
        match yaml[&Yaml::from_str(DESCRIPTION)] {
            Yaml::Null | Yaml::BadValue => Ok(None),
            Yaml::String(ref string) => Ok(Some(string.clone())),
            _ => Err(key_of_type(DESCRIPTION, Required::No, yaml, "string")),
        }
    }

    fn parse_metadata(yaml: &yaml::Hash) -> Result<HashMap<String, Yaml>, String> {
        const METADATA: &'static str = "metadata";
        let mut metadata = HashMap::new();
        match yaml[&Yaml::from_str(METADATA)] {
            Yaml::Null | Yaml::BadValue => Ok(metadata),
            Yaml::Hash(ref hash) => {
                for key in hash.keys() {
                    println!("Key: {:?}\n", key);
                    let key_str = key.as_str()
                        .ok_or(key_of_type("key of hash map", Required::No, hash, "string"))?;

                    match hash[key] {
                        Yaml::Null | Yaml::BadValue => {
                            return Err(key_of_type(key_str, Required::No, hash, "hash"));
                        }
                        ref valid_item @ Yaml::String(..) |
                        ref valid_item @ Yaml::Boolean(..) |
                        ref valid_item @ Yaml::Integer(..) |
                        ref valid_item @ Yaml::Real(..) => {
                            let result = metadata.insert(String::from(key_str), valid_item.clone());
                            if result.is_some() {
                                let main = format!("Double insertion of key {}.\n", key_str);
                                let detail = format!("First: {:?}\nSecond: {:?}",
                                                     result.unwrap(),
                                                     valid_item);
                                return Err(main + &detail);
                            }
                        }
                        _ => {
                            return Err(key_of_type(key_str,
                                                   Required::No,
                                                   hash,
                                                   "string, boolean, or integer"))
                        }
                    }
                }
                Ok(metadata)
            }
            _ => Err(key_of_type(METADATA, Required::No, yaml, "hash")),
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct Templates {
    pub item: PathBuf,
    pub list: Option<PathBuf>,
}


impl Templates {
    fn from_yaml(yaml: &yaml::Hash) -> Result<Templates, String> {
        const TEMPLATES: &'static str = "templates";
        let template_yaml = yaml.get(&Yaml::from_str(TEMPLATES))
            .ok_or(required_key(TEMPLATES, yaml))?
            .as_hash()
            .ok_or(key_of_type(TEMPLATES, Required::Yes, yaml, "hash"))?;

        let item = Self::item_from_yaml(template_yaml)?;
        let list = Self::list_from_yaml(template_yaml)?;

        Ok(Templates {
            item: item,
            list: list,
        })
    }

    /// Get the `item` value for a taxonomy's templates.
    fn item_from_yaml(yaml: &yaml::Hash) -> Result<PathBuf, String> {
        const ITEM: &'static str = "item";

        let item_str = yaml.get(&Yaml::from_str(ITEM))
            .ok_or(required_key(ITEM, yaml))?
            .as_str()
            .ok_or(key_of_type(ITEM, Required::Yes, yaml, "string"))?;

        Ok(item_str.into())
    }

    /// Get the `list` value for a taxonomy's templates.
    ///
    /// This return type isn't as crazy as it looks. A `list` entry is allowed
    /// to be explicitly `null`/`~` or simply unset, but if the key is
    /// included, it is not allowed to be anything other than a `string` or
    /// explicitly set to null.
    fn list_from_yaml(yaml: &yaml::Hash) -> Result<Option<PathBuf>, String> {
        const LIST: &'static str = "list";

        match yaml.get(&Yaml::from_str(LIST)) {
            None |
            Some(&Yaml::Null) => Ok(None),
            Some(&Yaml::String(ref string)) => Ok(Some(string.into())),
            _ => Err(key_of_type(LIST, Required::No, yaml, "string")),
        }
    }
}


#[test]
fn parses_valid_taxonomies() {
    const TAXONOMIES: &'static str = "
structure:
  taxonomies:
    - author:
        type: multiple
        required: true
        hierarchical: false
        templates:
          list: authors.html
          item: author.html
    - category:
        type: multiple
        default: Blog
        limit: 1
        required: false
        hierarchical: false
        templates:
          list: categories.html
          item: category.html
    - tag:
        type: multiple
        limit: ~
        required: false
        hierarchical: false
        templates:
          list: tags.html
          item: tag.html
    - date:
        type: temporal
        required: false
        templates:
          list: period_archives.html
          item: archives.html
    - page:
        type: binary
        hierarchical: true
        templates:
          item: page.html
        ";

    let expected = vec![Taxonomy::Multiple {
                            name: "author".into(),
                            default: None,
                            limit: None,
                            required: true,
                            hierarchical: false,
                            templates: Templates {
                                item: "author.html".into(),
                                list: Some("authors.html".into()),
                            },
                        },
                        Taxonomy::Multiple {
                            name: "category".into(),
                            default: Some("Blog".into()),
                            limit: Some(1),
                            required: false,
                            hierarchical: false,
                            templates: Templates {
                                item: "category.html".into(),
                                list: Some("categories.html".into()),
                            },
                        },
                        Taxonomy::Multiple {
                            name: "tag".into(),
                            default: None,
                            limit: None,
                            required: false,
                            hierarchical: false,
                            templates: Templates {
                                item: "tag.html".into(),
                                list: Some("tags.html".into()),
                            },
                        },
                        Taxonomy::Temporal {
                            name: "date".into(),
                            required: false,
                            templates: Templates {
                                item: "archives.html".into(),
                                list: Some("period_archives.html".into()),
                            },
                        },
                        Taxonomy::Binary {
                            name: "page".into(),
                            hierarchical: true,
                            templates: Templates {
                                item: "page.html".into(),
                                list: None,
                            },
                        }];

    let mut loaded = YamlLoader::load_from_str(TAXONOMIES).unwrap();
    let first = loaded.pop().unwrap();
    let structure =
        first.as_hash().unwrap().get(&Yaml::from_str("structure")).unwrap().as_hash().unwrap();

    assert_eq!(Ok(expected),
               Config::parse_taxonomies(&structure, &"expected".into()));
}


#[test]
fn parses_site_info() {
    const SITE_INFO: &'static str = "\
site_info:
  title: lx (lightning)
  url: https://lightning.rs
  description: >
    A ridiculously fast site generator and engine.
  metadata:
    foo: bar
    quux: 2
    ";

    let mut metadata = HashMap::new();
    metadata.insert("foo".into(), Yaml::from_str("bar"));
    metadata.insert("quux".into(), Yaml::from_str("2"));
    let expected = SiteInfo {
        title: "lx (lightning)".into(),
        url: ValidatedUrl::new("https://lightning.rs").unwrap(),
        description: Some("A ridiculously fast site generator and engine.\n".into()),
        metadata: metadata,
    };

    let mut loaded = YamlLoader::load_from_str(SITE_INFO).unwrap();
    let first = loaded.pop().unwrap();
    let site_info = first.as_hash().unwrap()[&Yaml::from_str("site_info")].as_hash().unwrap();
    assert_eq!(Ok(expected), SiteInfo::from_yaml(&site_info));
}

#[test]
fn parses_site_info_with_empty_metadata() {
    const SITE_INFO_EMPTY_METADATA: &'static str = "
site_info:
  title: lx (lightning)
  url: https://lightning.rs
  description: >
    A ridiculously fast site generator and engine.
  metadata: ~
    ";

    let expected = SiteInfo {
        title: "lx (lightning)".into(),
        url: ValidatedUrl::new("https://lightning.rs").unwrap(),
        description: Some("A ridiculously fast site generator and engine.\n".into()),
        metadata: HashMap::new(),
    };

    let mut loaded = YamlLoader::load_from_str(SITE_INFO_EMPTY_METADATA).unwrap();
    let first = loaded.pop().unwrap();
    let site_info = first.as_hash().unwrap()[&Yaml::from_str("site_info")].as_hash().unwrap();
    assert_eq!(Ok(expected), SiteInfo::from_yaml(&site_info));
}
