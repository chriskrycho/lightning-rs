//! Provide syntax highlighting via [Syntect].
//!
//! [Syntect]: https://docs.rs/syntect/1.0.0/syntect/

// Standard library
use std::default::Default;
use std::collections::HashMap;
use std::str;

// Third party
use quick_xml::{XmlReader, XmlWriter, Element, Event};
use syntect::html::highlighted_snippet_for_string;
use syntect::highlighting::Theme;
use syntect::parsing::{SyntaxDefinition, SyntaxSet};


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
    Whitespace,
    Other,
}


impl ParseState {
    /// Get the next `ParseState` given current `ParseState` and a `ParseEvent`.
    fn next(self, event: ParseEvent) -> ParseState {
        use self::ParseState::*;
        use self::ParseEvent::*;

        match (self, event) {
            (NotInBlock, StartPre(Some(language))) => MaybeStartBlock(language),
            (MaybeStartBlock(language), Whitespace) => MaybeStartBlock(language),
            (MaybeStartBlock(language), StartCode) => WillStartCodeBlock(language),
            (WillStartCodeBlock(language), Text) |
            (WillStartCodeBlock(language), Whitespace) => InCodeBlock(language),
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


impl<'e> From<&'e Event> for ParseEvent {
    fn from(event: &'e Event) -> ParseEvent {
        // TODO @1.15: remove `'static`.
        const PRE: &'static [u8] = b"pre";
        const CODE: &'static [u8] = b"code";
        const CLASS: &'static [u8] = b"class";
        const WHITE_SPACE: &'static [u8] = b"";

        match *event {
            Event::Start(ref element) => {
                match element.name() {
                    PRE => {
                        let maybe_class_attr = element.attributes()
                            .map(|attr| attr.unwrap())
                            .filter(|&(attr, _value)| attr == CLASS)
                            .next();

                        if let Some((_attr, value)) = maybe_class_attr {
                            match str::from_utf8(value) {
                                Ok(lang) => ParseEvent::StartPre(Some(lang.to_string())),
                                Err(_) => ParseEvent::StartPre(None),
                            }
                        } else {
                            ParseEvent::StartPre(None)
                        }
                    }
                    CODE => ParseEvent::StartCode,
                    _ => ParseEvent::Other,
                }
            }
            Event::End(ref element) => {
                match element.name() {
                    CODE => ParseEvent::EndCode,
                    _ => ParseEvent::Other,
                }
            }
            Event::Text(ref element) => {
                match element.name() {
                    WHITE_SPACE => ParseEvent::Whitespace,
                    _ => ParseEvent::Text,
                }
            }
            _ => ParseEvent::Other,
        }
    }
}


struct Accumulator {
    writer: XmlWriter<Vec<u8>>,
    state: ParseState,
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
    let original_string = html_string.clone();
    let reader = XmlReader::from(html_string.as_str());

    let ss = SyntaxSet::load_defaults_nonewlines();
    let mut syntax_definitions = HashMap::<Language, SyntaxDefinition>::new();

    let accumulator = Accumulator {
        writer: XmlWriter::new(Vec::<u8>::new()),
        state: ParseState::default(),
    };

    let final_state = reader.fold(accumulator, |mut acc, event| {
        let event = match event {
            Ok(event) => event,
            Err(_) => {
                return acc;
            }
        };

        let parse_event = ParseEvent::from(&event);
        acc.state = ParseState::next(acc.state, parse_event);

        let language = match acc.state.clone() {
            ParseState::InCodeBlock(language) => language,
            _ => {
                assert!(acc.writer.write(event.clone()).is_ok());
                return acc;
            }
        };

        let unescaped_content = match event.element().unescaped_content() {
            Ok(content) => content.into_owned(),
            Err(_) => {
                assert!(acc.writer.write(event.clone()).is_ok());
                return acc;
            }
        };

        let content_to_highlight = match str::from_utf8(&unescaped_content) {
            Ok(utf8_str) => utf8_str,
            Err(_) => {
                assert!(acc.writer.write(event.clone()).is_ok());
                return acc;
            }
        };

        let syntax_key = language.clone();
        let syntax_definition = syntax_definitions.entry(syntax_key).or_insert({
            match ss.find_syntax_by_token(&language) {
                Some(valid_syntax) => valid_syntax.clone(),
                None => {
                    assert!(acc.writer.write(event.clone()).is_ok());
                    return acc;
                }
            }
        });

        let highlighted =
            highlighted_snippet_for_string(content_to_highlight, syntax_definition, &theme);

        let text = Element::new(highlighted);
        assert!(acc.writer.write(Event::Text(text)).is_ok());

        acc
    });

    String::from_utf8(final_state.writer.into_inner()).unwrap_or(original_string)
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_state() {
        use super::ParseState;
        use super::ParseEvent;

        let lang = "rust";

        assert_eq!(ParseState::NotInBlock.next(ParseEvent::StartPre(Some(lang.to_string()))),
                   ParseState::MaybeStartBlock(lang.to_string()));

        assert_eq!(ParseState::NotInBlock.next(ParseEvent::EndCode),
                   ParseState::NotInBlock);

        assert_eq!(ParseState::NotInBlock.next(ParseEvent::Other),
                   ParseState::NotInBlock);

        assert_eq!(ParseState::NotInBlock.next(ParseEvent::StartCode),
                   ParseState::NotInBlock);

        assert_eq!(ParseState::MaybeStartBlock(lang.to_string()).next(ParseEvent::StartCode),
                   ParseState::WillStartCodeBlock(lang.to_string()));

        assert_eq!(ParseState::MaybeStartBlock(lang.to_string()).next(ParseEvent::Text),
                   ParseState::NotInBlock);

        assert_eq!(ParseState::MaybeStartBlock(lang.to_string()).next(ParseEvent::EndCode),
                   ParseState::NotInBlock);

        assert_eq!(ParseState::MaybeStartBlock(lang.to_string())
                       .next(ParseEvent::StartPre(Some(lang.to_string()))),
                   ParseState::NotInBlock);

        assert_eq!(ParseState::MaybeStartBlock(lang.to_string()).next(ParseEvent::Other),
                   ParseState::NotInBlock);

        assert_eq!(ParseState::WillStartCodeBlock(lang.to_string()).next(ParseEvent::Text),
                   ParseState::InCodeBlock(lang.to_string()));
    }
}
