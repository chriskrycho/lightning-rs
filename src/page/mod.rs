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

impl Page {
    pub(crate) fn new(source: Source, config: &Config) -> Result<Self, String> {
        let Components { header, body } = Components::try_from(source.contents.as_ref())?;
        let resolved_metadata = ResolvedMetadata::new(&source.path, header, config)?;

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

#[derive(Debug)]
pub(crate) struct ResolvedMetadata {
    /// The date, title, or both (every item must have one or the other)
    required: RequiredFields,

    /// Url used to link to this piece of content.
    url: String,

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

impl<'h> ResolvedMetadata {
    fn new(
        src_path: &PathBuf,
        header: &'h str,
        config: &Config,
    ) -> Result<ResolvedMetadata, String> {
        let item_metadata: Metadata = serde_yaml::from_str(header).map_err(|e| format!("{}", e))?;

        let required = (match (item_metadata.title, item_metadata.date) {
            (Some(title), Some(date)) => Ok(RequiredFields::Both { title, date }),
            (None, Some(date)) => Ok(RequiredFields::Date(date)),
            (Some(title), None) => Ok(RequiredFields::Title(title)),
            (None, None) => Err(String::from("missing date and title")),
        })?;

        // TODO: less dumb than this. Including, you know, slugifying it.
        let url_from_src = config.url.to_string()
            + src_path
                .to_str()
                .expect("why *wouldn't* the src_path be legit here?");

        let url = item_metadata.permalink.unwrap_or(url_from_src);

        Ok(ResolvedMetadata {
            required,
            url, // looool TODO: something less dumb
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
        })
    }
}
