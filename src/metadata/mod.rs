mod serialize;

use std::unimplemented;

use chrono::{format::ParseError, FixedOffset};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};

struct Page {
    /// The full path to the source input file.
    inputPath: String,

    /// Mapped from the input file name, useful for permalinks.
    fileSlug: String,

    /// The full path to the output file to be written for this content.
    outputPath: String,

    /// Url used to link to this piece of content.
    url: String,

    /// The resolved date.
    date: DateTime<FixedOffset>,
}

impl From<serialize::Page> for Page {
    fn from(_: serialize::Page) -> Self {
        unimplemented!()
    }
}
