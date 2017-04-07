//! Items are the fundamental building blocks of the content of a site.

pub mod metadata;

use chrono::TimeZone;

pub use self::metadata::{Metadata,ExtraMetadata};

/// A page or post or other such *item* of content.
pub enum Item<Tz: TimeZone> {
    /// An `Item` whose `content` has been converted from Markdown to HTML.
    Processed {
        content: String,
        metadata: Metadata<Tz>,
    },

    /// An `Item` whose `content` remains as plain-text/Markdown.
    Unprocessed {
        content: String,
        metadata: Metadata<Tz>,
    }
}
