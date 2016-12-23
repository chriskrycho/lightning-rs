//! Process configurations for Lightning sites.


// First-party
use std::error::Error;
use std::io::Read;
use std::fs::File;
use std::path::PathBuf;

// Third-party
use yaml_rust::{Yaml, YamlLoader};


const CONFIG_FILE_NAME: &'static str = "lightning.yaml";
const CONTENT_DIRECTORY: &'static str = "content_directory";
const TEMPLATE_DIRECTORY: &'static str = "directory";


pub struct Config {
    pub content_directory: PathBuf,
    pub template_directory: PathBuf,
}


pub enum Taxonomy {
    Multiple {
        name: String,
        limit: Option<u8>,
        required: bool,
        hierarchical: bool,
    },
    Binary,
    Temporal { required: bool },
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
    let yaml_config = YamlLoader::load_from_str(&contents)
        .map_err(|err| format!("{} ({:?})", err, &config_path))?;
    let yaml_config = yaml_config.into_iter().next().ok_or("Empty configuration file")?;
    let yaml_config = yaml_config.as_hash().ok_or("Configuration is not a map")?;

    let content_directory_yaml = yaml_config.get(&Yaml::from_str(CONTENT_DIRECTORY))
        .ok_or(format!("No `{:}` key in {:?}", CONTENT_DIRECTORY, config_path))?;

    fn path_buf_from_yaml(yaml: &Yaml,
                          key: &str,
                          config_path: &PathBuf)
                          -> Result<PathBuf, String> {
        match yaml {
            &Yaml::String(ref path_str) => Ok(PathBuf::from(path_str)),
            value => Err(format!("invalid `{:}` value {:?} in {:?}", key, value, config_path)),
        }
    }

    let content_directory =
        path_buf_from_yaml(&content_directory_yaml, CONTENT_DIRECTORY, &config_path)?;

    let structure = yaml_config.get(&Yaml::from_str("structure"))
        .ok_or(format!("No `structure` key in {:?}", config_path))?
        .as_hash()
        .ok_or(format!("`structure` is not a map in {:?}", config_path))?;

    let template_directory_yaml = structure.get(&Yaml::from_str(TEMPLATE_DIRECTORY))
        .ok_or(format!("No `directory` key in `structure` in {:?}", config_path))?;

    let template_directory =
        path_buf_from_yaml(&template_directory_yaml, TEMPLATE_DIRECTORY, &config_path)?;

    Ok(Config {
        content_directory: content_directory,
        template_directory: template_directory,
    })
}
