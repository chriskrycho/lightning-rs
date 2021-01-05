pub(crate) mod components;
pub(crate) mod metadata;
pub(crate) mod source;

use std::{convert::TryFrom, path::PathBuf, unimplemented};

use chrono::{format::ParseError, FixedOffset};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};

use components::Components;
use metadata::Metadata;

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
    /// Mapped from the input file name, useful for permalinks.
    file_slug: String,

    /// Url used to link to this piece of content.
    url: String,

    /// The resolved date.
    date: DateTime<FixedOffset>,

    /// The fully-parsed metadata associated with the page.
    metadata: ResolvedMetadata,

    /// The fully-rendered contents of the page.
    contents: String,
}

impl<'s, 'p> TryFrom<&Source<'s, 'p>> for Page {
    type Error = String;

    fn try_from(source: &Source) -> Result<Self, Self::Error> {
        let Components { header, body } = Components::try_from(source.contents.as_ref())?;
        let metadata: Metadata = serde_yaml::from_str(header).map_err(|e| format!("{}", e))?;

        todo!()
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

impl TryFrom<&Metadata> for RequiredFields {
    type Error = String;

    fn try_from(metadata: &Metadata) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug)]
pub(crate) struct ResolvedMetadata {
    /// The date, title, or both (every item must have one or the other)
    required: RequiredFields,

    /// Mapped from the input file name, useful for permalinks.
    file_slug: String,

    /// Url used to link to this piece of content.
    url: String,

    /// The resolved date.
    date: DateTime<FixedOffset>,

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
}

impl ResolvedMetadata {
    fn new(src_path: PathBuf, header: &str, config: Config) -> Result<ResolvedMetadata, String> {
        let metadata: Metadata = serde_yaml::from_str(header).map_err(|e| format!("{}", e))?;

        let required_fields = match (metadata.title, metadata.date) {
            (None, None) => {
                return Err(format!(
                    "missing required fields (title|date) in {}",
                    src_path.display()
                ));
            }
            (Some(title), None) => RequiredFields::Title(title),
            (None, Some(date)) => RequiredFields::Date(date),
            (Some(title), Some(date)) => RequiredFields::Both { title, date },
        };

        unimplemented!()
    }
}

impl Page {
    pub(crate) fn new(src_path: PathBuf, contents: String) -> Page {
        let (header, body) = to_header_and_body(&contents);
        // - parse header into yaml
        // -
        unimplemented!()
    }
}

fn to_header_and_body(s: &str) -> (String, String) {
    unimplemented!()
}
