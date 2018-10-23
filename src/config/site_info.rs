// Standard library
use std::collections::HashMap;
use std::str::FromStr;

// Third party
use chrono_tz::Tz;
use yaml_rust::{yaml, Yaml};

// First party
pub use validated_types::Url as ValidatedUrl;
use yaml_util::*;

#[derive(Debug, PartialEq)]
pub struct SiteInfo {
    /// The name of the site. Required.
    pub title: String,

    /// The canonical URL for the root of the site. Required.
    pub url: ValidatedUrl,

    /// The default timezone to use with posts. May be any of the full text
    /// strings specified by [the `chrono_tz` crate][chrono_tz].
    ///
    /// [chrono_tz]: https://docs.rs/chrono-tz/
    pub default_timezone: Option<Tz>,

    /// The description of the site. Optional.
    pub description: Option<String>,

    /// Arbitrary metadata associated with the site. Optional.
    pub metadata: HashMap<String, Yaml>,
}

impl SiteInfo {
    pub fn from_yaml(yaml: &yaml::Hash) -> Result<SiteInfo, String> {
        let title = SiteInfo::parse_title(yaml)?;
        let url = SiteInfo::parse_url(yaml)?;
        let description = SiteInfo::parse_description(yaml)?;
        let metadata = SiteInfo::parse_metadata(yaml)?;
        let default_timezone = SiteInfo::parse_default_timezone(yaml)?;

        Ok(SiteInfo {
            title,
            url,
            description,
            metadata,
            default_timezone,
        })
    }

    fn parse_title(yaml: &yaml::Hash) -> Result<String, String> {
        match yaml.get(&Yaml::from_str("title")) {
            Some(Yaml::String(ref string)) => Ok(string.clone()),
            Some(_val) => Err(key_of_type("title", Required::Yes, yaml, "string")),
            _ => Err(required_key("title", yaml)),
        }
    }

    fn parse_url(yaml: &yaml::Hash) -> Result<ValidatedUrl, String> {
        match yaml.get(&Yaml::from_str("url")) {
            Some(Yaml::String(ref string)) => ValidatedUrl::new(string),
            Some(Yaml::Null) => Err(required_key("url", yaml)),
            _ => Err(key_of_type("url", Required::Yes, yaml, "string")),
        }
    }

    fn parse_default_timezone(yaml: &yaml::Hash) -> Result<Option<Tz>, String> {
        let key = "default_timezone";

        match yaml.get(&Yaml::from_str(key)) {
            None | Some(Yaml::Null) => Ok(None),
            Some(Yaml::String(ref string)) => Ok(Some(Tz::from_str(&string)?)),
            _ => Err(key_of_type(key, Required::Yes, yaml, "string (time zone)")),
        }
    }

    fn parse_description(yaml: &yaml::Hash) -> Result<Option<String>, String> {
        let key = "description";
        match yaml.get(&Yaml::from_str(key)) {
            None | Some(Yaml::Null) => Ok(None),
            Some(Yaml::String(ref string)) => Ok(Some(string.clone())),
            _ => Err(key_of_type(key, Required::No, yaml, "string")),
        }
    }

