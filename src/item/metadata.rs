//! Process individual content items (pages, posts, etc.).


// First-party
use std::collections::HashMap;
use std::error::Error;

// Third-party
use chrono::{DateTime, FixedOffset, Local, LocalResult, TimeZone};
use chrono::NaiveDateTime;
use yaml_rust::YamlLoader;

// First-party
use yaml_util::*;
use config::Taxonomies;
use config::taxonomy::Taxonomy;


pub type OtherMetadata = HashMap<String, OtherMetadatum>;

pub struct Metadata {
    pub title: String,
    pub slug: String,
    pub date: Option<DateTime<FixedOffset>>,
    pub taxonomies: Taxonomies, 
    pub other: OtherMetadata, 
}

pub struct Defaults {
    pub slug: String,
}

impl Metadata {
    /// Extract the metadata for an item from its body.
    ///
    /// - `defaults`: used as a fallback wherever the required item is not
    ///   present in the supplied content.
    /// - `tz`: the time zone to use if the item includes a `Taxonomy::Temporal`
    ///   field.
    /// - `taxonomies`:
    pub fn from_content(
        content: &str,
        defaults: Defaults,
        date_format: &str,
        tz: Option<FixedOffset>,
        taxonomy_configs: &Taxonomies,
    ) -> Result<Metadata, String> {

        let metadata = extract_metadata(&content)
            .ok_or(format!(
                "content passed to `Metadata::parse` has no metadata and no default"
            ))?;

        let bad_yaml_message = |reason: &str| {
            format!(
                "content passed to `Metadata::parse` had invalid metadata: {}\n{}",
                metadata,
                reason
            )
        };

        let yaml = YamlLoader::load_from_str(&metadata)
            .map_err(|reason| bad_yaml_message(&reason.description()))?;

        let yaml = yaml.into_iter()
            .next()
            .ok_or(bad_yaml_message("empty metadata block"))?;

        let yaml = yaml.as_hash()
            .ok_or(bad_yaml_message("could not parse item as metadata hash"))?;

        let slug = case_insensitive_string("slug", yaml, Required::No)?
            .unwrap_or(defaults.slug.clone());

        let title = case_insensitive_string("title", yaml, Required::No)?
            .unwrap_or("".into());

        // TODO: use taxonomy configs or fall back to defaults.
        let naive_date_time_result = case_insensitive_string("date", yaml, Required::No)?
            .map(|supplied_value| {
                NaiveDateTime::parse_from_str(&supplied_value, date_format)
            });

        // TODO: extract into function; this is gross.
        let date = match naive_date_time_result {
            Some(Err(parse_error)) => {
                return Err(format!("{}", parse_error));
            }
            Some(Ok(naive_date_time)) => {
                let offset = if let Some(offset) = tz {
                    offset
                } else {
                    let local = Local;
                    let local_offset = local.offset_from_local_datetime(&naive_date_time).single();
                    local_offset.unwrap_or(FixedOffset::east(0))
                };

                match offset.from_local_datetime(&naive_date_time) {
                    LocalResult::None |
                    LocalResult::Ambiguous(_, _) => None,
                    LocalResult::Single(date_time) => Some(date_time),
                }
            }
            None => None,
        };

        let taxonomies = HashMap::new();
        let other = HashMap::new();

        Ok(Metadata {
            date,
            title,
            slug,
            taxonomies,
            other,
        })
    }
}

pub enum OtherMetadatum {
    SingleLineString(String),
    MultiLineString(String),
    List(Vec<String>),
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

fn extract_taxonomies(content: &str, config_taxonomies: Vec<Taxonomy>) -> Vec<String> {
    unimplemented!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_basic_metadata() {
        let source = "---
Title: With Terminal Dashes
---

Some other text!
        ";

        assert_eq!(
            extract_metadata(&source),
            Some("Title: With Terminal Dashes".into())
        );
    }

    #[test]
    fn extract_terminal_dots() {
        let source = "---
Title: With Terminal Dots
...

Some other text!
        ";

        assert_eq!(
            extract_metadata(&source),
            Some("Title: With Terminal Dots".into())
        );
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

        assert_eq!(
            extract_metadata(&source),
            Some("Title: Who cares how many *initial* empty lines there are?".into())
        );
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

        assert_eq!(
            extract_metadata(&source),
            Some("Jedi: Luke Skywalker\nRogue: Han Solo\nBadass: Princess Leia".into())
        );

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

        assert_eq!(
            extract_metadata(&source),
            Some("Title: This is fine.\nSubtitle: Even with all the spaces.\n".into())
        )
    }

    #[test]
    fn blank_spaces_anywhere_in_block() {
        let source = "---

Title: They Can Be At The Start

Subtitle: Or even in the middle...



And: they can even be multiple lines long.

---
        ";

        assert_eq!(
            extract_metadata(&source),
            Some(
                String::from("\nTitle: They Can Be At The Start\n\n") +
                    "Subtitle: Or even in the middle...\n\n\n\n" +
                    "And: they can even be multiple lines long.\n"
            )
        );
    }
}
