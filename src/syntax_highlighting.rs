/// Provide syntax highlighting via [Syntect].
///
/// [Syntect]: https://docs.rs/syntect/1.0.0/syntect/

// Standard library
use std::default::Default;
use std::str;

// Third party
use quick_xml::{XmlReader, XmlWriter, Element, Event};
use syntect::html::highlighted_snippet_for_string;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;


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
    fn next(&self, event: &ParseEvent) -> ParseState {
        use self::ParseState::*;
        use self::ParseEvent::*;

        match (self, event) {
            (&NotInBlock, &StartPre(Some(ref language))) => MaybeStartBlock(language.clone()),
            (&MaybeStartBlock(ref language), &Whitespace) => MaybeStartBlock(language.clone()),
            (&MaybeStartBlock(ref language), &StartCode) => WillStartCodeBlock(language.clone()),
            (&WillStartCodeBlock(ref language), &Text) |
            (&WillStartCodeBlock(ref language), &Whitespace) => InCodeBlock(language.clone()),
            (&InCodeBlock(_), &EndCode) => NotInBlock,
            (_, _) => NotInBlock,
        }
    }
}


impl Default for ParseState {
    fn default() -> ParseState {
        ParseState::NotInBlock
    }
}


impl<'a> From<&'a Event> for ParseEvent {
    fn from(event: &'a Event) -> ParseEvent {
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
                            .filter(|attr| attr.0 == CLASS)
                            .next();

                        match maybe_class_attr {
                            Some(class_attr) => {
                                match str::from_utf8(class_attr.1) {
                                    Ok(lang) => ParseEvent::StartPre(Some(String::from(lang))),
                                    Err(_) => ParseEvent::StartPre(None),
                                }
                            }
                            None => ParseEvent::StartPre(None),
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
pub fn syntax_highlight(html_string: String) -> String {
    let original_string = html_string.clone();
    let reader = XmlReader::from(html_string.as_str());

    let mut accumulator = Accumulator {
        writer: XmlWriter::new(Vec::<u8>::new()),
        state: ParseState::default(),
    };

    reader.fold(&mut accumulator, |acc, event| {
        let event = match event {
            Ok(event) => event,
            Err(_) => {
                return acc;
            }
        };

        let parse_event = ParseEvent::from(&event);
        acc.state = acc.state.next(&parse_event);

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
            },
        };

        let ss = SyntaxSet::load_defaults_nonewlines();
        let syntax = ss.find_syntax_by_token(&language);
        let valid_syntax = match syntax {
            Some(valid_syntax) => valid_syntax,
            None => {
                assert!(acc.writer.write(event.clone()).is_ok());
                return acc;
            }
        };

        let highlighted = highlighted_snippet_for_string(
            content_to_highlight,
            valid_syntax,
            &ThemeSet::load_defaults().themes["base16-eighties.dark"]
        );

        let text = Element::new(highlighted);
        assert!(acc.writer.write(Event::Text(text)).is_ok());

        acc
    });

    String::from_utf8(accumulator.writer.into_inner()).unwrap_or(original_string)
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_state() {
        use super::ParseState;
        use super::ParseEvent;

        let lang = String::from("rust");

        assert_eq!(ParseState::NotInBlock.next(&ParseEvent::StartPre(Some(lang.clone()))),
                   ParseState::MaybeStartBlock(lang.clone()));

        assert_eq!(ParseState::NotInBlock.next(&ParseEvent::EndCode),
                   ParseState::NotInBlock);

        assert_eq!(ParseState::NotInBlock.next(&ParseEvent::Other),
                   ParseState::NotInBlock);

        assert_eq!(ParseState::NotInBlock.next(&ParseEvent::StartCode),
                   ParseState::NotInBlock);

        assert_eq!(ParseState::MaybeStartBlock(lang.clone()).next(&ParseEvent::StartCode),
                   ParseState::WillStartCodeBlock(lang.clone()));

        assert_eq!(ParseState::MaybeStartBlock(lang.clone()).next(&ParseEvent::Text),
                   ParseState::NotInBlock);

        assert_eq!(ParseState::MaybeStartBlock(lang.clone()).next(&ParseEvent::EndCode),
                   ParseState::NotInBlock);

        assert_eq!(ParseState::MaybeStartBlock(lang.clone())
                       .next(&ParseEvent::StartPre(Some(lang.clone()))),
                   ParseState::NotInBlock);

        assert_eq!(ParseState::MaybeStartBlock(lang.clone()).next(&ParseEvent::Other),
                   ParseState::NotInBlock);

        assert_eq!(ParseState::WillStartCodeBlock(lang.clone()).next(&ParseEvent::Text),
                   ParseState::InCodeBlock(lang.clone()));
    }
}
