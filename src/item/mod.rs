//! Items are the fundamental building blocks of the content of a site.

pub mod metadata;
pub mod taxonomy;

use std::path::Path;

use chrono::FixedOffset;

pub use self::metadata::{Defaults, Metadata};
use config::Config;

/// A page or post or other such *item* of content.
pub enum Item {
    /// An `Item` whose `content` has been converted from Markdown to HTML.
    Processed { content: String, metadata: Metadata },

    /// An `Item` whose `content` remains as plain-text/Markdown.
    Unprocessed { content: String, metadata: Metadata },
}

impl Item {
    pub fn from_str_unprocessed(
        content: &str,
        file_name: &Path,
        tz: FixedOffset,
        config: &Config,
    ) -> Result<Item, String> {
        let defaults = Defaults {
            slug: slug_from_file_name(file_name)?,
        };

        // TODO: MetaData::from_content() should probably not require all this
        // to be passed in like this. Maybe a builder of some sort? Or have it
        // take some default values?
        Ok(Item::Unprocessed {
            content: content.to_string(),
            metadata: Metadata::from_content(
                content,
                defaults,
                "%Y-%m-%d %H:%M", // TODO: from config?
                Some(tz),
                config,
            )?,
        })
    }
}

fn slug_from_file_name(file_name: &Path) -> Result<String, String> {
    let stem = file_name.file_stem().ok_or_else(|| {
        format!(
            "file name `{}` passed to `Metadata::parse` has no stem",
            file_name.to_string_lossy()
        )
    })?;

    let slug = stem
        .to_str()
        .ok_or_else(|| {
            format!(
                "file name `{}` passed to `Metadata::parse` has invalid UTF-8",
                file_name.to_string_lossy()
            )
        })?.into();

    Ok(slug)
}
