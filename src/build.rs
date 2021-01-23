use std::path::PathBuf;

use rayon::prelude::*;
use syntect::parsing::SyntaxSet;

use crate::config::Config;
use crate::page::source::Source;
use crate::page::Page;

pub fn build(in_dir: PathBuf) -> Result<(), String> {
    let in_dir = std::fs::canonicalize(in_dir).map_err(|e| e.to_string())?;
    let config_path = in_dir.join(PathBuf::from("_data/config.json5"));
    let config = Config::from_file(&config_path)?;

    let syntax_set = load_syntaxes();

    get_files_to_load(in_dir)
        .into_par_iter()
        .map(|path| {
            std::fs::read_to_string(&path)
                .map(|contents| Source {
                    path: path.clone(),
                    contents,
                })
                .map_err(|e| format!("{}: {}", path.display(), e))
        })
        .map(|result| {
            result.and_then(|source| {
                Page::new(&source, &syntax_set)
                    .map_err(|e| format!("{}: {}", source.path.display(), e))
            })
        })
        .map(|result| {
            result.and_then(|page| {
                let path = page.path(&config.output);
                println!(
                    "built final path {} from {} and {}",
                    &path.display(),
                    &page.metadata.slug,
                    &config.output.display()
                );
                let containing_dir = path
                    .parent()
                    .ok_or_else(|| format!("{} should have a containing dir!", path.display()))?;
                std::fs::create_dir_all(containing_dir)
                    .map_err(|e| format!("{}: {}", path.display(), e.to_string()))?;
                println!("writing {}", path.display());
                std::fs::write(&path.with_extension("html"), page.contents)
                    .map_err(|e| format!("{}: {}", path.display(), e))
            })
        })
        .fold(
            || Ok(()),
            |so_far, result| match (so_far, result) {
                (Ok(_), Ok(_)) => Ok(()),
                (Err(s), Ok(_)) => Err(s),
                (Ok(_), Err(e)) => Err(e),
                (Err(s), Err(e)) => Err(s + &e),
            },
        )
        .collect()
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
