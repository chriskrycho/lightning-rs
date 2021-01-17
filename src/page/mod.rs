pub(crate) mod components;
pub(crate) mod markdown;
pub(crate) mod metadata;
pub mod source;

use std::{
    convert::TryFrom,
    path::{Path, PathBuf},
};

use chrono::{DateTime, FixedOffset};

use components::Components;
use markdown::render_markdown;
use metadata::{Metadata, Subscribe};
use syntect::parsing::SyntaxSet;

use crate::config::Config;

use self::{
    metadata::{Book, Qualifiers, Series},
    source::Source,
};

/// A fully-resolved representation of a page.
///
/// In this struct, the metadata has been parsed and resolved, and the content
/// has been converted from Markdown to HTML and preprocessed with both the
/// templating engine and my typography tooling. It is render to render into the
/// target layout template specified by its `metadata: ResolvedMetadata` and
/// then to print to the file system.
#[derive(Debug)]
pub(crate) struct Page {
    /// The fully-parsed metadata associated with the page.
    pub(crate) metadata: ResolvedMetadata,

    /// The fully-rendered contents of the page.
    pub(crate) contents: String,
}

impl Page {
    pub(crate) fn new(source: &Source, syntax_set: &SyntaxSet) -> Result<Self, String> {
        let Components { header, body } = Components::try_from(source.contents.as_ref())?;
        let metadata = ResolvedMetadata::new(&source.path, header)?;

        let contents = render_markdown(body, syntax_set)?;

        Ok(Page { metadata, contents })
    }

    pub(crate) fn path(&self, output_dir: &Path) -> PathBuf {
        output_dir.join(&self.metadata.slug)
    }

    /// Given a config, generate the (canonicalized) URL for the page
    pub(crate) fn url(&self, config: &Config) -> String {
        String::from(config.url.trim_end_matches('/')) + "/" + &self.metadata.slug
    }
}

#[derive(Debug)]
pub(crate) enum RequiredFields {
    Title(String),
    Date(DateTime<FixedOffset>),
    Both {
        title: String,
        date: DateTime<FixedOffset>,
    },
}

/// Metadata after combining the header config with all items in data hierarchy,
/// including the root config.
#[derive(Debug)]
pub(crate) struct ResolvedMetadata {
    /// The date, title, or both (every item must have one or the other)
    required: RequiredFields,

    /// The path to this piece of content.
    pub(crate) slug: String,

    layout: String,

    subtitle: Option<String>,
    summary: Option<String>,
    qualifiers: Option<Qualifiers>,
    updated: Option<DateTime<FixedOffset>>,
    thanks: Option<String>,
    tags: Vec<String>,
    featured: bool,
    book: Option<Book>,
    series: Option<Series>,
    subscribe: Option<Subscribe>,
}

impl ResolvedMetadata {
    // TODO: need to include root dir so that this can generate the slug
    // correctly.
    fn new(src_path: &PathBuf, header: &str) -> Result<ResolvedMetadata, String> {
        let item_metadata: Metadata = serde_yaml::from_str(header).map_err(|e| format!("{}", e))?;

        let required = (match (item_metadata.title, item_metadata.date) {
            (Some(title), Some(date)) => Ok(RequiredFields::Both { title, date }),
            (None, Some(date)) => Ok(RequiredFields::Date(date)),
            (Some(title), None) => Ok(RequiredFields::Title(title)),
            (None, None) => Err(String::from("missing date and title")),
        })?;

        let slug = item_metadata
            .permalink
            .map(|permalink| {
                permalink
                    .trim_start_matches('/')
                    .trim_end_matches('/')
                    .to_string()
            })
            .unwrap_or_else(|| {
                slug::slugify(src_path.to_str().unwrap_or_else(|| {
                    panic!("it should be impossible to get here without a valid source path")
                }))
            });

        Ok(ResolvedMetadata {
            required,
            slug,
            subtitle: item_metadata.subtitle,
            layout: String::from("base.html"), // TODO: not this!
            summary: item_metadata.summary,
            qualifiers: item_metadata.qualifiers,
            updated: item_metadata.updated,
            thanks: item_metadata.thanks,
            tags: item_metadata.tags,
            featured: item_metadata.featured,
            book: item_metadata.book,
            series: item_metadata.series,
            subscribe: item_metadata.subscribe,
        })
    }
}
