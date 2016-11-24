//! Generate the site content.

// Standard library
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

// Third party
use glob::glob;
use pandoc::{Pandoc, PandocOption, InputFormat, OutputFormat, OutputKind};
use rayon::prelude::*;

// First party
use syntax_highlighting::syntax_highlight;


/// Generate content from a configuration.
pub fn generate() -> Result<(), String> {
    // In the vein of "MVP": let's start by just loading all the files. We'll
    // extract this all into standalone functions as necessary later.

    // TODO: load this from the configuration file.
    let directory = Path::new("tests/data");
    let dir_str = format!(
        "{}/**/*.md",
        directory.to_str().ok_or(String::from("bad directory"))?
    );

    let markdown_files = glob(&dir_str).map_err(|err| format!("{:?}", err))?;
    let (paths, failures): (Vec<_>, Vec<_>) = markdown_files
        .map(|path_result| { path_result.map_err(|e| format!("{:?}", e)) })
        .partition(|result| result.is_ok());

    if failures.len() > 0 {
        return failures.into_iter().next().unwrap().map(|_| ());
    }

    let (file_names, failures): (Vec<_>, Vec<_>) = paths.into_iter()
        .map(|path| {
            let err = format!("Could not convert path {:?} to str", &path);
            path.unwrap()
                .to_str()
                .ok_or(err)
                .map(|str| str.to_string())
        })
        .partition(|result| result.is_ok());

    if failures.len() > 0 {
        return failures.into_iter().next().unwrap().map(|_| ());
    }

    let file_names: Vec<_> = file_names.into_iter().map(|result| result.unwrap()).collect();
    let mut results: Vec<Result<(), String>> = Vec::new();

    file_names.par_iter().map(|file_name| {
        let mut pandoc = Pandoc::new();
        pandoc.set_input_format(InputFormat::Markdown)
            .set_output_format(OutputFormat::Html5)
            .add_options(&[PandocOption::Smart, PandocOption::NoHighlight])
            .add_input(&file_name)
            .set_output(OutputKind::Pipe);

        let converted = pandoc.execute_with_output()
            .map_err(|err| format!("pandoc failure: {}:\n{:?}", file_name, err))?;

        let highlighted = syntax_highlight(converted);

        // TODO: extract this as part of the writing it out process.
        let ff_path = Path::new(&file_name);
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

        return Ok(())
    }).collect_into(&mut results);

    let (_, errors): (Vec<_>, Vec<_>) = results.into_iter().partition(|result| result.is_ok());
    if errors.len() > 0 {
        return errors.first().unwrap().clone();
    }

    Ok(())
}
