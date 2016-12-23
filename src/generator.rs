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


/// Generate content from a configuration.
pub fn generate(site: PathBuf) -> Result<(), String> {
    // In the vein of "MVP": let's start by just loading all the files. We'll
    // extract this all into standalone functions as necessary later.

    let config = load_config(&PathBuf::from(&site))?;

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


const CONTENT_DIRECTORY: &'static str = "content_directory";
const TEMPLATE_DIRECTORY: &'static str = "directory";


struct Config {
    content_directory: PathBuf,
    template_directory: PathBuf,
}


enum Taxonomy {
    Multiple {
        name: String,
        limit: Option<u8>,
        required: bool,
        hierarchical: bool,
    },
    Binary,
    Temporal { required: bool },
}


fn load_config(directory: &PathBuf) -> Result<Config, String> {
    const CONFIG_FILE: &'static str = "lightning.yaml";
    let config_path = directory.join(CONFIG_FILE);
    if !config_path.exists() {
        return Err(format!("The specified configuration path {:?} does not exist.",
                           config_path.to_string_lossy()));
    }

    let mut file = File::open(&config_path)
        .map_err(|reason| format!("Error reading {:?}: {:?}", config_path, reason))?;

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(err) => return Err(String::from(err.description())),
    };

    // A Lightning config is always a hash. We need all these intermediate
    // bindings because the temporaries created along the way don't live long
    // enough otherwise.
    let yaml_config = YamlLoader::load_from_str(&contents)
        .map_err(|err| format!("{} ({:?})", err, &config_path))?;
    let yaml_config = yaml_config.into_iter().next().ok_or("Empty configuration file")?;
    let yaml_config = yaml_config.as_hash().ok_or("Configuration is not a map")?;

    let content_directory_yaml = yaml_config.get(&Yaml::from_str(CONTENT_DIRECTORY))
        .ok_or(format!("No `{:}` key in {:?}", CONTENT_DIRECTORY, config_path))?;

    fn path_buf_from_yaml(yaml: &Yaml,
                          key: &str,
                          config_path: &PathBuf)
                          -> Result<PathBuf, String> {
        match yaml {
            &Yaml::String(ref path_str) => Ok(PathBuf::from(path_str)),
            value => Err(format!("invalid `{:}` value {:?} in {:?}", key, value, config_path)),
        }
    }

    let content_directory =
        path_buf_from_yaml(&content_directory_yaml, CONTENT_DIRECTORY, &config_path)?;

    let structure = yaml_config.get(&Yaml::from_str("structure"))
        .ok_or(format!("No `structure` key in {:?}", config_path))?
        .as_hash()
        .ok_or(format!("`structure` is not a map in {:?}", config_path))?;

    let template_directory_yaml = structure.get(&Yaml::from_str(TEMPLATE_DIRECTORY))
        .ok_or(format!("No `directory` key in `structure` in {:?}", config_path))?;

    let template_directory =
        path_buf_from_yaml(&template_directory_yaml, TEMPLATE_DIRECTORY, &config_path)?;

    Ok(Config {
        content_directory: content_directory,
        template_directory: template_directory,
    })
}
