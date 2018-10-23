//! Generate the site content.

// Standard library
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

// Third party
use chrono::FixedOffset;
use glob::{glob, Paths};
use pandoc::{
    InputFormat, InputKind, OutputFormat, OutputKind, Pandoc, PandocOption, PandocOutput,
};
use syntect::highlighting::ThemeSet;

// First party
use config::Config;
use item;
use syntax_highlighting::syntax_highlight;

/// Load the `Paths` for all markdown files in the specified content directory.
fn glob_md_paths(site_directory: &PathBuf, config: &Config) -> Result<Paths, String> {
    let content_glob_str = format!(
        "{}/{}/**/*.md",
        site_directory
            .to_str()
            .ok_or_else(|| String::from("bad `site`"))?,
        config
            .directories
            .content
            .to_str()
            .ok_or_else(|| String::from("bad content directory"))?
    );

    glob(&content_glob_str).map_err(|err| format!("{:?}", err))
}

/// Load the templates associated with each taxonomy.
fn load_templates(site_directory: &PathBuf, config: &Config) -> Result<Paths, String> {
    unimplemented!()
}

/// Generate content from a configuration.
pub fn build(site_directory: PathBuf) -> Result<(), String> {
    // In the vein of "MVP": let's start by just loading all the files. We'll
    // extract this all into standalone functions as necessary later.

    let config = Config::load(&PathBuf::from(&site_directory))?;
    let markdown_paths = glob_md_paths(&site_directory, &config)?;
    // let templates = load_templates(&site_directory, &config)?;

    // TODO: build from config.
    let theme_file = PathBuf::from("data/base16-harmonic16.light.tmTheme");
    let theme = &ThemeSet::get_theme(theme_file).map_err(|err| format!("{:?}", err))?;

    let mut pandoc = Pandoc::new();
    pandoc
        .set_input_format(InputFormat::Markdown)
        .set_output_format(OutputFormat::Html5)
        .add_options(&[PandocOption::Smart, PandocOption::NoHighlight])
        .set_output(OutputKind::Pipe);

    for path_result in markdown_paths {
        let path = path_result.map_err(|e| format!("{:?}", e))?;
        let contents = load_file(&path)?;

        // TODO: get an *item*. Indeed, extract all of this over there: the
        // builder needs to delegate all of that, and simply get back an item
        // with metadata and string content to convert.
        // TODO: use something besides UTC: pass it in from config.
        let utcish = FixedOffset::east(0);
        let item = item::Item::from_str_unprocessed(&contents, &path, utcish, &config)?;

        let mut pandoc = pandoc.clone();
        pandoc.set_input(InputKind::Pipe(contents));
        let converted = match pandoc.execute() {
            Ok(PandocOutput::ToFile(path_buf)) => {
                let msg = format!(
                    "We wrote to a file ({}) instead of a pipe. That was weird.",
                    path_buf.to_string_lossy()
                );
                return Err(msg);
            }
            Ok(PandocOutput::ToBuffer(string)) => string,
            Err(err) => {
                return Err(format!(
                    "pandoc failed on {}:\n{:?}",
                    path.to_string_lossy(),
                    err
                ));
            }
        };

        let highlighted = syntax_highlight(converted, theme);

        // TODO: this is a temporary hack to get this to build.
        let slug = match item {
            item::Item::Processed { metadata, .. } => metadata.slug,
            item::Item::Unprocessed { metadata, .. } => metadata.slug,
        };

        write_file(&config.directories.output, &slug, &highlighted)?;
    }

    Ok(())
}

fn load_file(path: &Path) -> Result<String, String> {
    let mut file = File::open(&path).map_err(|err| format!("{:?}", err.kind()))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|err| format!("{:?}", err.kind()))?;
    Ok(contents)
}

fn write_file(output_dir: &Path, slug: &str, contents: &str) -> Result<(), String> {
    let path = output_dir.join(slug).with_extension("html");

    let mut fd = File::create(&path).map_err(|err| {
        format!(
            "Could not open {} for write: {}",
            path.to_string_lossy(),
            err
        )
    })?;

    write!(fd, "{}", contents).map_err(|err| format!("{:?}", err.kind()))
}
