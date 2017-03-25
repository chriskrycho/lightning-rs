//! Process individual content items (pages, posts, etc.).


// First-party
use std::collections::HashMap;
use std::path::Path;

// Third-party
use yaml_rust::{yaml, Yaml, YamlLoader};


/// The metadata for each item, with common data natively on the struct.
pub struct Metadata {
    /// An item may or may not have a title, but it must explicitly opt out.
    pub title: Option<String>,
    /// The `slug` may be auto-generated, but is required.
    pub slug: String,

    pub date: Option<Date>,
    pub extra: HashMap<String, ExtraMetadata>,
}


/// Items may have metadata beyond the slug and title.
pub enum ExtraMetadata {
    /// A value consisting of a string with no newlines in it.
    SingleLineString(String),
    /// A value consisting of
    MultiLineString(String),
    List(Vec<String>),
    Slug(String),
}

// TODO: figure out type. HashMap<String, T> where T: ...
pub fn parse_metadata(content: &str, file_name: &Path) -> Result<Metadata, String> {
    let just_metadata = extract_metadata(&content);
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
        date: None,
        extra: HashMap::new(),
    })
}


fn extract_metadata<'c>(content: &'c str) -> &'c str {
    content  // FIXME: this is not it!
}
