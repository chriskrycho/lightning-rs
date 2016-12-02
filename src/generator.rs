//! Generate the site content.

// Standard library
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

// Third party
use glob::glob;
use pandoc::{Pandoc, PandocOption, InputFormat, OutputFormat, OutputKind};

// First party
use syntax_highlighting::syntax_highlight;


pub struct Site {
    pub source_directory: PathBuf,
    pub template_directory: Option<PathBuf>,
}


/// Generate content from a configuration.
pub fn generate(site: Site) -> Result<(), String> {
    // In the vein of "MVP": let's start by just loading all the files. We'll
    // extract this all into standalone functions as necessary later.

    // TODO: load this from the configuration file.
    let directory = Path::new("tests/data");
    let dir_str = format!(
        "{}/**/*.md",
        directory.to_str().ok_or(String::from("bad directory"))?
    );

    let markdown_files = glob(&dir_str).map_err(|err| format!("{:?}", err))?;

    // TODO: Iterate with `rayon::par_iter::for_each()`.
    for path_result in markdown_files {
        // TODO: extract this into a nice function to call in a for loop/foreach.
        let path = path_result.map_err(|e| format!("{:?}", e))?;
        let file_name = path.to_str()
            .ok_or(format!("Could not convert path {:?} to str", path))?;

        let mut pandoc = Pandoc::new();
        pandoc.set_input_format(InputFormat::Markdown)
            .set_output_format(OutputFormat::Html5)
            .add_options(&[PandocOption::Smart, PandocOption::NoHighlight])
            .add_input(file_name)
            .set_output(OutputKind::Pipe);

        let converted = pandoc.execute_with_output()
            .map_err(|err| format!("pandoc failed on {}:\n{:?}", file_name, err))?;

        let highlighted = syntax_highlight(converted);

        // TODO: extract this as part of the writing it out process.
        let ff_path = Path::new(file_name);
        let dest = Path::new("./tests/output")
            .join(ff_path.file_name().ok_or(format!("invalid file: {}", file_name))?)
            .with_extension("html");

        let mut fd = OpenOptions::new()
            .write(true)
            .create(true)
            .open(dest.clone())
            .map_err(|reason| {
                format!("Could not open {} for write: {}", dest.to_string_lossy(), reason)
            })?;

        let result = write!(fd, "{}", highlighted);
        if let Err(reason) = result {
            return Err(format!("{:?}", reason.kind()));
        }
    }

    Ok(())
}
