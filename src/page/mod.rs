mod serialize;

use std::{path::PathBuf, unimplemented};

use chrono::{format::ParseError, FixedOffset};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};

struct Page {
    /// The full path to the source input file.
    inputPath: PathBuf,

    /// Mapped from the input file name, useful for permalinks.
    fileSlug: String,

    /// The full path to the output file to be written for this content.
    outputPath: PathBuf,

    /// Url used to link to this piece of content.
    url: String,

    /// The resolved date.
    date: DateTime<FixedOffset>,

    /// The parsed metadata associated with the page.
    metadata: serialize::Metadata,
}

impl Page {
    pub(crate) fn new(src_path: PathBuf, contents: String) -> Page {
        unimplemented!()
    }
}
