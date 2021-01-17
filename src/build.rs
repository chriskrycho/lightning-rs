use std::path::PathBuf;

use json5;

use crate::config::Config;
use crate::page::source::Source;
use crate::page::Page;

pub fn build(in_dir: PathBuf) -> Result<(), String> {
    let data_file = in_dir.join("_data/config.json5");
    let data = std::fs::read_to_string(&data_file)
        .map_err(|e| format!("could not read '{}'\n{}", &data_file.to_string_lossy(), e))?;
    let config: Config = json5::from_str(&data)
        .map_err(|e| format!("could not parse '{}':\n{}", &data_file.display(), e))?;

    let content_dir = in_dir.join("content");
    let content_glob = content_dir.to_string_lossy() + "/**/*.md";
    let all_contents = glob::glob(&content_glob)
        .expect(&format!("bad glob: '{}'", &content_glob))
        .map(|result| {
            result
                .map_err(|e| format!("{}", e))
                .and_then(|path| {
                    std::fs::read_to_string(&path)
                        .map(|contents| Source { path, contents })
                        .map_err(|e| format!("{}", e))
                })
                .and_then(|source| Page::new(source, &config))
        })
        .collect::<Vec<Result<Page, String>>>();

    let page = all_contents
        .into_iter()
        .nth(25)
        .expect("srsly tho")
        .expect("PLS");

    dbg!(page);
    Ok(())
}
