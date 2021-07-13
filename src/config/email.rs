use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::{de, Deserialize, Deserializer};

lazy_static! {
    /// An incredibly stupid email-"parsing" regex.
    static ref EMAIL_RE: Regex = Regex::new(r"([^@]+)@([^@]+)").unwrap();
}

#[derive(Deserialize, Debug)]
pub(super) struct Email {
    /// The username, the bit before the `@`
    local: String,
    /// The email host, the bit after the `@`
    host: String,
}

impl Email {
    pub(super) fn de_from_str<'de, D>(deserializer: D) -> Result<Email, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Email::from_str(&s).map_err(de::Error::custom)
    }
}

impl std::str::FromStr for Email {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        EMAIL_RE
            .captures(s)
            .ok_or(format!("could not parse {}", s))
            .and_then(|captures| match (captures.get(0), captures.get(1)) {
                (Some(local), Some(host)) => Ok(Email {
                    local: local.as_str().to_owned(),
                    host: host.as_str().to_owned(),
                }),
                (Some(..), None) => Err(format!("missing host name in {}", s)),
                (None, Some(..)) => Err(format!("missing username in {}", s)),
                _ => Err(format!("could not parse {}", s)),
            })
            .map_err(|e| format!("email validation error: {}", e))
    }
}
