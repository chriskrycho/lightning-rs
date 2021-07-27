pub mod components;
pub mod markdown;
pub mod metadata;

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

use self::{markdown::Processed, metadata::Metadata};

/// Source data for a file: where it came from, and its original contents.
pub struct Source {
    /// Original source location for the file.
    pub path: PathBuf,
    /// Original contents of the file.
    pub contents: String,
}

/// A unique identifier
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Deserialize, Serialize)]
pub struct Id(Uuid);

/// A fully-resolved representation of a page.
///
/// In this struct, the metadata has been parsed and resolved, and the content
/// has been converted from Markdown to HTML and preprocessed with both the
/// templating engine and my typography tooling. It is ready to render into the
/// target layout template specified by its `metadata: ResolvedMetadata` and
/// then to print to the file system.
#[derive(Debug)]
pub struct Page {
    pub id: Id,

    /// The fully-parsed metadata associated with the page.
    pub metadata: Metadata,

    /// The fully-rendered contents of the page.
    pub contents: PostProcessed,
}

impl Page {
    pub fn new(
        source: &Source,
        root_dir: &Path,
        syntax_set: &SyntaxSet,
        config: &Config,
    ) -> Result<Self, String> {
        let id = Id(Uuid::new_v5(
            &Uuid::NAMESPACE_OID,
            source.contents.as_bytes(),
        ));

        let Components { header, body } = Components::try_from(source.contents.as_ref())?;
        let metadata = Metadata::new(&source.path, root_dir, header)?;

        let body = preprocess(body.into(), &config, &metadata);
        let contents = render_markdown(body, syntax_set)?;
        let contents = postprocess(contents, &config, &metadata);

        Ok(Page {
            id,
            metadata,
            contents,
        })
    }

    pub fn path_from_root(&self, root_dir: &Path) -> PathBuf {
        root_dir.join(&self.metadata.slug)
    }

    /// Given a config, generate the (canonicalized) URL for the page
    pub fn url(&self, config: &Config) -> String {
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

pub struct Preprocessed(String);

impl<'a> Preprocessed {
    fn as_str(&'a self) -> &'a str {
        self.0.as_str()
    }
}

/// Ready the text for rendering as markdown
fn preprocess(text: String, _config: &Config, _metadata: &Metadata) -> Preprocessed {
    // TODO: implement *actual* preprocessing using the data:
    //
    // -
    // - substitute all references from metadata
    Preprocessed(text)
}

#[derive(Debug)]
pub struct PostProcessed(String);

impl AsRef<[u8]> for PostProcessed {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl std::fmt::Display for PostProcessed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn postprocess(processed: Processed, _config: &Config, _metadata: &Metadata) -> PostProcessed {
    // TODO: use the config and metadata to substitute the values
    PostProcessed(processed.0)
}
