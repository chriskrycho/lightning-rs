use std::path::PathBuf;

use json5;

use crate::config::Config;

pub fn build(in_dir: PathBuf) -> Result<(), String> {
    let data_file = in_dir.join("_data/config.json5");
    let data = std::fs::read_to_string(&data_file)
        .map_err(|e| format!("could not read '{}'\n{}", &data_file.to_string_lossy(), e))?;
    let config: Config = json5::from_str(&data)
        .map_err(|e| format!("could not parse '{}':\n{}", &data_file.display(), e))?;

    let content_dir = in_dir.join("content");
    let content_glob = content_dir.to_string_lossy() + "/**/*.md";
    let all_contents: Vec<Result<(PathBuf, String), String>> = glob::glob(&content_glob)
        .expect(&format!("bad glob: '{}'", &content_glob))
        .map(|result| {
            result.map_err(|e| format!("{}", e)).and_then(|file| {
                std::fs::read_to_string(&file)
                    .map(|content| (file, content))
                    .map_err(|e| format!("{}", e))
            })
        })
        .collect();

    dbg!("{}", &all_contents[0]);
    Ok(())
}
