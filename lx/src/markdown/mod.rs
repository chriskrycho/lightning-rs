use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;

enum CodeHighlightingState<'a> {
    NotInCodeBlock,
    RequiresFirstLineParse,
    UnknownSyntax,
    KnownSyntax(ClassedHTMLGenerator<'a>),
}

/// The result of rendering the content with Markdown.
pub struct Rendered(String);

impl From<Rendered> for String {
    fn from(value: Rendered) -> Self {
        value.0
    }
}

pub(super) fn render<S: AsRef<str>>(src: S, syntax_set: &SyntaxSet) -> Result<Rendered, String> {
    let src = src.as_ref();
    let mut options = Options::all();
    options.set(Options::ENABLE_OLD_FOOTNOTES, false);
    options.set(Options::ENABLE_FOOTNOTES, true);
    let parser = Parser::new_ext(src, options);
    let mut state = CodeHighlightingState::NotInCodeBlock;

    let mut events = Vec::<Event>::with_capacity(src.len() * 2);
    for event in parser {
        match event {
            Event::Text(text) => match &mut state {
                // This is a little quirky: it hands off the text to the highlighter
                // and relies on correctly calling `highlighter.finalize()` when we
                // reach the end of the code block.
                CodeHighlightingState::KnownSyntax(ref mut generator) => {
                    generator.parse_html_for_line_which_includes_newline(text.as_ref());
                    events.push(Event::Text("".into()));
                }
                // This has the same constraint as `KnownSyntax`, but requires that
                // we also try to get a
                CodeHighlightingState::RequiresFirstLineParse => {
                    match syntax_set.find_syntax_by_first_line(&text) {
                        Some(definition) => {
                            let mut generator = ClassedHTMLGenerator::new_with_class_style(
                                definition,
                                syntax_set,
                                ClassStyle::Spaced,
                            );
                            events.push(Event::Html(
                                format!(
                                    "<pre lang='{name}'><code class='{name}'>",
                                    name = definition.name
                                )
                                .into(),
                            ));
                            generator.parse_html_for_line_which_includes_newline(&text);
                            state = CodeHighlightingState::KnownSyntax(generator);
                            events.push(Event::Text("".into()));
                        }
                        None => {
                            events.push(Event::Html("<pre><code>".to_string().into()));
                            state = CodeHighlightingState::UnknownSyntax;
                            events.push(Event::Text(text));
                        }
                    }
                }
                CodeHighlightingState::UnknownSyntax | CodeHighlightingState::NotInCodeBlock => {
                    events.push(Event::Text(text))
                }
            },
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(name))) => {
                if let Some(looked_up) = syntax_set.find_syntax_by_token(name.as_ref()) {
                    state = CodeHighlightingState::KnownSyntax(
                        ClassedHTMLGenerator::new_with_class_style(
                            looked_up,
                            syntax_set,
                            ClassStyle::Spaced,
                        ),
                    );
                    let html = format!("<pre><code class='{}'>", looked_up.name);
                    events.push(Event::Html(html.into()));
                } else {
                    state = CodeHighlightingState::UnknownSyntax;
                    events.push(Event::Html("<pre><code>".into()));
                }
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Indented)) => match state {
                CodeHighlightingState::NotInCodeBlock => {
                    state = CodeHighlightingState::RequiresFirstLineParse;
                }
                _ => {
                    unreachable!("should never be entering a codeblock when already in a codeblock")
                }
            },
            Event::End(TagEnd::CodeBlock) => match state {
                CodeHighlightingState::KnownSyntax(generator) => {
                    let highlighted = generator.finalize();
                    state = CodeHighlightingState::NotInCodeBlock;
                    events.push(Event::Html((highlighted + "</code></pre>").into()));
                }
                CodeHighlightingState::UnknownSyntax
                | CodeHighlightingState::RequiresFirstLineParse => {
                    state = CodeHighlightingState::NotInCodeBlock;
                    events.push(Event::Html("</code></pre>".into()));
                }
                CodeHighlightingState::NotInCodeBlock => {
                    unreachable!("Cannot *not* be in a code block when ending a code block")
                }
            },
            _ => events.push(event),
        }
    }

    let mut html_output = String::with_capacity(src.len() * 2);

    html::push_html(&mut html_output, events.into_iter());

    Ok(Rendered(html_output))
}
