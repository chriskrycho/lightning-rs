mod email;

use std::path::{Path, PathBuf};

use serde_derive::Deserialize;

use email::Email;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub url: String,
    pub repo: String,
    pub title: Title,
    pub subtitle: String,
    pub description: String,
    pub author: Author,
    pub output: PathBuf,
}

impl Config {
    pub fn from_file(path: &Path) -> Result<Config, String> {
        let data = std::fs::read_to_string(path)
            .map_err(|e| format!("could not read '{}'\n{}", &path.to_string_lossy(), e))?;
        let mut config: Config = json5::from_str(&data)
            .map_err(|e| format!("could not parse '{}':\n{}", &path.display(), e))?;

        config.output = std::fs::canonicalize(
            path.parent()
                .ok_or_else(|| String::from("config file will have a parent dir"))?
                .join(config.output),
        )
        .map_err(|e| e.to_string())?;

        Ok(config)
    }
}

#[derive(Deserialize, Debug)]
pub struct Title {
    normal: String,
    stylized: String,
}

#[derive(Deserialize, Debug)]
pub struct Author {
    name: String,
    #[serde(deserialize_with = "Email::de_from_str")]
    email: Email,
    links: Vec<String>,
}
