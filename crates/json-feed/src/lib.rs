//! A Rust implementation of [JSON Feed](https://jsonfeed.org) v1.1. (Since v1.1
//! is totally backwards compatible with v1.0, there is no particular reason to
//! supply a v1.0 implementation.)

// NOTE: the above comment notwithstanding, the crate is organized using common
// types in the root here and then a `v1_1` module so that if it *becomes*
// useful to extract a separate `v1_0` interface, it's trivial to do so.

mod v1_1;

use std::{collections::HashMap, convert::TryFrom};

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub use v1_1::{AuthorOptions, Builder as JSONFeedBuilder, JSONFeed};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Version {
    /// The feed is [v1](https://jsonfeed.org/version/1).
    V1_0,
    /// The feed is [v1.1](https://jsonfeed.org/version/1.1).
    V1_1,
}

impl TryFrom<&str> for Version {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "https://jsonfeed.org/version/1" => Ok(Version::V1_0),
            "https://jsonfeed.org/version/1.1" => Ok(Version::V1_1),
            bad_version => Err(format!("Bad JSON Feed `version` field: '{}'", bad_version)),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Author {
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
        url: String,
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
#[derive(Clone, Deserialize, Serialize)]
pub struct Hub {
    #[serde(alias = "type")]
    r#type: String,
    topic: String,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_version() {
        let v1 = "https://jsonfeed.org/version/1";
        assert_eq!(Version::try_from(v1), Ok(Version::V1_0));

        let v1_1 = "https://jsonfeed.org/version/1.1";
        assert_eq!(Version::try_from(v1_1), Ok(Version::V1_1));

        let bad = "whatever";
        assert_eq!(
            Version::try_from(bad),
            Err(format!("Bad JSON Feed `version` field: '{}'", bad))
        );
    }
}
