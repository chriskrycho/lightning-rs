//! Provide syntax highlighting via [Syntect].
//!
//! [Syntect]: https://docs.rs/syntect/1.0.0/syntect/

// Standard library
use std::collections::HashMap;
use std::default::Default;
use std::str;

// Third party
use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesText;
use quick_xml::events::Event;
use quick_xml::{Reader, Writer};
use syntect::highlighting::Theme;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::{SyntaxReference, SyntaxSet};

/// A `Language` is a `String` representing a code highlighting language.
type Language = String;

/// Define XML parsing states of interest for highlighting.
///
/// We use these in conjunction with `ParseEvent`s to drive a state machine
/// which tracks whether a stream of `quick_xml::Event`s indicates the presence
/// of a code block to be highlighted.
#[derive(Clone, Debug, PartialEq)]
enum ParseState {
    /// Not in a code block at all.
    NotInBlock,
    /// Possibly the start of a highlight block, with the language as payload.
    MaybeStartBlock(Language),
    /// The start of the code block has been confirmed; highlight next text node.
    WillStartCodeBlock(Language),
    /// In a code block; use the content for highlighting.
    InCodeBlock(Language),
}

/// Define XML parsing events of interest for highlighting.
///
/// We don't care at all about any other items for this specific scenario; we
/// use these in conjunction with the `ParseState` to drive the state  machine
/// described there.
#[derive(Debug)]
enum ParseEvent {
    // Starting a `<pre>`; optionally specifying a language.
    StartPre(Option<Language>),
    // Starting a `<code>`.
    StartCode,
    // Ending a `<code>`.
    EndCode,
    Text,
    Other,
}

impl ParseState {
    /// Get the next `ParseState` given current `ParseState` and a `ParseEvent`.
    fn next(self, event: ParseEvent) -> ParseState {
        use self::ParseEvent::*;
        use self::ParseState::*;

        match (self, event) {
            (NotInBlock, StartPre(Some(language))) => MaybeStartBlock(language),
            (MaybeStartBlock(language), Text) => MaybeStartBlock(language),
            (MaybeStartBlock(language), StartCode) => WillStartCodeBlock(language),
            (WillStartCodeBlock(language), Text) => InCodeBlock(language),
            (InCodeBlock(_), EndCode) => NotInBlock,
            (_, _) => NotInBlock,
        }
    }
}

impl Default for ParseState {
    fn default() -> ParseState {
        ParseState::NotInBlock
    }
}

impl<'a> From<&Event<'a>> for ParseEvent {
    fn from(event: &Event) -> ParseEvent {
        const PRE: &[u8] = b"pre";
        const CODE: &[u8] = b"code";
        const CLASS: &[u8] = b"class";

        match *event {
            Event::Start(ref element) => match element.name() {
                PRE => {
                    let maybe_class_attr = element
                        .attributes()
                        .map(|attr| attr.unwrap())
                        .filter(
                            |&Attribute {
                                 key: attr,
                                 value: _,
                             }| attr == CLASS,
                        ).next();

                    if let Some(Attribute { key: _attr, value }) = maybe_class_attr {
                        match str::from_utf8(&value) {
                            Ok(lang) => ParseEvent::StartPre(Some(lang.into())),
                            Err(_) => ParseEvent::StartPre(None),
                        }
                    } else {
                        ParseEvent::StartPre(None)
                    }
                }
                CODE => ParseEvent::StartCode,
                _ => ParseEvent::Other,
            },
            Event::End(ref element) => match element.name() {
                CODE => ParseEvent::EndCode,
                _ => ParseEvent::Other,
            },
            Event::Text(ref element) => ParseEvent::Text,
            _ => ParseEvent::Other,
        }
    }
}

/// Highlight all code blocks in a block of HTML.
///
/// Assumes that the blocks to be highlighted are in the following basic format:
///
/// ```html
/// <pre class="rust">
///   <code>
///     pub fn syntax_highlight(html_string: String) -> String {
///         // implementation details
///     }
///   </code>
/// </pre>
/// ```
///
/// The `class` attribute value from the `pre` tag defines the language used to
/// highlight the code.
///
/// Note that any `html_string` will do; if it cannot be parsed as XML, it will
/// simply be returned unchanged; and if there are no code blocks to highlight,
/// it will also be returned unchanged.
pub fn syntax_highlight(html_string: String, theme: &Theme) -> String {
    let ss = SyntaxSet::load_defaults_nonewlines();
    let mut syntax_definitions = HashMap::<Language, &SyntaxReference>::new();

    let mut writer = Writer::new(Vec::<u8>::new());
    let mut state = ParseState::default();

    let mut reader = Reader::from_str(html_string.as_str());
    let mut buf = Vec::new();

    loop {
        let event = reader.read_event(&mut buf);
        let event = match event {
            Ok(Event::Eof) => break,
            Ok(event) => event,
            Err(_) => continue,
        };

        let parse_event = ParseEvent::from(&event);
        state = ParseState::next(state, parse_event);

        if let ParseState::InCodeBlock(ref language) = state {
            if let Event::Text(unescaped_content) = &event {
                if let Ok(content_to_highlight) =
                    str::from_utf8(&unescaped_content.unescaped().unwrap())
                {
                    if let Some(valid_syntax) = ss.find_syntax_by_token(&language) {
                        let syntax_definition = syntax_definitions
                            .entry(language.clone())
                            .or_insert(valid_syntax);

                        let highlighted = highlighted_html_for_string(
                            content_to_highlight,
                            &ss,
                            syntax_definition,
                            &theme,
                        );
                        let text = Event::Text(BytesText::from_plain_str(&highlighted));
                        assert!(writer.write_event(text).is_ok());
                        continue;
                    }
                }
            }
        }

        // Syntax highlighting did not succeed, so just write the original event.
        assert!(writer.write_event(event).is_ok());
    }

    String::from_utf8(writer.into_inner()).unwrap_or(html_string)
}

