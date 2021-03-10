mod v1_0;
mod v1_1;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

trait FeedVersion {
    /// The URL of the version of the format the feed uses. This should appear
    /// at the very top, though we recognize that not all JSON generators allow
    /// for ordering.
    const VERSION: &'static str;
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
enum Author {
    AvatarOnly {
        avatar: String,
    },
    NameOnly {
        name: String,
    },
    UrlOnly {
        url: String,
    },
    AvatarAndName {
        avatar: String,
        name: String,
    },
    AvatarAndUrl {
        avatar: String,
        name: String,
    },
    NameAndUrl {
        name: String,
        url: String,
    },
    All {
        avatar: String,
        name: String,
        url: String,
    },
}

/// Traditional feed readers usually poll a web site for changes at a regular
/// interval. This is fine for many applications, but there’s a more efficient
/// approach for applications that need to know the moment a feed changes. The
/// top-level `hubs` array points to one or more services that can be used by
/// feed aggregators to subscribe to changes to this feed. Those hubs can then
/// send a notification to the application as soon as the feed is updated.
///
/// The `type` field describes the protocol used to talk with the hub, such as
/// “rssCloud”or “WebSub.”When using WebSub, the value for the JSON Feed’s
/// `feed_url` is passed for the `hub.topic` parameter. For more information
/// about WebSub, see [the specification at the
/// W3C](https://www.w3.org/TR/websub/).
///
/// **NOTE:** This definition is *extremely* incomplete; it represents only the
/// tiny bits from the JSON Feed website
#[derive(Deserialize, Serialize)]
struct Hub {
    #[serde(alias = "type")]
    r#type: String,
    topic: String,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}
