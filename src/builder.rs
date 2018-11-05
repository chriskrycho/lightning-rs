//! Generate the site content.

// Standard library
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

// Third party
use glob::{glob, Paths};
use pandoc::{InputFormat, OutputFormat, OutputKind, Pandoc, PandocOption, PandocOutput};
use syntect::highlighting::ThemeSet;

// First party
use crate::config::Config;
use crate::syntax_highlighting::syntax_highlight;

/// Load the `Paths` for all markdown files in the specified content directory.
fn glob_md_paths(site_directory: &PathBuf, config: &Config) -> Result<Paths, String> {
    let content_glob_str = format!(
        "{}/{}/**/*.md",
        site_directory.to_str().ok_or(String::from("bad `site`"))?,
        config
            .directories
            .content
            .to_str()
            .ok_or(String::from("bad content directory"))?
    );

    glob(&content_glob_str).map_err(|err| format!("{:?}", err))
}

/// Load the templates associated with each taxonomy.
fn load_templates(_site_directory: &PathBuf, _config: &Config) -> Result<Paths, String> {
    unimplemented!()
}

/// Generate content from a configuration.
pub fn build(site_directory: PathBuf) -> Result<(), String> {
    // In the vein of "MVP": let's start by just loading all the files. We'll
    // extract this all into standalone functions as necessary later.

    let config = Config::from_file(&PathBuf::from(&site_directory))?;
    let markdown_paths = glob_md_paths(&site_directory, &config)?;
    //    let templates = load_templates(&site_directory, &config)?;

    // TODO: build from config. Also, extract and just do this once *not* at the
    //       top level function.
    let theme_file = PathBuf::from("data/base16-harmonic16.light.tmTheme");
    let theme = &ThemeSet::get_theme(theme_file).map_err(|err| format!("{:?}", err))?;

    for path_result in markdown_paths {
        // TODO: extract this into a nice function to call in a for loop/foreach.
        let path = path_result.map_err(|e| format!("{:?}", e))?;
        let file_name = path
            .to_str()
            .ok_or(format!("Could not convert path {:?} to str", path))?;

        let mut pandoc = Pandoc::new();
        pandoc
            .set_input_format(InputFormat::Markdown)
            .set_output_format(OutputFormat::Html5)
            .add_options(&[PandocOption::Smart, PandocOption::NoHighlight])
            .add_input(file_name)
            .set_output(OutputKind::Pipe);

        let pandoc_output = pandoc
            .execute()
            .map_err(|err| format!("pandoc failed on {}:\n{:?}", file_name, err))?;

        let converted = match pandoc_output {
            PandocOutput::ToFile(path_buf) => {
                let msg = format!(
                    "We wrote to a file ({}) instead of a pipe. That was weird.",
                    path_buf.to_string_lossy()
                );
                return Err(msg);
            }
            PandocOutput::ToBuffer(string) => string,
        };

        let highlighted = syntax_highlight(converted, theme);

        // TODO: extract this as part of the writing it out process.
        // TODO: set output location in config.
        let ff_path = Path::new(file_name);
        let dest = Path::new("./tests/output")
            .join(
                ff_path
                    .file_name()
                    .ok_or(format!("invalid file: {}", file_name))?,
            )
            .with_extension("html");

        let mut fd = match File::create(&dest) {
            Ok(file) => file,
            Err(reason) => {
                return Err(format!(
                    "Could not open {} for write: {}",
                    dest.to_string_lossy(),
                    reason
                ));
            }
        };

        let result = write!(fd, "{}", highlighted);
        if let Err(reason) = result {
            return Err(format!("{:?}", reason.kind()));
        }
    }

    Ok(())
}
