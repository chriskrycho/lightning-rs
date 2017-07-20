//! Process configurations for Lightning sites.

pub mod site_info;
pub mod taxonomy;

// First-party
use std::collections::{BTreeMap, HashMap};
use std::convert::From;
use std::error::Error;
use std::io::Read;
use std::fs::File;
use std::path::PathBuf;

// Third-party
use yaml_rust::{yaml, Yaml, YamlLoader};

// First-party
use validated_types::Url as ValidatedUrl;
use yaml_util::*;

use self::site_info::*;
use self::taxonomy::*;


const CONFIG_FILE_NAME: &str = "lightning.yaml";


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
            return Err(format!(
                "The specified configuration path {:?} does not exist.",
                config_path.to_string_lossy()
            ));
        }

        let mut file = File::open(&config_path)
            .map_err(|reason| {
                format!("Error reading {:?}: {:?}", config_path, reason)
            })?;

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

        let structure = Self::get_structure(config_map)?;

        Ok(Config {
            site: Self::parse_site_meta(config_map)?,
            directories: Directories::from_yaml(config_map, &config_path, &structure)?,
            taxonomies: Self::parse_taxonomies(&structure, &config_path)?,
        })
    }

    fn get_structure<'map>(
        config_map: &'map BTreeMap<Yaml, Yaml>,
    ) -> Result<&'map BTreeMap<Yaml, Yaml>, String> {
        const STRUCTURE: &str = "structure";
        config_map
            .get(&Yaml::from_str(STRUCTURE))
            .ok_or(required_key(STRUCTURE, config_map))?
            .as_hash()
            .ok_or(key_of_type(STRUCTURE, Required::Yes, config_map, "hash"))
    }


    /// Load the site data from the configuration file.
    fn parse_site_meta(config_map: &BTreeMap<Yaml, Yaml>) -> Result<SiteInfo, String> {
        const SITE_INFO: &str = "site_info";
        let site_info_yaml = config_map
            .get(&Yaml::from_str(SITE_INFO))
            .ok_or(required_key(SITE_INFO, config_map))?
            .as_hash()
            .ok_or(key_of_type(SITE_INFO, Required::Yes, config_map, "hash"))?;

        SiteInfo::from_yaml(&site_info_yaml)
    }


    /// Load the taxonomies from the configuration file.
    fn parse_taxonomies(
        structure: &BTreeMap<Yaml, Yaml>,
        config_path: &PathBuf,
    ) -> Result<HashMap<String, Taxonomy>, String> {
        const TAXONOMIES: &str = "taxonomies";

        let taxonomies_yaml = structure
            .get(&Yaml::from_str(TAXONOMIES))
            .ok_or(format!("No `{}` key in {:?}", TAXONOMIES, config_path))?
            .as_hash()
            .ok_or(format!(
                "`{}` is not a hash in {:?}",
                TAXONOMIES,
                config_path
            ))?;

        let mut taxonomies = HashMap::new();
        if taxonomies_yaml.len() == 0 {
            return Ok(taxonomies);
        }

        for name in taxonomies_yaml.keys() {
            let key = name.as_str().expect("If this isn't here, YAML is broken.");
            let content = taxonomies_yaml
                .get(name)
                .ok_or(required_key(key, taxonomies_yaml))?
                .as_hash()
                .ok_or(key_of_type(key, Required::Yes, taxonomies_yaml, "hash"))?;
            let taxonomy = Taxonomy::from_yaml(content, key)?;
            if taxonomies.insert(key.into(), taxonomy).is_none() {
                return Err(format!("duplicate key {}", key));
            }
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
    fn from_yaml(
        config_map: &BTreeMap<Yaml, Yaml>,
        config_path: &PathBuf,
        structure: &BTreeMap<Yaml, Yaml>,
    ) -> Result<Directories, String> {
        const CONTENT_DIRECTORY: &str = "content_directory";
        const OUTPUT_DIRECTORY: &str = "output_directory";
        const TEMPLATE_DIRECTORY: &str = "directory";

        let content_directory_yaml = config_map
            .get(&Yaml::from_str(CONTENT_DIRECTORY))
            .ok_or(required_key(CONTENT_DIRECTORY, config_map))?;

        let content_directory = Directories::path_buf_from_yaml(
            &content_directory_yaml,
            CONTENT_DIRECTORY,
            &config_path,
        )?;

        let output_directory_yaml = config_map
            .get(&Yaml::from_str(OUTPUT_DIRECTORY))
            .ok_or(required_key(OUTPUT_DIRECTORY, config_map))?;

        let output_directory =
            Directories::path_buf_from_yaml(output_directory_yaml, OUTPUT_DIRECTORY, &config_path)?;

        let template_directory_yaml = structure
            .get(&Yaml::from_str(TEMPLATE_DIRECTORY))
            .ok_or(
                required_key(TEMPLATE_DIRECTORY, structure) + &format!(" in {:?}", config_path),
            )?;

        let template_directory = Directories::path_buf_from_yaml(
            &template_directory_yaml,
            TEMPLATE_DIRECTORY,
            &config_path,
        )?;

        Ok(Directories {
            content: content_directory,
            output: output_directory,
            template: template_directory,
        })
    }

    fn path_buf_from_yaml(
        yaml: &Yaml,
        key: &str,
        config_path: &PathBuf,
    ) -> Result<PathBuf, String> {
        match yaml {
            &Yaml::String(ref path_str) => Ok(PathBuf::from(path_str)),
            value => Err(
                bad_value(value, key, yaml) + &format!(" in {:?}", config_path),
            ),
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_full_config() {
        unimplemented!();
    }
}
