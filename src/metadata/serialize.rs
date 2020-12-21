use chrono::{DateTime, FixedOffset};
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub(super) struct Page {
    title: Option<String>,
    subtitle: Option<String>,
    summary: Option<String>,
    qualifiers: Option<Qualifiers>,
    date: Option<DateTime<FixedOffset>>,
    updated: Option<DateTime<FixedOffset>>,
    permalink: Option<String>,
    thanks: Option<String>,
    tags: Vec<String>,
    featured: bool,
    layout: Option<String>,
    series: Option<Series>,
}

#[derive(Deserialize)]
struct Qualifiers {
    audience: Option<String>,
    epistemic: Option<String>,
}

#[derive(Deserialize)]
struct Book {
    title: String,
    author: String,
    editors: Vec<String>,
    translators: Vec<String>,
    cover: Option<String>,
    link: Option<String>,
    year: u16,
    review: Review,
}

#[derive(Deserialize)]
struct Review {
    rating: Rating,
    summary: String,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
struct Series {
    name: String,
    part: u8,
}
