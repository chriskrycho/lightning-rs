use comrak::{
    format_html,
    nodes::{AstNode, NodeCodeBlock, NodeHtmlBlock, NodeValue},
    parse_document, Arena, ComrakExtensionOptions, ComrakOptions, ComrakParseOptions,
    ComrakRenderOptions,
};
use html_escape::encode_text_to_string;
use lazy_static::lazy_static;
use syntect::{
    html::{ClassStyle, ClassedHTMLGenerator},
    parsing::{SyntaxReference, SyntaxSet},
};

use super::{Preprocessed, Processed};

lazy_static! {
    static ref OPTIONS: ComrakOptions = ComrakOptions {
        extension: ComrakExtensionOptions {
            autolink: true,
            description_lists: true,
            footnotes: true,
            front_matter_delimiter: None,
            header_ids: Some(String::from("")),
            strikethrough: true,
            superscript: true,
            table: true,
            tagfilter: false,
            tasklist: true,
        },
        parse: ComrakParseOptions {
            default_info_string: None,
            smart: true,
        },
        render: ComrakRenderOptions {
            github_pre_lang: true,
            hardbreaks: false,
            width: 80,
            // I control the Markdown source for the site entirely. So I'm going to
            // render all the HTML in my source!
            unsafe_: true,
            // And I also don't want to escape it!
            escape: false,
        },
    };
}

pub fn render_markdown(src: Preprocessed, syntax_set: &SyntaxSet) -> Result<Processed, String> {
    let arena = Arena::new();

    let doc_root = parse_document(&arena, src.as_str(), &OPTIONS);

    iter_nodes(doc_root, &|node| {
        let result: Result<Option<String>, String> =
            if let &mut NodeValue::CodeBlock(NodeCodeBlock {
                fenced,
                ref mut info,
                ref mut literal,
                ..
            }) = &mut node.data.borrow_mut().value
            {
                let orig = std::mem::replace(literal, vec![]);
                let data = String::from_utf8(orig).map_err(|e| {
                    format!(
                        "error {}\n\nwhile trying to parse as utf8:\n{:?}",
                        e, &literal
                    )
                })?;

                let mut lines = data.split_inclusive(|c| c == '\n').peekable();

                let specified_syntax = if fenced {
                    let token = String::from_utf8(info.clone()).map_err(|e| {
                        format!("error {}\nwhile attempting to parse token:\n{:?}", e, info)
                    })?;
                    syntax_set.find_syntax_by_token(&token)
                } else {
                    None
                };

                let result = specified_syntax
                    .or_else(|| {
                        lines
                            .peek()
                            .and_then(|first_line| syntax_set.find_syntax_by_first_line(first_line))
                    })
                    .map(|syntax| {
                        let body = highlight(&syntax, &syntax_set, lines.into_iter());
                        format!(r#"<pre lang="{}"><code>{}</pre></code>"#, syntax.name, body)
                    })
                    .or_else(|| {
                        let mut encoded = String::with_capacity(data.len());
                        encode_text_to_string(data, &mut encoded);
                        let text = format!(r#"<pre><code>{}</pre></code>"#, encoded);
                        Some(text)
                    });

                Ok(result)
            } else {
                Ok(None)
            };

        match result {
            Ok(Some(highlighted)) => {
                let mut new_node = NodeHtmlBlock::default();
                new_node.literal = highlighted.as_bytes().to_vec();
                let mut ast = node.data.borrow_mut();
                ast.value = NodeValue::HtmlBlock(new_node);
                Ok(())
            }
            Ok(None) => Ok(()),
            Err(e) => Err(e),
        }
    })
    .map_err(|errs| errs.join("\n"))?;

    let mut output = Vec::with_capacity(src.len());
    format_html(doc_root, &OPTIONS, &mut output).map_err(|e| format!("{}", e))?;
    let output = String::from_utf8(output).map_err(|e| format!("{}", e))?;
    Ok(Processed(output))
}

fn highlight<'l, L>(syntax: &SyntaxReference, syntax_set: &SyntaxSet, lines: L) -> String
where
    L: Iterator<Item = &'l str>,
{
    let mut generator =
        ClassedHTMLGenerator::new_with_class_style(syntax, &syntax_set, ClassStyle::Spaced);
    for line in lines {
        generator.parse_html_for_line_which_includes_newline(&line);
    }
    generator.finalize()
}

fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F) -> Result<(), Vec<String>>
where
    F: Fn(&'a AstNode<'a>) -> Result<(), String>,
{
    let mut errors = vec![];
    if let Err(e) = f(node) {
        errors.push(e);
    }
    for c in node.children() {
        if let Err(ref mut e) = iter_nodes(c, f) {
            errors.append(e);
        }
    }

    if errors.len() == 0 {
        Ok(())
    } else {
        Err(errors)
    }
}
