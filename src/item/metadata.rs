//! Process individual content items (pages, posts, etc.).


// First-party
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

// Third-party
use chrono::{DateTime, TimeZone};
use yaml_rust::{yaml, Yaml, YamlLoader};

// First-party
use yaml_util::*;


pub struct Metadata<Tz: TimeZone> {
    pub title: String,
    pub slug: String,
    pub date: Option<DateTime<Tz>>,
    pub extra: Option<HashMap<String, ExtraMetadata>>,
}

pub struct MetadataFromFile<Tz: TimeZone> {
    pub title: String,
    pub date: Option<DateTime<Tz>>,
    pub extra: Option<HashMap<String, ExtraMetadata>>,
}

impl<Tz> Metadata<Tz>
    where Tz: TimeZone
{
    pub fn parse(content: &str, file_name: &Path, tz: Tz) -> Result<Metadata<Tz>, String> {
        let metadata = extract_metadata(&content)
            .ok_or(format!("file `{}` passed to `Metadata::parse` has no metadata",
                           file_name.to_string_lossy()))?;

        let bad_yaml_message = |reason: &str| {
            format!("file `{}` passed to `Metadata::parse` had invalid metadata: {}\n{}",
                    file_name.to_string_lossy(),
                    metadata,
                    reason)
        };

        let yaml = YamlLoader::load_from_str(&metadata)
            .map_err(|reason| bad_yaml_message(&reason.description()))?;

        let yaml = yaml.into_iter()
            .next()
            .ok_or(bad_yaml_message("empty metadata block"))?;

        let yaml = yaml.as_hash()
            .ok_or(bad_yaml_message("could not parse as hash"))?;

        // TODO: Parse from YAML
        let slug = file_name
            .file_stem()
            .ok_or(format!("file name `{}` passed to `Metadata::parse` has no stem",
                           file_name.to_string_lossy()))?
            .to_str()
            .ok_or(format!("file name `{}` passed to `Metadata::parse` has invalid UTF-8",
                           file_name.to_string_lossy()))?;

        let title = case_insensitive_string("title", "Title", yaml, Required::No)
            .unwrap_or("".into());

        Ok(Metadata {
               title: title,
               date: None,
               slug: slug.to_string(),
               extra: None,
           })
    }
}

pub enum ExtraMetadata {
    SingleLineString(String),
    MultiLineString(String),
    List(Vec<String>),
    Slug(String),
}

const TRIPLE_DASH: &str = "---";
const TRIPLE_DOT: &str = "...";

fn is_initial_delimiter(line: &str) -> bool {
    line == TRIPLE_DASH
}

fn is_terminal_delimiter(line: &str) -> bool {
    match line {
        TRIPLE_DASH | TRIPLE_DOT => true,
        _ => false,
    }
}

fn extract_metadata<'c>(content: &'c str) -> Option<String> {
    let lines_of_interest = |c: &'c str| c.lines().skip_while(|&line| line == "");

    let mut lines = lines_of_interest(content);
    let has_initial = lines.nth(0).map(is_initial_delimiter).unwrap_or(false);
    let has_terminal = lines.any(is_terminal_delimiter);
    if !(has_initial && has_terminal) {
        return None;
    }

    let metadata = lines_of_interest(content)
        .skip(1)  // the initial "---"
        .take_while(|line| !is_terminal_delimiter(&line))  // until "---" or "..."
        .fold(Vec::new(), |mut vec, line| { vec.push(line); vec })
        .join("\n");

    Some(metadata)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_extract_metadata() {
        let has_metadata = "---
Title: With Terminal Dashes
---

Some other text!e
        ";

        assert_eq!(extract_metadata(&has_metadata),
                   Some("Title: With Terminal Dashes".into()));

        let has_metadata_terminal_dots = "---
Title: With Terminal Dots
...

Some other text!
        ";

        assert_eq!(extract_metadata(&has_metadata_terminal_dots),
                   Some("Title: With Terminal Dots".into()));

        let no_metadata = "
Some text, no metadata though.
        ";

        assert_eq!(extract_metadata(&no_metadata), None);

        let has_opening_but_no_terminal = "---
This is *not* metadata; it has an opening `<hr/>`, which is weird.
        ";

        assert_eq!(extract_metadata(&has_opening_but_no_terminal), None);

        let has_closing_but_no_opening = "
Whatever this says is irrelevant.
...
        ";

        assert_eq!(extract_metadata(&has_closing_but_no_opening), None);

        let has_space_before_opening = "

---
Title: Who cares how many *initial* empty lines there are?
---
        ";

        assert_eq!(extract_metadata(&has_space_before_opening),
                   Some("Title: Who cares how many *initial* empty lines there are?".into()));

        let multiline_metadata = "---
Jedi: Luke Skywalker
Rogue: Han Solo
Badass: Princess Leia
...

Whatever other content...
        ";

        assert_eq!(extract_metadata(&multiline_metadata),
                   Some("Jedi: Luke Skywalker\nRogue: Han Solo\nBadass: Princess Leia".into()));

        let whole_lines_only_please = "---This: Is Not Valid
Even: Thought it *almost* looks valid.
...
        ";

        assert_eq!(extract_metadata(&whole_lines_only_please), None);
    }
}
