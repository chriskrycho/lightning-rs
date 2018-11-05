//! Process configurations for Lightning sites.

// First-party
use std::convert::From;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::u8;

// Third-party
use serde;
use serde_derive::Deserialize;
use serde_yaml;

// First-party
pub use crate::validated_types::Url as ValidatedUrl;

const CONFIG_FILE_NAME: &'static str = "lightning.yaml";

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub site: SiteInfo,
    pub directories: Directories,
    pub taxonomies: Vec<Taxonomy>,
}

impl Config {
    pub fn from_file(directory: &PathBuf) -> Result<Config, String> {
        let config_path = directory.join(CONFIG_FILE_NAME);
        if !config_path.exists() {
            return Err(format!(
                "The specified configuration path {:} does not exist.",
                config_path.to_string_lossy()
            ));
        }

        let mut contents = String::new();
        File::open(&config_path)
            .map_err(|reason| format!("Error reading {:?}: {:?}", config_path, reason))?
            .read_to_string(&mut contents)
            .map_err(|reason| String::from(reason.description()))?;

        Config::parse(&contents)
    }

    fn parse(source: &str) -> Result<Config, String> {
        let config = serde_yaml::from_str(&source).map_err(|e| format!("{:}", e));
        // TODO: add some basic validation here
        config
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Directories {
    pub content: PathBuf,
    pub output: PathBuf,
    pub template: PathBuf,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(tag = "type")]
pub enum Taxonomy {
    #[serde(rename = "binary")]
    Binary {
        name: String,
        templates: Templates,
        hierarchical: bool,

        /// The feed types associated with the taxonomy
        #[serde(default)]
        feeds: Vec<Feed>,
    },

    #[serde(rename = "multiple")]
    Multiple {
        name: String,
        templates: Templates,
        default: Option<String>,
        limit: Option<u8>,
        required: bool,
        hierarchical: bool,

        /// The feed types associated with the taxonomy
        #[serde(default)]
        feeds: Vec<Feed>,
    },

    #[serde(rename = "temporal")]
    Temporal {
        name: String,
        templates: Templates,
        required: bool,

        /// The feed types associated with the taxonomy
        #[serde(default)]
        feeds: Vec<Feed>,
    },
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct SiteInfo {
    /// The name of the site. Required.
    pub title: String,

    /// The canonical URL for the root of the site. Required.
    pub url: String,

    /// The description of the site. Optional.
    pub description: Option<String>,

    /// Arbitrary metadata associated with the site. Optional.
    #[serde(deserialize_with = "parse_metadata")]
    pub metadata: serde_yaml::Mapping,
}

fn parse_metadata<'de, D>(d: D) -> Result<serde_yaml::Mapping, D::Error>
where
    D: serde::Deserializer<'de>,
{
    serde::Deserialize::deserialize(d)
        .map(|value: Option<_>| value.unwrap_or(serde_yaml::Mapping::new()))
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Templates {
    pub item: PathBuf,
    pub list: Option<PathBuf>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum Feed {
    Atom,
    RSS,
    JSON,
}

#[test]
fn parses_valid_taxonomies() {
    const TAXONOMIES: &'static str = "
-   name: author
    type: multiple
    required: true
    hierarchical: false
    templates:
        list: authors.html
        item: author.html
    feeds:
        - RSS
-   name: category
    type: multiple
    default: Blog
    limit: 1
    required: false
    hierarchical: false
    templates:
        list: categories.html
        item: category.html
    feeds:
        - Atom
-   name: tag
    type: multiple
    limit: ~
    required: false
    hierarchical: false
    templates:
        list: tags.html
        item: tag.html
    feeds:
        - JSON
-   name: date
    type: temporal
    required: false
    templates:
        list: period_archives.html
        item: archives.html
    feeds:
        - RSS
        - Atom
        - JSON
-   name: page
    type: binary
    hierarchical: true
    templates:
        item: page.html
        ";

    let expected = vec![
        Taxonomy::Multiple {
            name: "author".into(),
            default: None,
            limit: None,
            required: true,
            hierarchical: false,
            templates: Templates {
                item: "author.html".into(),
                list: Some("authors.html".into()),
            },
            feeds: vec![Feed::RSS],
        },
        Taxonomy::Multiple {
            name: "category".into(),
            default: Some("Blog".into()),
            limit: Some(1),
            required: false,
            hierarchical: false,
            templates: Templates {
                item: "category.html".into(),
                list: Some("categories.html".into()),
            },
            feeds: vec![Feed::Atom],
        },
        Taxonomy::Multiple {
            name: "tag".into(),
            default: None,
            limit: None,
            required: false,
            hierarchical: false,
            templates: Templates {
                item: "tag.html".into(),
                list: Some("tags.html".into()),
            },
            feeds: vec![Feed::JSON],
        },
        Taxonomy::Temporal {
            name: "date".into(),
            required: false,
            templates: Templates {
                item: "archives.html".into(),
                list: Some("period_archives.html".into()),
            },
            feeds: vec![Feed::RSS, Feed::Atom, Feed::JSON],
        },
        Taxonomy::Binary {
            name: "page".into(),
            hierarchical: true,
            templates: Templates {
                item: "page.html".into(),
                list: None,
            },
            feeds: vec![],
        },
    ];

    let loaded: Vec<Taxonomy> =
        serde_yaml::from_str(TAXONOMIES).expect("bad test data: TAXONOMIES");

    assert_eq!(expected, loaded);
}

#[test]
fn parses_site_info() {
    const SITE_INFO: &'static str = "\
title: lx (lightning)
url: https://lightning.rs
description: >
    A ridiculously fast site generator and engine.
metadata:
    foo: bar
    quux: 2
    ";

    let mut metadata = serde_yaml::Mapping::new();
    metadata.insert("foo".into(), "bar".into());
    metadata.insert("quux".into(), 2.into());

    let expected = SiteInfo {
        title: "lx (lightning)".into(),
        url: String::from("https://lightning.rs"),
        description: Some("A ridiculously fast site generator and engine.\n".into()),
        metadata: metadata,
    };

    let loaded: SiteInfo = serde_yaml::from_str(SITE_INFO).expect("bad test data: SITE_INFO");
    assert_eq!(expected, loaded);
}

#[test]
fn parses_site_info_with_empty_metadata() {
    const SITE_INFO_EMPTY_METADATA: &'static str = "
title: lx (lightning)
url: https://lightning.rs
description: >
    A ridiculously fast site generator and engine.
metadata: ~
    ";

    let expected = SiteInfo {
        title: "lx (lightning)".into(),
        url: String::from("https://lightning.rs"),
        description: Some("A ridiculously fast site generator and engine.\n".into()),
        metadata: serde_yaml::Mapping::new(),
    };

    let loaded: SiteInfo = serde_yaml::from_str(SITE_INFO_EMPTY_METADATA)
        .expect("bad test data: SITE_INFO_EMPTY_METADATA");

    assert_eq!(expected, loaded);
}
