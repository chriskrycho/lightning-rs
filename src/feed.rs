mod json;

use std::convert::TryFrom;

use lx_json_feed::{AuthorOptions, JSONFeed};

use crate::{config::Config, page::Page};

/// Required resources for a `Feed`.
pub(crate) struct Feed<'a> {
    /// Every feed has its own title.
    title: String,

    /// Feeds also need read access to the site config to be able to render the
    /// full set of data specified for Atom, JSON, or RSS.
    site_config: &'a Config,

    /// The set of items to render in the feed. A read-only slice because I will
    /// never actually need to *write* to these. I just need the parsed metadata
    /// and rendered HTML contents of the page, to render into the template.
    items: &'a [Page],
}

impl<'a> Feed<'a> {
    pub(crate) fn _new(title: String, site_config: &'a Config, items: &'a [Page]) -> Feed<'a> {
        Feed {
            title,
            site_config,
            items,
        }
    }
}

impl<'a> TryFrom<Feed<'a>> for JSONFeed {
    type Error = String;

    fn try_from(feed: Feed<'a>) -> Result<Self, Self::Error> {
        let items = feed.items.iter().map(|page| page.into()).collect();
        let feed = JSONFeed::builder(&feed.title, items)
            .with_author(&AuthorOptions {
                name: Some(&feed.site_config.author.name),
                url: None,
                avatar: None,
            })?
            .build();

        Ok(feed)
    }
}
