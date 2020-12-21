use std::{fmt::format, path::PathBuf};

use json5;

use crate::config::Config;

pub fn build(in_dir: PathBuf) -> Result<(), String> {
    let data_file = in_dir.join("_data/config.json5");
    let data = std::fs::read_to_string(&data_file)
        .map_err(|e| format!("could not read '{}'\n{}", &data_file.to_string_lossy(), e))?;
    let config: Config = json5::from_str(&data)
        .map_err(|e| format!("could not parse '{}':\n{}", &data_file.display(), e))?;

    dbg!("{}", config);
    Ok(())
}
