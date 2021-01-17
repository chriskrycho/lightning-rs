use std::path::PathBuf;

use rayon::prelude::*;
use syntect::parsing::SyntaxSet;

use crate::config::Config;
use crate::page::source::Source;
use crate::page::Page;

pub fn build(in_dir: PathBuf) -> Result<(), String> {
    let data_file = in_dir.join("_data/config.json5");
    let data = std::fs::read_to_string(&data_file)
        .map_err(|e| format!("could not read '{}'\n{}", &data_file.to_string_lossy(), e))?;
    let config: Config = json5::from_str(&data)
        .map_err(|e| format!("could not parse '{}':\n{}", &data_file.display(), e))?;

    let syntax_set = load_syntaxes();

    let all_contents = get_files_to_load(in_dir)
        .into_par_iter()
        .map(|path| {
            std::fs::read_to_string(&path)
                .map(|contents| Source { path, contents })
                .map_err(|e| format!("{}", e))
        })
        .map(|result| result.and_then(|source| Page::new(source, &config, &syntax_set)))
        .collect::<Vec<Result<Page, String>>>();

    let page = all_contents
        .into_iter()
        .find(|result| match result {
            Ok(page) => page.contents.contains("lightweight reactivity system"),
            _ => false,
        })
        .expect("srsly tho")
        .expect("PLS");

    std::fs::write("/Users/chris/Desktop/cool.html", page.contents).expect("YEAH");
    Ok(())
}

fn get_files_to_load(in_dir: PathBuf) -> Vec<PathBuf> {
    let content_dir = in_dir.join("content");
    let content_glob = content_dir.to_string_lossy() + "/**/*.md";

    let (ok_files, err_files): (Vec<PathBuf>, Vec<String>) = glob::glob(&content_glob)
        .unwrap_or_else(|_| panic!("bad glob: '{}'", &content_glob))
        .fold((vec![], vec![]), |(mut good, mut bad), result| {
            match result {
                Ok(path) => good.push(path),
                Err(e) => bad.push(e.to_string()),
            };

            (good, bad)
        });

    for err in err_files {
        eprintln!("problem with {}", err);
    }

    ok_files
}

fn load_syntaxes() -> SyntaxSet {
    // let mut extra_syntaxes_dir = std::env::current_dir().map_err(|e| format!("{}", e))?;
    // extra_syntaxes_dir.push("syntaxes");

    let syntax_builder = SyntaxSet::load_defaults_newlines().into_builder();
    // let mut syntax_builder = SyntaxSet::load_defaults_newlines().into_builder();
    // syntax_builder
    //     .add_from_folder(&extra_syntaxes_dir, false)
    //     .map_err(|e| format!("could not load {}: {}", &extra_syntaxes_dir.display(), e))?;

    syntax_builder.build()
}
