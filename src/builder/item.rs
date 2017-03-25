//! Process individual content items (pages, posts, etc.).


// First-party
use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;

// Third-party
use yaml_rust::{yaml, Yaml, YamlLoader};


pub struct Metadata {
    pub title: Option<String>,
    pub slug: String,
    pub extra: HashMap<String, ExtraMetadata>,
}

pub enum ExtraMetadata {
    SingleLineString(String),
    MultiLineString(String),
    List(Vec<String>),
    Slug(String),
}

const TRIPLE_DASH: &'static str = "---";
const TRIPLE_DOT: &'static str = "...";

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
    let lines_of_interest = |c: &'c str| c.lines().skip_while(|line| *line =="");

    let mut lines = lines_of_interest(content);
    let has_initial = lines.nth(0).map(is_initial_delimiter).unwrap_or(false);
    let has_terminal = lines.any(is_terminal_delimiter);
    if !(has_initial && has_terminal) {
        return None;
    }

    let metadata = lines_of_interest(content)
        .skip(1)  // "---"
        .take_while(|line| !is_terminal_delimiter(&line))  // until "---" or "..."
        .fold(Vec::new(), |mut vec, line| { vec.push(line); vec })
        .join("\n");

    Some(metadata)
}


// TODO: figure out type. HashMap<String, T> where T: ...
pub fn parse_metadata(content: &str, file_name: &Path) -> Result<Metadata, String> {
    let just_metadata = extract_metadata(&content)
        .ok_or(format!("file `{}` passed to `parse_metadata` has no metadata",
                       file_name.to_string_lossy()))?;

    let yaml = YamlLoader::load_from_str(&just_metadata);

    // TODO: Parse from YAML
    let slug = file_name.file_stem()
        .ok_or(format!("file name `{}` passed to `parse_metadata` has no stem",
                       file_name.to_string_lossy()))?
        .to_str()
        .ok_or(format!("file name `{}` passed to `parse_metadata` has invalid UTF-8",
                       file_name.to_string_lossy()))?;

    // TODO: actually, check if on YAML, -> Some if so, None if not.
    let title = slug;

    Ok(Metadata {
           title: Some(slug.to_string()),
           slug: slug.to_string(),
           extra: HashMap::new(),
       })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_metadata() {
        let has_metadata = "---
Title: With Terminal Dashes
---

Some other text!e
        ";

        assert_eq!(
            extract_metadata(&has_metadata),
            Some("Title: With Terminal Dashes".into()));

        let has_metadata_terminal_dots = "---
Title: With Terminal Dots
...

Some other text!
        ";

        assert_eq!(
            extract_metadata(&has_metadata_terminal_dots),
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

        assert_eq!(
            extract_metadata(&has_space_before_opening),
            Some("Title: Who cares how many *initial* empty lines there are?".into()));

        let multiline_metadata = "---
Jedi: Luke Skywalker
Rogue: Han Solo
Badass: Princess Leia
...

Whatever other content...
        ";

        assert_eq!(
            extract_metadata(&multiline_metadata),
            Some("Jedi: Luke Skywalker\nRogue: Han Solo\nBadass: Princess Leia".into()));

        let whole_lines_only_please = "---This: Is Not Valid
Even: Thought it *almost* looks valid.
...
        ";

        assert_eq!(extract_metadata(&whole_lines_only_please), None);
    }
}
