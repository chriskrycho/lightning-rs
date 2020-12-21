use chrono::{format::ParseError, FixedOffset};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};

struct Page {
    /// the full path to the source input file (including the path to the input directory)
    inputPath: String,

    /// Mapped from the input file name, useful for permalinks. Read more about
    fileSlug: String,
    /** the full path to the output file to be written for this content */
    outputPath: String,

    /// url used to link to this piece of content.
    url: String,

    /// the resolved date used for sorting. Read more about [Content Dates].
    date: DateTime<FixedOffset>,
}
