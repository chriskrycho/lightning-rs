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
    fn extract_basic() {
        let source = "---
Title: With Terminal Dashes
---

Some other text!e
        ";

        assert_eq!(extract_metadata(&source),
                   Some("Title: With Terminal Dashes".into()));
    }

    #[test]
    fn extract_terminal_dots() {
        let source = "---
Title: With Terminal Dots
...

Some other text!
        ";

        assert_eq!(extract_metadata(&source),
                   Some("Title: With Terminal Dots".into()));
    }

    #[test]
    fn no_metadata() {
        let source = "
Some text, no metadata though.
        ";

        assert_eq!(extract_metadata(&source), None);
    }

    #[test]
    fn opening_but_no_closing() {
        let source = "---
This is *not* metadata; it has an opening `<hr/>`, which is weird.
        ";

        assert_eq!(extract_metadata(&source), None);
    }

    #[test]
    fn closing_but_no_opening() {
        let source = "
Whatever this says is irrelevant.
...
        ";

        assert_eq!(extract_metadata(&source), None);
    }

    #[test]
    fn space_before_opening() {
        let source = "

---
Title: Who cares how many *initial* empty lines there are?
---
        ";

        assert_eq!(extract_metadata(&source),
                   Some("Title: Who cares how many *initial* empty lines there are?".into()));
    }

    #[test]
    fn multiline_metadata() {
        let source = "---
Jedi: Luke Skywalker
Rogue: Han Solo
Badass: Princess Leia
...

Whatever other content...
        ";

        assert_eq!(extract_metadata(&source),
                   Some("Jedi: Luke Skywalker\nRogue: Han Solo\nBadass: Princess Leia".into()));

    }

    #[test]
    fn whole_lines_required() {
        let source = "---This: Is Not Valid
Even: Thought it *almost* looks valid.
...
        ";

        assert_eq!(extract_metadata(&source), None);
    }

    #[test]
    fn blank_spaces_before_closing() {
        let source = "---
Title: This is fine.
Subtitle: Even with all the spaces.

...
        ";

        assert_eq!(extract_metadata(&source),
                   Some("Title: This is fine.\nSubtitle: Even with all the spaces.\n".into()))
    }

    #[test]
    fn blank_spaces_anywhere_in_block() {
        let source = "---

Title: They Can Be At The Start

Subtitle: Or even in the middle...



And: they can even be multiple lines long.

---
        ";

        assert_eq!(extract_metadata(&source),
                   Some(String::from("\nTitle: They Can Be At The Start\n\n") +
                        "Subtitle: Or even in the middle...\n\n\n\n" +
                        "And: they can even be multiple lines long.\n"));
    }
}
