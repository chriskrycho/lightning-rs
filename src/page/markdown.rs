use std::fmt::Display;

use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, Options, Parser, Tag};
use serde::private::de::StrDeserializer;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;

use super::Processed;

enum CodeHighlightingState<'a> {
    NotInCodeBlock,
    RequiresFirstLineParse,
    UnknownSyntax,
    KnownSyntax(ClassedHTMLGenerator<'a>),
}

enum FootnoteState<'e> {
    NotInFootnote,
    InFootnote {
        name: CowStr<'e>,
        children: Vec<Event<'e>>,
    },
}

pub(super) fn render_markdown<S: AsRef<str>>(
    src: S,
    syntax_set: &SyntaxSet,
) -> Result<Processed, String> {
    let src = src.as_ref();
    let parser = Parser::new_ext(src, Options::all());

    let mut highlight_state = CodeHighlightingState::NotInCodeBlock;
    let mut footnote_state = FootnoteState::NotInFootnote;
    let mut footnote_definitions = vec![];
    let mut events = Vec::<Event>::with_capacity(src.len() * 2);

    for event in parser {
        // If we are already in a footnote, push the events into the footnote
        // definition instead.
        let events_sink: &mut Vec<Event> = match footnote_state {
            FootnoteState::NotInFootnote => &mut events,
            FootnoteState::InFootnote {
                ref mut children, ..
            } => children,
        };

        match event {
            Event::Text(text) => match &mut highlight_state {
                // This is a little quirky: it hands off the text to the
                // highlighter and relies on correctly calling
                // `highlighter.finalize()` when we reach the end of the code
                // block.
                CodeHighlightingState::KnownSyntax(ref mut generator) => {
                    generator.parse_html_for_line_which_includes_newline(text.as_ref());
                    events_sink.push(Event::Text("".into()));
                }
                // This has the same constraint as `KnownSyntax`, but requires
                // that we also try to determine the syntax by parsing the first
                // line of the text.
                CodeHighlightingState::RequiresFirstLineParse => {
                    match syntax_set.find_syntax_by_first_line(&text) {
                        Some(definition) => {
                            let mut generator = ClassedHTMLGenerator::new_with_class_style(
                                definition,
                                &syntax_set,
                                ClassStyle::Spaced,
                            );
                            events_sink.push(Event::Html(
                                format!(
                                    "<pre lang='{name}'><code class='{name}'>",
                                    name = definition.name
                                )
                                .into(),
                            ));
                            generator.parse_html_for_line_which_includes_newline(&text);
                            highlight_state = CodeHighlightingState::KnownSyntax(generator);
                            events_sink.push(Event::Text("".into()));
                        }
                        None => {
                            events_sink.push(Event::Html("<pre><code>".to_string().into()));
                            highlight_state = CodeHighlightingState::UnknownSyntax;
                            events_sink.push(Event::Text(text));
                        }
                    }
                }
                CodeHighlightingState::UnknownSyntax | CodeHighlightingState::NotInCodeBlock => {
                    events_sink.push(Event::Text(text))
                }
            },
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(name))) => {
                if let Some(looked_up) = syntax_set.find_syntax_by_token(name.as_ref()) {
                    highlight_state = CodeHighlightingState::KnownSyntax(
                        ClassedHTMLGenerator::new_with_class_style(
                            looked_up,
                            &syntax_set,
                            ClassStyle::Spaced,
                        ),
                    );
                    let html = format!("<pre><code class='{}'>", looked_up.name);
                    events_sink.push(Event::Html(html.into()));
                } else {
                    highlight_state = CodeHighlightingState::UnknownSyntax;
                    events_sink.push(Event::Html("<pre><code>".into()));
                }
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Indented)) => match highlight_state {
                CodeHighlightingState::NotInCodeBlock => {
                    highlight_state = CodeHighlightingState::RequiresFirstLineParse;
                }
                _ => {
                    unreachable!("should never be entering a codeblock when already in a codeblock")
                }
            },
            Event::End(Tag::CodeBlock(_)) => match highlight_state {
                CodeHighlightingState::KnownSyntax(generator) => {
                    let highlighted = generator.finalize();
                    highlight_state = CodeHighlightingState::NotInCodeBlock;
                    events_sink.push(Event::Html((highlighted + "</code></pre>").into()));
                }
                CodeHighlightingState::UnknownSyntax
                | CodeHighlightingState::RequiresFirstLineParse => {
                    highlight_state = CodeHighlightingState::NotInCodeBlock;
                    events.push(Event::Html("</code></pre>".into()));
                }
                CodeHighlightingState::NotInCodeBlock => {
                    unreachable!("Cannot *not* be in a code block when ending a code block")
                }
            },
            Event::FootnoteReference(fn_def) => {
                let link = format!(
                    r##"<sup class="footnote-ref"><a href="#{}" id="{}"></a></sup>"##,
                    fn_ref_name(&fn_def),
                    fn_backref_name(&fn_def)
                );
                events.push(Event::Html(link.into()));
            }
            Event::Start(Tag::FootnoteDefinition(fn_def)) => match footnote_state {
                FootnoteState::NotInFootnote => {
                    footnote_state = FootnoteState::InFootnote {
                        name: fn_def,
                        children: vec![],
                    }
                }
                FootnoteState::InFootnote { name, .. } => {
                    panic!("Already in footnote definition for {} and entered a new footnote definition for {}", name, fn_def);
                }
            },
            Event::End(Tag::FootnoteDefinition(fn_def)) => match footnote_state {
                FootnoteState::InFootnote {
                    ref name,
                    ref mut children,
                } => {
                    let start = format!(r#"<li id="{}" class="footnote-item">"#, fn_ref_name(name));
                    let end = format!(
                        r#"<a href="{}" class="footnote-backref">↩︎</a></li>"#,
                        fn_backref_name(name),
                    );
                    footnote_definitions.push(Event::Html(start.into()));
                    footnote_definitions.append(children);
                    footnote_definitions.push(Event::Html(end.into()));
                    footnote_state = FootnoteState::NotInFootnote;
                }
                FootnoteState::NotInFootnote => {
                    unreachable!("Cannot *not* be in a footnote when ending a footnote, but ending footnote named {}", fn_def);
                }
            },
            _ => events_sink.push(event),
        }
    }

    if !footnote_definitions.is_empty() {
        let start =
            r#"<hr class="footnotes-sep" /><section class="footnotes"><ul class="footnotes-list">"#;
        let end = "</ul></section>";
        events.push(Event::Html(start.into()));
        events.append(&mut footnote_definitions);
        events.push(Event::Html(end.into()));
    }

    let mut html_output = String::with_capacity(src.len() * 2);

    html::push_html(&mut html_output, events.into_iter());

    Ok(Processed(html_output))
}

fn fn_ref_name<D: Display>(def: D) -> String {
    format!("fn-{}", def)
}

fn fn_backref_name<D: Display>(def: D) -> String {
    format!("fnref-{}", def)
}
