use std::convert::TryFrom;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref COMPONENTS: Regex =
        Regex::new(r"^[[:space:]]*---(?P<header>\n(?s).*?(?-s))---\n(?P<body>(?s).*(?-s))$")
            .expect("Regex is legit");
}

pub(crate) struct Components<'c> {
    pub(crate) header: &'c str,
    pub(crate) body: &'c str,
}

impl<'s> TryFrom<&'s str> for Components<'s> {
    type Error = String;

    fn try_from(source: &'s str) -> Result<Self, Self::Error> {
        let captures = COMPONENTS
            .captures(source)
            .ok_or("invalid/missing YAML header")?;

        let header = captures
            .name("header")
            .expect("regex capture 'header' cannot be missing")
            .as_str();

        let body = captures
            .name("body")
            .expect("regex capture 'body' cannot be missing")
            .as_str();

        Ok(Components { header, body })
    }
}
