pub(crate) mod components;
pub(crate) mod markdown;
pub(crate) mod metadata;

use std::{
    collections::HashMap,
    convert::TryFrom,
    hash::Hash,
    path::{Path, PathBuf},
};

use components::Components;
use markdown::render_markdown;
use serde::{Deserialize, Serialize};
use syntect::parsing::SyntaxSet;
use uuid::Uuid;

use crate::config::Config;

use self::metadata::Metadata;

/// Source data for a file: where it came from, and its original contents.
pub struct Source {
    /// Original source location for the file.
    pub path: PathBuf,
    /// Original contents of the file.
    pub contents: String,
}

/// A unique identifier
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Deserialize, Serialize)]
pub(crate) struct Id(Uuid);

/// A fully-resolved representation of a page.
///
/// In this struct, the metadata has been parsed and resolved, and the content
/// has been converted from Markdown to HTML and preprocessed with both the
/// templating engine and my typography tooling. It is ready to render into the
/// target layout template specified by its `metadata: ResolvedMetadata` and
/// then to print to the file system.
#[derive(Debug)]
pub(crate) struct Page {
    pub(crate) id: Id,

    /// The fully-parsed metadata associated with the page.
    pub(crate) metadata: Metadata,

    /// The fully-rendered contents of the page.
    pub(crate) contents: String,
}

impl Page {
    pub(crate) fn new(
        source: &Source,
        root_dir: &PathBuf,
        syntax_set: &SyntaxSet,
    ) -> Result<Self, String> {
        let id = Id(Uuid::new_v5(
            &Uuid::NAMESPACE_OID,
            source.contents.as_bytes(),
        ));

        let Components { header, body } = Components::try_from(source.contents.as_ref())?;
        let metadata = Metadata::new(&source.path, root_dir, header)?;

        let contents = render_markdown(body, syntax_set)?;

        Ok(Page {
            id,
            metadata,
            contents,
        })
    }

    pub(crate) fn path(&self, output_dir: &Path) -> PathBuf {
        output_dir.join(&self.metadata.slug)
    }

    /// Given a config, generate the (canonicalized) URL for the page
    pub(crate) fn url(&self, config: &Config) -> String {
        String::from(config.url.trim_end_matches('/')) + "/" + &self.metadata.slug
    }
}

impl From<&Page> for lx_json_feed::FeedItem {
    fn from(_: &Page) -> Self {
        unimplemented!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageCollections(HashMap<Id, crate::collection::Id>);
