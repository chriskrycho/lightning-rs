//! Process configurations for Lightning sites.

// First-party
use std::collections::BTreeMap;
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
    #[serde(rename = "site_info")]
    pub site: SiteInfo,
    pub directories: Directories,
    pub structure: Structure,
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
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(tag = "type")]
pub enum Taxonomy {
    #[serde(rename = "binary")]
    Binary {
        /// The name of the taxonomy.
        name: String,

        /// Configuration of the templates for this taxonomy.
        templates: Templates,

        /// Whether taxonomy terms can be nested.
        hierarchical: bool,

        /// Specify whether this taxonomy should generate feeds.
        #[serde(default)]
        generate_feeds: bool,
    },

    #[serde(rename = "multiple")]
    Multiple {
        /// The name of the taxonomy.
        name: String,

        /// Configuration of the templates for this taxonomy.
        templates: Templates,
        default: Option<String>,

        /// Whether an item may be in more than one of this taxonomy term at a
        /// time, e.g. whether a blog post may belong to multiple categories.
        limit: Option<u8>,

        /// Specify whether this taxonomy is required to exist on every item.
        #[serde(default)]
        required: bool,

        /// Whether taxonomy terms can be nested.
        hierarchical: bool,

        /// Specify whether this taxonomy should generate feeds.
        #[serde(default)]
        generate_feeds: bool,
    },

    #[serde(rename = "temporal")]
    Temporal {
        /// The name of the taxonomy.
        name: String,

        /// Configuration of the templates for this taxonomy.
        templates: Templates,

        /// Specify whether this taxonomy is required to exist on every item.
        #[serde(default)]
        required: bool,

        /// Specify whether this taxonomy should generate feeds.
        #[serde(default)]
        generate_feeds: bool,
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
pub enum FeedEngine {
    Atom,
    RSS,
    JSON,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Structure {
    directory: PathBuf,
    index: String,
    taxonomies: Vec<Taxonomy>,
    feeds: Feeds,
    other_content: OtherContent,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Feeds {
    engines: Vec<FeedEngine>,
    additional: Vec<AdditionalFeed>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct AdditionalFeed {
    name: String,
    taxonomies: Vec<TaxonomySubset>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct TaxonomySubset {
    taxonomy: String,
    terms: Vec<TaxonomyTerm>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum TaxonomyTerm {
    String(String),
    Number(u32), // TODO: this is wrong, should be parsed as a year!
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct OtherContent {
    copy: Vec<PathBuf>,

    #[serde(deserialize_with = "parse_exclude")]
    exclude: Vec<PathBuf>,
}

fn parse_exclude<'de, D>(d: D) -> Result<Vec<PathBuf>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    serde::Deserialize::deserialize(d).map(|value: Option<_>| value.unwrap_or(Vec::new()))
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
    generate_feeds: true
-   name: category
    type: multiple
    default: Blog
    limit: 1
    required: false
    hierarchical: false
    templates:
        list: categories.html
        item: category.html
    generate_feeds: false
-   name: tag
    type: multiple
    limit: ~
    required: false
    hierarchical: false
    templates:
        list: tags.html
        item: tag.html
-   name: date
    type: temporal
    required: false
    templates:
        list: period_archives.html
        item: archives.html
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
            generate_feeds: true,
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
            generate_feeds: false,
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
            generate_feeds: false,
        },
        Taxonomy::Temporal {
            name: "date".into(),
            required: false,
            templates: Templates {
                item: "archives.html".into(),
                list: Some("period_archives.html".into()),
            },
            generate_feeds: false,
        },
        Taxonomy::Binary {
            name: "page".into(),
            hierarchical: true,
            templates: Templates {
                item: "page.html".into(),
                list: None,
            },
            generate_feeds: false,
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

#[test]
fn parses_default_config() {
    static DEFAULT_CONFIG: &'static str = include_str!("../tests/pure-config.yaml");

    let expected = Config {
        site: SiteInfo {
            title: "lx (lightning)".into(),
            url: "https://lightning.rs".into(),
            description: Some("A ridiculously fast site generator and engine.\n".into()),
            metadata: serde_yaml::Mapping::new(),
        },
        directories: Directories {
            content: "content".into(),
            output: "output".into(),
        },
        structure: Structure {
            directory: "layout".into(),
            index: "index.html".into(),
            taxonomies: vec![
                Taxonomy::Multiple {
                    name: "author".into(),
                    required: false,
                    hierarchical: false,
                    templates: Templates {
                        item: "author.html".into(),
                        list: Some("authors.html".into()),
                    },
                    generate_feeds: false,
                    limit: None,
                    default: None,
                },
                Taxonomy::Multiple {
                    name: "category".into(),
                    default: Some("Blog".into()),
                    limit: None,
                    required: false,
                    hierarchical: false,
                    templates: Templates {
                        item: "category.html".into(),
                        list: Some("categories.html".into()),
                    },
                    generate_feeds: false,
                },
                Taxonomy::Multiple {
                    name: "tag".into(),
                    limit: None,
                    hierarchical: false,
                    templates: Templates {
                        item: "tag.html".into(),
                        list: Some("tags.html".into()),
                    },
                    generate_feeds: false,
                    required: false,
                    default: None,
                },
                Taxonomy::Temporal {
                    name: "date".into(),
                    required: false,
                    templates: Templates {
                        item: "archives.html".into(),
                        list: Some("period_archives.html".into()),
                    },
                    generate_feeds: false,
                },
                Taxonomy::Binary {
                    name: "page".into(),
                    templates: Templates {
                        item: "page.html".into(),
                        list: None,
                    },
                    hierarchical: true,
                    generate_feeds: false,
                },
            ],
            feeds: Feeds {
                engines: vec![FeedEngine::RSS, FeedEngine::JSON],
                additional: vec![
                    AdditionalFeed {
                        name: "Art and Tech".into(),
                        taxonomies: vec![TaxonomySubset {
                            taxonomy: "categories".into(),
                            terms: vec![
                                TaxonomyTerm::String("tech".into()),
                                TaxonomyTerm::String("art".into()),
                            ],
                        }],
                    },
                    AdditionalFeed {
                        name: "2018 Family Poetry".into(),
                        taxonomies: vec![
                            TaxonomySubset {
                                taxonomy: "date".into(),
                                terms: vec![TaxonomyTerm::Number(2018)],
                            },
                            TaxonomySubset {
                                taxonomy: "tags".into(),
                                terms: vec![
                                    TaxonomyTerm::String("family".into()),
                                    TaxonomyTerm::String("poetry".into()),
                                ],
                            },
                        ],
                    },
                ],
            },
            other_content: OtherContent {
                copy: vec!["static".into(), "extra".into()],
                exclude: Vec::new(),
            },
        },
    };

    let config: Config = serde_yaml::from_str(DEFAULT_CONFIG).unwrap();
    assert_eq!(expected, config, "successfully deserialized basic config");
}
