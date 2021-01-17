use chrono::{DateTime, FixedOffset};
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

#[derive(Deserialize, Debug)]
pub(super) struct Book {
    title: String,
    author: String,
    editors: Option<Vec<String>>,
    translators: Option<Vec<String>>,
    cover: Option<String>,
    link: Option<String>,
    year: u16,
    review: Review,
}

#[derive(Deserialize, Debug)]
pub(super) struct Review {
    rating: Rating,
    summary: String,
}

#[derive(Deserialize, Debug)]
enum Rating {
    NotRecommended,
    WithQualifications,
    Recommended,
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