#[cfg(test)]
mod tests {

    #[test]
    fn parse_state() {
        use super::ParseEvent;
        use super::ParseState;

        let lang = "rust";

        assert_eq!(
            ParseState::NotInBlock.next(ParseEvent::StartPre(Some(lang.into()))),
            ParseState::MaybeStartBlock(lang.into())
        );

        assert_eq!(
            ParseState::NotInBlock.next(ParseEvent::EndCode),
            ParseState::NotInBlock
        );

        assert_eq!(
            ParseState::NotInBlock.next(ParseEvent::Other),
            ParseState::NotInBlock
        );

        assert_eq!(
            ParseState::NotInBlock.next(ParseEvent::StartCode),
            ParseState::NotInBlock
        );

        assert_eq!(
            ParseState::MaybeStartBlock(lang.into()).next(ParseEvent::StartCode),
            ParseState::WillStartCodeBlock(lang.into())
        );

        assert_eq!(
            ParseState::MaybeStartBlock(lang.into()).next(ParseEvent::Text),
            ParseState::NotInBlock
        );

        assert_eq!(
            ParseState::MaybeStartBlock(lang.into()).next(ParseEvent::EndCode),
            ParseState::NotInBlock
        );

        assert_eq!(
            ParseState::MaybeStartBlock(lang.into()).next(ParseEvent::StartPre(Some(lang.into()))),
            ParseState::NotInBlock
        );

        assert_eq!(
            ParseState::MaybeStartBlock(lang.into()).next(ParseEvent::Other),
            ParseState::NotInBlock
        );

        assert_eq!(
            ParseState::WillStartCodeBlock(lang.into()).next(ParseEvent::Text),
            ParseState::InCodeBlock(lang.into())
        );
    }

    #[test]
    fn highlight_code(){
        use syntect::highlighting::ThemeSet;
        use std::path::{Path, PathBuf};
        use crate::syntax_highlighting::syntax_highlight;

        let theme_file = PathBuf::from("data/base16-harmonic16.light.tmTheme");
        let theme = &ThemeSet::get_theme(theme_file).map_err(|err| format!("{:?}", err)).unwrap();

        let code = r#"
<pre class="rust">
 <code> 
    pub fn syntax_highlight(html_string: String) -> String {
        // implementation details
    }
  </code>
</pre>"#;

        let expected = "\n<pre class=\"rust\">\n <code>&lt;pre style=&quot;background-color:#f7f9fb;&quot;&gt;\n&lt;span style=&quot;color:#405c79;&quot;&gt; \n&lt;/span&gt;&lt;span style=&quot;color:#405c79;&quot;&gt;    &lt;/span&gt;&lt;span style=&quot;color:#bf568b;&quot;&gt;pub fn &lt;/span&gt;&lt;span style=&quot;color:#8b56bf;&quot;&gt;syntax_highlight&lt;/span&gt;&lt;span style=&quot;color:#405c79;&quot;&gt;(&lt;/span&gt;&lt;span style=&quot;color:#bf8b56;&quot;&gt;html_string&lt;/span&gt;&lt;span style=&quot;color:#405c79;&quot;&gt;: String) -&amp;gt; String {\n&lt;/span&gt;&lt;span style=&quot;color:#405c79;&quot;&gt;        &lt;/span&gt;&lt;span style=&quot;color:#aabcce;&quot;&gt;// implementation details\n&lt;/span&gt;&lt;span style=&quot;color:#405c79;&quot;&gt;    }\n&lt;/span&gt;&lt;span style=&quot;color:#405c79;&quot;&gt;  &lt;/span&gt;&lt;/pre&gt;\n</code>\n</pre>";
        let highlighted = syntax_highlight(code.to_string(), &theme);
        assert_eq!(highlighted, expected);
    }
}
