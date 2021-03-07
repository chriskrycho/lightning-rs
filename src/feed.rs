mod json;

use json_feed::JSONFeed;

use crate::page::Page;

/// Required resources for a `Feed`.
pub(crate) struct Feed<'a> {
    /// Every feed has its own title.
    title: String,
    /// The set of items to render in the feed. A read-only slice because I will
    /// never actually need to *write* to these. I just need the parsed metadata
    /// and rendered HTML contents of the page, to render into the template.
    items: &'a [Page],
}

impl<'a> Feed<'a> {
    pub(crate) fn new(title: String, items: &'a [Page]) -> Feed<'a> {
        Feed { title, items }
    }
}

impl<'a> From<Feed<'a>> for JSONFeed<'a> {
    fn from(feed: Feed<'a>) -> Self {
        todo!()
    }
}
