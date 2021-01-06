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
pub(crate) struct Page<'s> {
    /// Mapped from the input file name, useful for permalinks.
    file_slug: String,

    /// Url used to link to this piece of content.
    url: String,

    /// The resolved date.
    date: DateTime<FixedOffset>,

    /// The fully-parsed metadata associated with the page.
    metadata: ResolvedMetadata<'s>,

    /// The fully-rendered contents of the page.
    contents: String,
}

impl<'s> Page<'s> {
    pub(crate) fn new(source: Source, config: &Config) -> Result<Self, String> {
        let Components { header, body } = Components::try_from(source.contents.as_ref())?;
        let resolved_metadata = ResolvedMetadata::new(&source.path, header, config)?;

        todo!()
    }
}

#[derive(Debug)]
pub(crate) enum RequiredFields<'a> {
    Title(&'a str),
    Date(&'a DateTime<FixedOffset>),
    Both {
        title: &'a str,
        date: &'a DateTime<FixedOffset>,
    },
}

impl<'m> TryFrom<&'m Metadata> for RequiredFields<'m> {
    type Error = String;

    fn try_from(metadata: &'m Metadata) -> Result<Self, Self::Error> {
        match (metadata.title.as_ref(), metadata.date.as_ref()) {
            (Some(title), Some(date)) => Ok(RequiredFields::Both { title, date }),
            (None, Some(date)) => Ok(RequiredFields::Date(date)),
            (Some(title), None) => Ok(RequiredFields::Title(title)),
            (None, None) => Err(String::from("missing date and title")),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ResolvedMetadata<'source> {
    /// The date, title, or both (every item must have one or the other)
    required: RequiredFields<'source>,

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

impl<'h> ResolvedMetadata<'h> {
    fn new(
        src_path: &PathBuf,
        header: &'h str,
        config: &Config,
    ) -> Result<ResolvedMetadata<'h>, String> {
        let metadata: Metadata = serde_yaml::from_str(header).map_err(|e| format!("{}", e))?;

        let required_fields = RequiredFields::try_from(&metadata)?;

        unimplemented!()
    }
}
