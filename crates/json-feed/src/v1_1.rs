use serde::{Deserialize, Serialize};

use super::{Author, Hub};

#[derive(Deserialize, Serialize)]
pub struct JSONFeed {
    /// The URL of the version of the format the feed uses. This should appear
    /// at the very top, though we recognize that not all JSON generators allow
    /// for ordering.
    version: String,

    /// The name of the feed, which will often correspond to the name of the
    /// website (blog, for instance), though not necessarily
    title: String,

    /// (optional but strongly recommended) The URL of the resource that the
    /// feed describes. This resource may or may not actually be a “home” page,
    /// but it should be an HTML page. If a feed is published on the public web,
    /// this should be considered as required. But it may not make sense in the
    /// case of a file created on a desktop computer, when that file is not
    /// shared or is shared only privately.
    home_page_url: Option<String>,

    /// (optional but strongly recommended) The URL of the feed, and serves as
    /// the unique identifier for the feed. As with `home_page_url`, this should
    /// be considered required for feeds on the public web.
    feed_url: Option<String>,

    /// Provides more detail, beyond the `title`, on what the feed is about. A
    /// feed reader may display this text.
    description: Option<String>,

    /// Description of the purpose of the feed. This is for the use of people
    /// looking at the raw JSON, and should be ignored by feed readers.
    user_comment: Option<String>,

    /// The URL of a feed that provides the next n items, where n is determined
    /// by the publisher. This allows for pagination, but with the expectation
    /// that reader software is not required to use it and probably won’t use it
    /// very often. `next_url` must not be the same as `feed_url`, and it must
    /// not be the same as a previous `next_url` (to avoid infinite loops).
    next_url: Option<String>,

    /// The URL of an image for the feed suitable to be used in a timeline, much
    /// the way an avatar might be used. It should be square and relatively
    /// large ― such as 512 x 512 ― so that it can be scaled-down and so that it
    /// can look good on retina displays. It should use transparency where
    /// appropriate, since it may be rendered on a non-white background.
    icon: Option<String>,

    /// The URL of an image for the feed suitable to be used in a source list.
    /// It should be square and relatively small, but not smaller than 64 x 64
    /// (so that it can look good on retina displays). As with `icon`, this
    /// image should use transparency where appropriate, since it may be
    /// rendered on a non-white background.
    favicon: Option<String>,

    /// Specifies the feed author. The author object has several members. These
    /// are all optional ― but if you provide an author object, then at least
    /// one is required.
    author: Option<Author>,

    /// Says whether or not the feed is finished ― that is, whether or not it
    /// will ever update again. A feed for a temporary event, such as an
    /// instance of the Olympics, could expire. If the value is true, then it’s
    /// expired. Any other value, or the absence of expired, means the feed may
    /// continue to update.
    expired: Option<bool>,

    /// Describes endpoints that can be used to subscribe to real-time
    /// notifications from the publisher of this feed. Each object has a type
    /// and url, both of which are required. See the section [“Subscribing to
    /// Real-time Notifications”] for details.
    ///
    /// [“Subscribing to Real-time Notifications”]:
    /// https://jsonfeed.org/version/1#subscribing-to-real-time-notifications
    hubs: Option<Vec<Hub>>,

    /** The items in the feed. */
    items: Vec<FeedItem>,
}

impl<'a> super::FeedVersion for JSONFeed {
    const VERSION: &'static str = "https://jsonfeed.org/version/1";
}

#[derive(Deserialize, Serialize)]
struct FeedItem {
    /// A unique identifier for that item for that feed over time. If an item is
    /// ever updated, the `id` should be unchanged. New items should never use a
    /// previously-used `id`. Ideally, the `id` is the full URL of the resource
    /// described by the item, since URLs make great unique identifiers.
    id: String,

    /// The URL of the resource described by the item. It’s the permalink. This
    /// may be the same as the id ― but should be present regardless.
    url: Option<String>,

    /// The URL of a page elsewhere. This is especially useful for linkblogs. If
    /// `url` links to where you’re talking about a thing, then `external_url`
    /// links to the thing you’re talking about.
    external_url: Option<String>,

    /// Plain text. Microblog items in particular may omit titles.
    title: Option<String>,

    /// The plain text of the item.
    ///
    /// At least one of `content_html` and `content_text` must be present. A
    /// Twitter-like service might use `content_text`, while a blog might use
    /// `content_html`. Use whichever makes sense for your resource. (It doesn’t
    /// even have to be the same for each item in a feed.)
    content_text: Option<String>,

    /// The HTML of the item. Important: the only place HTML is allowed in this
    /// format is in `content_html`.
    ///
    /// At least one of `content_html` and `content_text` must be present. A
    /// Twitter-like service might use `content_text`, while a blog might use
    /// `content_html`. Use whichever makes sense for your resource. (It doesn’t
    /// even have to be the same for each item in a feed.)
    content_html: Option<String>,

    /// A plain text sentence or two describing the item. This might be
    /// presented in a timeline, for instance, where a detail view would display
    /// all of `content_html` or `content_text`.
    summary: Option<String>,

    /// The URL of the main image for the item. This image may also appear in
    /// the `content_html` ― if so, it’s a hint to the feed reader that this is
    /// the main, featured image. Feed readers may use the image as a preview
    /// (probably resized as a thumbnail and placed in a timeline).
    image: Option<String>,

    /// The URL of an image to use as a banner. Some blogging systems (such as
    /// [Medium]) display a different banner image chosen to go with each post,
    /// but that image wouldn’t otherwise appear in the `content_html`. A feed
    /// reader with a detail view may choose to show this banner image at the
    /// top of the detail view, possibly with the title overlaid.
    ///
    /// [Medium]: https://medium.com/
    banner_image: Option<String>,

    /// Specifies the date in [RFC 3339](https://tools.ietf.org/html/rfc3339)
    /// format. (Example: `2010-02-07T14:04:00-05:00`.)
    date_published: Option<String>,

    /// Specifies the modification date in [RFC 3339] format.
    ///
    /// [RFC 3339]: https://www.ietf.org/rfc/rfc3339.txt
    date_modified: Option<String>,

    /// The same structure as the top-level `author`. If not specified in an
    /// item, then the top-level `author`, if present, is the author of the
    /// item.
    author: Option<Author>,

    /// Any plain text values you want. Tags tend to be just one word, but they
    /// may be anything. Note: they are not the equivalent of Twitter hashtags.
    /// Some blogging systems and other feed formats call these categories.
    tags: Option<String>,

    /// An individual item may have one or more attachments. List related
    /// resources. Podcasts, for instance, would include an attachment that’s an
    /// audio or video file.
    attachments: Option<Vec<Attachment>>,
}

#[derive(Deserialize, Serialize)]
pub struct Attachment {
    // Yeah, this is an odd type -- but that's because it's representing a JS
    // number, which is... an odd type.
    /// Specifies how long it takes to listen to or watch, when played at normal
    /// speed
    duration_in_seconds: Option<f64>,

    /// Specifies the type of the attachment, such as “audio/mpeg.”
    mime_type: String,

    // As above. No, this doesn't make any sense in the usual case, but we're
    // generating JSON. :shrug:
    /// Specifies how large the file is.
    size_in_bytes: Option<f64>,

    /// A name for the attachment. Important: if there are multiple attachments,
    /// and two or more have the exact same `title` (when `title` is present),
    /// then they are considered as alternate representations of the same thing.
    /// In this way a podcaster, for instance, might provide an audio recording
    /// in different formats.
    title: Option<String>,

    /// Specifies the location of the attachment.
    url: String,
}
