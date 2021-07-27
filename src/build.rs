use std::path::{Path, PathBuf};

use rayon::prelude::*;
use syntect::highlighting::ThemeSet;
use syntect::html::{css_for_theme_with_class_style, ClassStyle};
use syntect::parsing::SyntaxSet;

use crate::config::Config;
use crate::page::{Page, Source};

pub fn build(in_dir: PathBuf) -> Result<(), String> {
    let in_dir = std::fs::canonicalize(in_dir).map_err(|e| e.to_string())?;
    let config_path = in_dir.join(PathBuf::from("_data/config.json5"));
    let config = Config::from_file(&config_path)?;

    let syntax_set = load_syntaxes();

    let SiteFiles {
        // TODO: generate collections/taxonomies/whatever from configs
        configs: _configs,
        content,
    } = get_files_to_load(&in_dir);
    let ThemeSet { themes } = ThemeSet::load_defaults();

    let style = ClassStyle::Spaced;
    let light = css_for_theme_with_class_style(&themes["InspiredGitHub"], style);
    let dark = css_for_theme_with_class_style(&themes["base16-ocean.dark"], style);

    std::fs::write(&config.output.join("light.css"), light).expect("can write output yo!");
    std::fs::write(&config.output.join("dark.css"), dark).expect("can write output yo!");

    content
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
                Page::new(&source, &in_dir.join("content"), &syntax_set, &config)
                    .map_err(|e| format!("{}: {}", source.path.display(), e))
            })
        })
        .map(|result| {
            result.and_then(|page| {
                let path = page.path_from_root(&config.output).with_extension("html");
                let containing_dir = path
                    .parent()
                    .ok_or_else(|| format!("{} should have a containing dir!", path.display()))?;
                std::fs::create_dir_all(containing_dir)
                    .map_err(|e| format!("{}: {}", path.display(), e.to_string()))?;
                // TODO: replace with a templating engine!
                std::fs::write(
                    &path,
                    format!(
                        r#"<html>
                            <head>
                                <link rel="stylesheet" href="/light.css" media="(prefers-color-scheme: light)" />
                                <link rel="stylesheet" href="/dark.css" media="(prefers-color-scheme: dark)" />
                            </head>
                            <body>
                                {body}
                            </body>
                        </html>"#,
                        body = page.contents
                    ),
                )
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

struct SiteFiles {
    configs: Vec<PathBuf>,
    content: Vec<PathBuf>,
}

fn get_files_to_load(in_dir: &Path) -> SiteFiles {
    let content_dir = in_dir.join("content");
    let dir_for_glob = content_dir.display();

    SiteFiles {
        configs: get_files(format!("{}/**/config.lx.yaml", dir_for_glob)),
        content: get_files(format!("{}/**/*.md", dir_for_glob)),
    }
}

fn get_files<S: AsRef<str>>(glob_src: S) -> Vec<PathBuf> {
    let src = glob_src.as_ref();
    let (ok_files, err_files): (Vec<PathBuf>, Vec<String>) = glob::glob(src)
        .unwrap_or_else(|_| panic!("bad glob: '{}'", src))
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
