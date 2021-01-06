use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    url: String,
    repo: String,
    title: Title,
    subtitle: String,
    description: String,
    author: Author,
}

#[derive(Deserialize, Debug)]
struct Title {
    normal: String,
    stylized: String,
}

#[derive(Deserialize, Debug)]
struct Author {
    name: String,
    #[serde(deserialize_with = "email::de_from_str")]
    email: email::Email,
    links: Vec<String>,
}

mod email {
    use std::str::FromStr;

    use lazy_static::lazy_static;
    use regex::Regex;
    use serde::{de, Deserialize, Deserializer};
    use serde_derive::Deserialize;

    /// An incredibly stupid email-"parsing" regex.
    lazy_static! {
        static ref EMAIL_RE: Regex = Regex::new(r"([^@]+)@([^@]+)").unwrap();
    }

    #[derive(Deserialize, Debug)]
    pub(super) struct Email {
        local: String,
        host: String,
    }

    pub(super) fn de_from_str<'de, D>(deserializer: D) -> Result<Email, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Email::from_str(&s).map_err(de::Error::custom)
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
}
