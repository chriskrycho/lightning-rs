//! Configuration directories

// Standard library dependencies
use std::collections::BTreeMap;
use std::path::PathBuf;

// Third party
use yaml_rust::Yaml;

// First party
use yaml_util::*;

#[derive(Debug, PartialEq)]
pub struct Directories {
    pub content: PathBuf,
    pub output: PathBuf,
    pub template: PathBuf,
}

impl Directories {
    pub fn from_yaml(
        config_map: &BTreeMap<Yaml, Yaml>,
        config_path: &PathBuf,
        structure: &BTreeMap<Yaml, Yaml>,
    ) -> Result<Directories, String> {
        const CONTENT_DIRECTORY: &str = "content_directory";
        const OUTPUT_DIRECTORY: &str = "output_directory";
        const TEMPLATE_DIRECTORY: &str = "directory";

        let content_directory_yaml = config_map
            .get(&Yaml::from_str(CONTENT_DIRECTORY))
            .ok_or_else(|| required_key(CONTENT_DIRECTORY, config_map))?;

        let content_directory = Directories::path_buf_from_yaml(
            &content_directory_yaml,
            CONTENT_DIRECTORY,
            &config_path,
        )?;

        let output_directory_yaml = config_map
            .get(&Yaml::from_str(OUTPUT_DIRECTORY))
            .ok_or_else(|| required_key(OUTPUT_DIRECTORY, config_map))?;

        let output_directory =
            Directories::path_buf_from_yaml(output_directory_yaml, OUTPUT_DIRECTORY, &config_path)?;

        let template_directory_yaml = structure.get(&Yaml::from_str(TEMPLATE_DIRECTORY)).ok_or(
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

    pub fn path_buf_from_yaml(
        yaml: &Yaml,
        key: &str,
        config_path: &PathBuf,
    ) -> Result<PathBuf, String> {
        match yaml {
            &Yaml::String(ref path_str) => Ok(PathBuf::from(path_str)),
            value => Err(bad_value(value, key, yaml) + &format!(" in {:?}", config_path)),
        }
    }
}
