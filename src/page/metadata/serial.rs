//! The serialization inputs for metadata. Covers both YAML metadata in headers
//! and associated data from JSON/TOML/YAML/JSON5/whatever else I decide to
//! support in data files.

use std::str::FromStr;

use chrono::{DateTime, FixedOffset};
use serde::{de, Deserialize, Deserializer};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub(super) struct Metadata {
    pub(super) title: Option<String>,
    pub(super) subtitle: Option<String>,
    pub(super) summary: Option<String>,
    pub(super) qualifiers: Option<Qualifiers>,
    pub(super) date: Option<DateTime<FixedOffset>>,
    pub(super) updated: Option<DateTime<FixedOffset>>,
    pub(super) permalink: Option<String>,
    pub(super) thanks: Option<String>,
    #[serde(default)]
    pub(super) tags: Vec<String>,
    #[serde(default)]
    pub(super) featured: bool,
    pub(super) layout: Option<String>,
    pub(super) book: Option<Book>,
    pub(super) series: Option<Series>,
    pub(super) subscribe: Option<Subscribe>,
}

#[derive(Deserialize, Debug)]
pub(super) struct Qualifiers {
    audience: Option<String>,
    epistemic: Option<String>,
}

#[derive(Deserialize, Debug)]
pub(super) struct Subscribe {
    atom: Option<String>,
    json: Option<String>,
}

// TODO: This is correct for final output, but I *currently* have a bunch of
// things being handled by "the cascade" in 11ty, and this *cannot* handle that.
// As with a bunch of other things, the input from disk should have more options
// and then the final merged data fewer.
#[derive(Deserialize, Debug)]
pub(super) struct Book {
    title: Option<String>,
    author: Option<String>,
    /// Year is a `String`, rather than something like a `u16`, because years
    /// are a lot more complicated than a number represents. If I write "400
    /// B.C.", for example, the system should still work.
    year: Option<String>,
    editors: Option<Vec<String>>,
    translators: Option<Vec<String>>,
    cover: Option<String>,
    link: Option<String>,
    review: Option<Review>,
}

#[derive(Deserialize, Debug)]
pub(super) struct Review {
    rating: Rating,
    summary: String,
}

// TODO: right now this assumes it can be deserialized from the associated text,
// but in fact it should be derived from the same text as its `Display`
// implementation below. (A later enhancement: converting "****" etc. to it or
// something cool like that.)
#[derive(Deserialize, Debug)]
enum Rating {
    #[serde(rename = "Not recommended")]
    NotRecommended,
    #[serde(rename = "Recommended with qualifications")]
    WithQualifications,
    #[serde(rename = "Recommended")]
    Recommended,
    #[serde(rename = "Required")]
    Required,
}

impl std::fmt::Display for Rating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rating::NotRecommended => "Not recommended",
                Rating::WithQualifications => "Recommended with qualifications",
                Rating::Recommended => "Recommended",
                Rating::Required => "Required",
            }
        )
    }
}

#[derive(Deserialize, Debug)]
pub(super) struct Series {
    name: String,
    part: u8,
}
