//! Generate the site content.

// Standard library
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

// Third party
use glob::glob;
use pandoc::{Pandoc, PandocOption, PandocOutput, InputFormat, OutputFormat, OutputKind};
use yaml_rust::{Yaml, YamlLoader};

// First party
use syntax_highlighting::syntax_highlight;


pub struct Site {
    pub source_directory: PathBuf,
}


/// Generate content from a configuration.
pub fn generate(site: Site) -> Result<(), String> {
    // In the vein of "MVP": let's start by just loading all the files. We'll
    // extract this all into standalone functions as necessary later.

    // TODO: load config!
    //
    // Instead of just loading the files in the source directory as a glob of
    // all Markdown files, load the *config* and let *it* specify the source of
    // the files to convert.
    let dir_str = format!("{}/**/*.md",
                          site.source_directory.to_str().ok_or(String::from("bad directory"))?);

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

        let mut fd = match File::create(dest.clone()) {
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

const CONTENT_DIRECTORIES: &'static str = "content_directories";
const TEMPLATE_DIRECTORY: &'static str = "template_directory";

struct Config {
    content_directories: Vec<PathBuf>,
    template_directory: PathBuf,
}

fn load_config(directory: &PathBuf) -> Result<Config, String> {
    const CONFIG_FILE: &'static str = "lightning.yaml";
    let config_path = directory.join(CONFIG_FILE);
    if !config_path.exists() {
        return Err(format!("The specified configuration path {:?} does not exist",
                           config_path.to_string_lossy()));
    }

    let mut file = File::open(&config_path)
        .map_err(|reason| format!("Error reading {:?}: {:?}", config_path, reason))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents);

    // A Lightning config is always a hash. We need all these intermediate
    // bindings because the temporaries created along the way don't live long
    // enough otherwise.
    let yaml_config =
        YamlLoader::load_from_str(&contents).map_err(|err| String::from(err.description()))?;
    let yaml_config = yaml_config.into_iter().next().ok_or("Empty configuration file")?;
    let yaml_config = yaml_config.as_hash().ok_or("Configuration is not a map")?;

    let (content_directories, errs): (Vec<PathBuf>, Vec<String>) =
        yaml_config.get(&Yaml::from_str("content_directories"))
            .ok_or(format!("No `content_directories` key in {:?}", config_path))?
            .as_vec()
            .ok_or(format!("`content_directories` is not a vector in {:?}", config_path))?
            .into_iter()
            .map(|val| match val {
                &Yaml::String(ref path_str) => Ok(PathBuf::from(path_str)),
                value => {
                    Err(format!("invalid `content_directories` value {:?} in {:?}",
                                value,
                                config_path))
                }
            })
            .fold((Vec::new(), Vec::new()), |(mut dirs, mut errs), entry| {
                match entry {
                    Ok(path_buf) => {
                        dirs.push(path_buf);
                    }
                    Err(description) => {
                        errs.push(description);
                    }
                }

                return (dirs, errs);
            });

    if errs.len() > 0 {
        let combined_errs =
            errs.into_iter().fold(format!("Invalid `content_directories` values:"),
                                  |joined_errs, err| joined_errs + &format!("\n{:?}", err));
        return Err(combined_errs);
    }

    let structure = yaml_config.get(&Yaml::from_str("structure"))
        .ok_or(format!("No `structure` key in {:?}", config_path))?
        .as_hash()
        .ok_or(format!("`structure` is not a map in {:?}", config_path))?;

    let template_directory_yaml = structure.get(&Yaml::from_str("directory"))
        .ok_or(format!("No `directory` key in `structure` in {:?}", config_path))?;

    let template_directory = match template_directory_yaml {
        &Yaml::String(ref path_str) => PathBuf::from(path_str),
        value => {
            return Err(format!("invalid `template_directory` value {:?} in {:?}",
                               value,
                               config_path))
        }
    };

    Ok(Config {
        content_directories: content_directories,
        template_directory: template_directory,
    })
}
