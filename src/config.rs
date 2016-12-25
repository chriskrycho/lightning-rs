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
use yaml_util::*;


const CONFIG_FILE_NAME: &'static str = "lightning.yaml";


pub struct Config {
    pub site: Site,
    pub directories: Directories,
    pub taxonomies: Vec<Taxonomy>,
}


pub struct Directories {
    pub content: PathBuf,
    pub output: PathBuf,
    pub template: PathBuf,
}


impl Directories {
    fn from_yaml(config_map: &BTreeMap<Yaml, Yaml>,
                 config_path: &PathBuf)
                 -> Result<Directories, String> {
        const CONTENT_DIRECTORY: &'static str = "content_directory";
        const OUTPUT_DIRECTORY: &'static str = "output_directory";
        const TEMPLATE_DIRECTORY: &'static str = "directory";

        let content_directory_yaml = config_map.get(&Yaml::from_str(CONTENT_DIRECTORY))
            .ok_or(format!("No `{:}` key in {:?}", CONTENT_DIRECTORY, config_path))?;

        let content_directory =
            path_buf_from_yaml(&content_directory_yaml, CONTENT_DIRECTORY, &config_path)?;

        let output_directory_yaml = config_map.get(&Yaml::from_str(OUTPUT_DIRECTORY))
            .ok_or(format!("No `{:} key in `{:?}", OUTPUT_DIRECTORY, config_path))?;

        let output_directory =
            path_buf_from_yaml(output_directory_yaml, OUTPUT_DIRECTORY, &config_path)?;

        let structure = get_structure(&config_map, &config_path)?;

        let template_directory_yaml = structure.get(&Yaml::from_str(TEMPLATE_DIRECTORY))
            .ok_or(format!("No `directory` key in `structure` in {:?}", config_path))?;

        let template_directory =
            path_buf_from_yaml(&template_directory_yaml, TEMPLATE_DIRECTORY, &config_path)?;

        Ok(Directories {
            content: content_directory,
            output: output_directory,
            template: template_directory,
        })
    }
}


pub enum Taxonomy {
    Binary { name: String, templates: Templates },
    Multiple {
        name: String,
        templates: Templates,
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
    // TODO: make Result<Taxonomy, Vec<String>> instead? Collect errors to
    //       supply *all* validation errors to the user? And generalize that to
    //       all types?
    fn from_yaml_hash(hash: &yaml::Hash, name: &str) -> Result<Taxonomy, String> {
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
                })
            }
            MULTIPLE => {
                Ok(Taxonomy::Multiple {
                    name: name,
                    templates: templates,
                    hierarchical: Self::is_hierarchical(hash)?,
                    required: Self::is_required(hash)?,
                    limit: Self::limit(hash)?,
                })
            }
            TEMPORAL => {
                Ok(Taxonomy::Temporal {
                    name: name,
                    templates: templates,
                    required: Self::is_required(hash)?
                })
            }
            _ => Err(format!("Invalid taxonomy type `{:?}` in {:?}", taxonomy_type, hash)),
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
            None  => Ok(None),
            Some(&Yaml::Integer(i)) if i < 0 => Err(bad_value(i, LIMIT, hash)),
            Some(&Yaml::Integer(i)) if i == 0 => Ok(None),
            Some(&Yaml::Integer(i)) if i > 0 && i < max => Ok(Some(i as u8)),
            Some(&Yaml::Integer(i)) if i > max as i64 => Err(ridiculous_number(i, LIMIT, hash)),
            _ => Err(key_of_type(LIMIT, Required::No, hash, "integer")),
        }
    }
}


pub struct Site {
    pub name: String,
    pub description: String,
    pub metadata: HashMap<Yaml, Yaml>,
    pub url: ValidatedUrl,
}


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


mod validated {
    pub struct Url(String);

    impl Url {
        /// Get a URL. `Err` if the item passed in is not a spec-conformant URL.
        pub fn new(unvalidated_url: String) -> Result<Url, String> {
            // TODO: validate the URLs!
            Ok(Url(unvalidated_url))
        }

        pub fn value(&self) -> String {
            self.0.clone()
        }
    }
}

pub use self::validated::Url as ValidatedUrl;


fn path_buf_from_yaml(yaml: &Yaml, key: &str, config_path: &PathBuf) -> Result<PathBuf, String> {
    match yaml {
        &Yaml::String(ref path_str) => Ok(PathBuf::from(path_str)),
        value => Err(format!("invalid `{:}` value {:?} in {:?}", key, value, config_path)),
    }
}


fn get_structure<'map>(config_map: &'map BTreeMap<Yaml, Yaml>,
                       config_path: &PathBuf)
                       -> Result<&'map BTreeMap<Yaml, Yaml>, String> {
    config_map.get(&Yaml::from_str("structure"))
        .ok_or(format!("No `structure` key in {:?}", config_path))?
        .as_hash()
        .ok_or(format!("`structure` is not a map in {:?}", config_path))
}


/// Load the site data from the configuration file.
fn site(config_map: &BTreeMap<Yaml, Yaml>) -> Result<Site, String> {
    // TODO: build these:
    let name = String::new();
    let description = String::new();
    let metadata = HashMap::new();
    let url = ValidatedUrl::new(String::new())?;

    Ok(Site {
        name: name,
        description: description,
        metadata: metadata,
        url: url,
    })
}


fn taxonomies(config_map: &BTreeMap<Yaml, Yaml>,
              config_path: &PathBuf)
              -> Result<Vec<Taxonomy>, String> {
    const KEY: &'static str = "taxonomies";

    let structure = get_structure(config_map, config_path)?;
    let taxonomies_yaml = structure.get(&Yaml::from_str(KEY))
        .ok_or(format!("No `{}` key in {:?}", KEY, config_path))?
        .as_vec()
        .ok_or(format!("`{}` is not an array in {:?}", KEY, config_path))?;

    if taxonomies_yaml.len() == 0 {
        return Ok(Vec::new());
    }

    // TODO: actually get all the keys instead of just doing it with the first one.
    // This is safe because we have at least one; also it's temporary. We'll
    // extract all of this to be a function which can be used as an argument to
    // part of a map: Vec<Yaml> -> Vec<Taxonomy>
    let first = taxonomies_yaml.first()
        .unwrap()
        .as_hash()
        .ok_or(format!("Cannot expand `{}` item as a hash", KEY))?;

    let first_key =
        first.keys().next().ok_or(key_of_type("first key", Required::Yes, first, "hash"))?;
    let first_key_string = first_key.as_str().expect(":wat:");  // FIXME: this is dumb.
    let first_yaml_hash = first.get(first_key)
        .ok_or(required_key(first_key_string, first))?
        .as_hash()
        .ok_or(key_of_type(first_key_string, Required::Yes, first, "hash"))?;
    let first_taxonomy = Taxonomy::from_yaml_hash(first_yaml_hash, first_key_string)?;
    let taxonomy_data: Vec<Taxonomy> = vec![first_taxonomy];

    Ok(taxonomy_data)
}


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

    Ok(Config {
        site: site(config_map)?,
        directories: Directories::from_yaml(config_map, &config_path)?,
        taxonomies: taxonomies(config_map, &config_path)?,
    })
}