    fn parse_metadata(yaml: &yaml::Hash) -> Result<HashMap<String, Yaml>, String> {
        let key = "metadata";
        let mut metadata = HashMap::new();
        match yaml.get(&Yaml::from_str(key)) {
            None | Some(Yaml::Null) => Ok(metadata),
            Some(Yaml::Hash(ref hash)) => {
                for hash_key in hash.keys() {
                    let hash_key_str = hash_key.as_str().ok_or_else(|| {
                        key_of_type("key of hash map", Required::No, hash, "string")
                    })?;

                    match hash.get(hash_key) {
                        None | Some(&Yaml::Null) => {
                            return Err(key_of_type(hash_key_str, Required::No, hash, "hash"));
                        }
                        Some(inner_yaml @ &Yaml::String(..))
                        | Some(inner_yaml @ &Yaml::Boolean(..))
                        | Some(inner_yaml @ &Yaml::Integer(..))
                        | Some(inner_yaml @ &Yaml::Real(..)) => {
                            let result =
                                metadata.insert(String::from(hash_key_str), inner_yaml.clone());
                            if result.is_some() {
                                let main = format!("Double insertion of key {}.\n", hash_key_str);
                                let detail = format!(
                                    "First: {:?}\nSecond: {:?}",
                                    result.unwrap(),
                                    inner_yaml
                                );
                                return Err(main + &detail);
                            }
                        }
                        _ => {
                            return Err(key_of_type(
                                hash_key_str,
                                Required::No,
                                hash,
                                "string, boolean, or integer",
                            ))
                        }
                    }
                }
                Ok(metadata)
            }
            _ => Err(key_of_type(key, Required::No, yaml, "hash")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono_tz::UTC;
    use std::collections::BTreeMap;
    use validated_types::Url;
    use yaml_rust::YamlLoader;

    fn load_site_info(source: &str) -> BTreeMap<Yaml, Yaml> {
        let mut loaded = YamlLoader::load_from_str(source).unwrap();
        let first = loaded.pop().unwrap();
        first.as_hash().unwrap()[&Yaml::from_str("site_info")]
            .as_hash()
            .unwrap()
            .clone()
    }

    #[test]
    fn parses_title() {
        let site_info_empty_metadata = "
site_info:
    title: lx (lightning)
    url: https://lightning.rs
    description: >
        A ridiculously fast site generator and engine.
    metadata: ~
        ";

        let site_info = load_site_info(site_info_empty_metadata);

        assert_eq!(
            Ok("lx (lightning)".into()),
            SiteInfo::parse_title(&site_info)
        );
    }

    #[test]
    fn parses_url() {
        let site_info_empty_metadata = "
site_info:
    title: lx (lightning)
    url: https://lightning.rs
    description: >
        A ridiculously fast site generator and engine.
    metadata: ~
        ";

        let site_info = load_site_info(site_info_empty_metadata);

        assert_eq!(
            Url::new("https://lightning.rs".into()),
            SiteInfo::parse_url(&site_info)
        );
    }

    #[test]
    fn parses_metadata() {
        let site_info = "\
site_info:
    title: lx (lightning)
    url: https://lightning.rs
    description: >
        A ridiculously fast site generator and engine.
    default_timezone: UTC
    metadata:
        foo: bar
        quux: 2
        ";

        let mut metadata = HashMap::new();
        metadata.insert("foo".into(), Yaml::from_str("bar"));
        metadata.insert("quux".into(), Yaml::from_str("2"));

        let site_info = load_site_info(site_info);
        assert_eq!(Ok(metadata), SiteInfo::parse_metadata(&site_info));
    }

    #[test]
    fn parses_default_timezone() {
        let site_info_empty_metadata = "
site_info:
    title: lx (lightning)
    url: https://lightning.rs
    description: >
        A ridiculously fast site generator and engine.
    default_timezone: UTC
    metadata: ~
        ";

        let site_info = load_site_info(site_info_empty_metadata);

        //let expected = Ok(Some(Tz::from_str("UTC".into()).unwrap()));

        assert_eq!(
            Ok(Some(Tz::UTC)),
            SiteInfo::parse_default_timezone(&site_info)
        );
    }

    #[test]
    fn parses_default_timezone_empty() {
        let site_info_empty_metadata = "
site_info:
    title: lx (lightning)
    url: https://lightning.rs
    description: >
        A ridiculously fast site generator and engine.
    metadata: ~
        ";

        let site_info = load_site_info(site_info_empty_metadata);

        assert_eq!(Ok(None), SiteInfo::parse_default_timezone(&site_info));
    }

    #[test]
    fn parses_site_info() {
        let site_info = "\
site_info:
    title: lx (lightning)
    url: https://lightning.rs
    description: >
        A ridiculously fast site generator and engine.
    default_timezone: UTC
    metadata:
        foo: bar
        quux: 2
        ";

        let mut metadata = HashMap::new();
        metadata.insert("foo".into(), Yaml::from_str("bar"));
        metadata.insert("quux".into(), Yaml::from_str("2"));
        let expected = SiteInfo {
            title: "lx (lightning)".into(),
            url: ValidatedUrl::new("https://lightning.rs").unwrap(),
            description: Some("A ridiculously fast site generator and engine.\n".into()),
            default_timezone: Some(UTC),
            metadata: metadata,
        };

        let site_info = load_site_info(site_info);
        assert_eq!(Ok(expected), SiteInfo::from_yaml(&site_info));
    }

    #[test]
    fn parses_site_info_with_empty_metadata() {
        let site_info_empty_metadata = "
site_info:
    title: lx (lightning)
    url: https://lightning.rs
    description: >
        A ridiculously fast site generator and engine.
    default_timezone: UTC
    metadata: ~
        ";

        let expected = SiteInfo {
            title: "lx (lightning)".into(),
            url: ValidatedUrl::new("https://lightning.rs").unwrap(),
            description: Some("A ridiculously fast site generator and engine.\n".into()),
            default_timezone: Some(UTC),
            metadata: HashMap::new(),
        };

        let site_info = load_site_info(site_info_empty_metadata);

        assert_eq!(Ok(expected), SiteInfo::from_yaml(&site_info));
    }
}
