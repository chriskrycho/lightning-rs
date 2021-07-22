use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag};
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;

use super::Preprocessed;

enum ParseState<'a> {
    NotInCodeBlock,
    RequiresFirstLineParse,
    UnknownSyntax,
    KnownSyntax(ClassedHTMLGenerator<'a>),
}

/// The result of rendering the content.
pub struct Processed(pub(super) String);

pub(super) fn render_markdown(
    src: Preprocessed,
    syntax_set: &SyntaxSet,
) -> Result<Processed, String> {
    let src = src.as_str();
    let parser = Parser::new_ext(src, Options::all());
    let mut state = ParseState::NotInCodeBlock;

    let mut events = Vec::<Event>::with_capacity(src.len() * 2);
    for event in parser {
        match event {
            Event::Text(text) => match &mut state {
                // This is a little quirky: it hands off the text to the highlighter
                // and relies on correctly calling `highlighter.finalize()` when we
                // reach the end of the code block.
                ParseState::KnownSyntax(ref mut generator) => {
                    generator.parse_html_for_line_which_includes_newline(text.as_ref());
                    events.push(Event::Text("".into()));
                }
                // This has the same constraint as `KnownSyntax`, but requires that
                // we also try to get a
                ParseState::RequiresFirstLineParse => {
                    match syntax_set.find_syntax_by_first_line(&text) {
                        Some(definition) => {
                            let mut generator = ClassedHTMLGenerator::new_with_class_style(
                                definition,
                                &syntax_set,
                                ClassStyle::Spaced,
                            );
                            events.push(Event::Html(
                                format!("<pre><code class='{}'>", definition.name).into(),
                            ));
                            generator.parse_html_for_line_which_includes_newline(&text);
                            state = ParseState::KnownSyntax(generator);
                            events.push(Event::Text("".into()));
                        }
                        None => {
                            state = ParseState::UnknownSyntax;
                            events.push(Event::Text(text));
                        }
                    }
                }
                ParseState::UnknownSyntax | ParseState::NotInCodeBlock => {
                    events.push(Event::Text(text))
                }
            },
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(name))) => {
                if let Some(looked_up) = syntax_set.find_syntax_by_token(name.as_ref()) {
                    state = ParseState::KnownSyntax(ClassedHTMLGenerator::new_with_class_style(
                        looked_up,
                        &syntax_set,
                        ClassStyle::Spaced,
                    ));
                    let html = format!("<pre><code class='{}'>", looked_up.name);
                    events.push(Event::Html(html.into()));
                } else {
                    state = ParseState::UnknownSyntax;
                    events.push(Event::Html("<pre><code>".into()));
                }
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Indented)) => match state {
                ParseState::NotInCodeBlock => {
                    state = ParseState::RequiresFirstLineParse;
                }
                _ => {
                    unreachable!("should never be entering a codeblock when already in a codeblock")
                }
            },
            Event::End(Tag::CodeBlock(_)) => match state {
                ParseState::KnownSyntax(generator) => {
                    let highlighted = generator.finalize();
                    state = ParseState::NotInCodeBlock;
                    events.push(Event::Html((highlighted + "</code></pre>").into()));
                }
                ParseState::UnknownSyntax | ParseState::RequiresFirstLineParse => {
                    state = ParseState::NotInCodeBlock;
                    events.push(Event::Html("</code></pre>".into()));
                }
                ParseState::NotInCodeBlock => {
                    unreachable!("Cannot *not* be in a code block when ending a coceblock")
                }
            },
            _ => events.push(event),
        }
    }

    let mut html_output = String::with_capacity(src.len() * 2);

    html::push_html(&mut html_output, events.into_iter());

    Ok(Processed(html_output))
}
