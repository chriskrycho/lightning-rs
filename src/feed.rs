mod json;

use json_feed::JSONFeed;

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
    pub(crate) fn new(title: String, site_config: &'a Config, items: &'a [Page]) -> Feed<'a> {
        Feed {
            title,
            site_config,
            items,
        }
    }
}

impl<'a> From<Feed<'a>> for JSONFeed<'a> {
    fn from(feed: Feed<'a>) -> Self {
        todo!()
    }
}
