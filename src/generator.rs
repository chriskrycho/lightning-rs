//! Generate the site content.

// Standard library
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

// Third party
use glob::glob;
use pandoc::{Pandoc, PandocOption, PandocOutput, InputFormat, OutputFormat, OutputKind};

// First party
use config::load;
use syntax_highlighting::syntax_highlight;


/// Generate content from a configuration.
pub fn generate(site: PathBuf) -> Result<(), String> {
    // In the vein of "MVP": let's start by just loading all the files. We'll
    // extract this all into standalone functions as necessary later.

    let config = load(&PathBuf::from(&site))?;

    let content_glob_str =
        format!("{}/{}/**/*.md",
                site.to_str().ok_or(String::from("bad `site`"))?,
                config.content_directory.to_str().ok_or(String::from("bad content_directory"))?);

    let markdown_files = glob(&content_glob_str).map_err(|err| format!("{:?}", err))?;

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

        let pandoc_output = pandoc.execute()
            .map_err(|err| format!("pandoc failed on {}:\n{:?}", file_name, err))?;

        let converted = match pandoc_output {
            PandocOutput::ToFile(path_buf) => {
                let msg = format!("We wrote to a file ({}) instead of a pipe. That was weird.",
                                  path_buf.to_string_lossy());
                return Err(msg);
            }
            PandocOutput::ToBuffer(string) => string,
        };

        let highlighted = syntax_highlight(converted);

        // TODO: extract this as part of the writing it out process.
        // TODO: set output location in config.
        let ff_path = Path::new(file_name);
        let dest = Path::new("./tests/output")
            .join(ff_path.file_name().ok_or(format!("invalid file: {}", file_name))?)
            .with_extension("html");

        let mut fd = match File::create(&dest) {
            Ok(file) => file,
            Err(reason) => {
                return Err(format!("Could not open {} for write: {}",
                                   dest.to_string_lossy(),
                                   reason));
            }
        };

        let result = write!(fd, "{}", highlighted);
        if let Err(reason) = result {
            return Err(format!("{:?}", reason.kind()));
        }
    }

    Ok(())
}
