//! Generate the site content.

// Standard library
use std::borrow::Cow::Owned;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

// Third party
use glob::{glob, Paths};
use pulldown_cmark as cmark;
use pulldown_cmark::{Event, Parser, Tag};
use syntect::highlighting::ThemeSet;

// First party
use crate::config::{Config, SyntaxOption};

/// Load the `Paths` for all markdown files in the specified content directory.
///
/// If the specified configuration values are wrong, we return a vector of all
/// the errors in the config as far as this is concerned.
///
/// TODO: move that validation elsewhere? Should probably extract a more general
/// site configuration validator.
fn load_markdown_paths(site_directory: &PathBuf, config: &Config) -> Result<Vec<PathBuf>, String> {
    let site = site_directory
        .to_str()
        .ok_or(format!("bad `site`: {:?}", site_directory));

    let directories = config.directories.content.to_str().ok_or(format!(
        "bad content directory: {:?}",
        config.directories.content
    ));

    let (site, directories) = match (site, directories) {
        (Err(s), Err(d)) => Err(s + "\n" + &d),
        (Err(s), Ok(_)) => Err(s),
        (Ok(_), Err(d)) => Err(d),
        (Ok(s), Ok(d)) => Ok((s, d)),
    }?;

    let content_glob = format!("{}/{}/**/*.md", site, directories);

    let paths = glob(&content_glob).map_err(|err| format!("{:?}", err))?;

    let (valid, errors): (Vec<Result<PathBuf, _>>, Vec<Result<_, String>>) = paths
        .map(|glob_result| glob_result.map_err(|err| format!("{:?}", err)))
        .partition(|path| path.is_ok());

    let errors: Vec<String> = errors.into_iter().map(|e| e.unwrap_err()).collect();
    if errors.len() == 0 {
        Ok(valid.into_iter().map(|v| v.unwrap()).collect())
    } else {
        Err(errors.join(",\n"))
    }
}

type LoadTuple<'a> = (&'a PathBuf, Result<String, String>);

fn load_content<'p>(paths: &'p [PathBuf]) -> Result<Vec<(&'p PathBuf, String)>, String> {
    let (contents, errs): (Vec<LoadTuple>, Vec<LoadTuple>) = paths
        .into_iter()
        .map(|path| {
            (
                path,
                std::fs::read_to_string(path).map_err(|e| format!("{:?}", e)),
            )
        })
        .partition(|(_, result)| result.is_ok());

    if errs.len() == 0 {
        Ok(contents
            .into_iter()
            .map(|(path, contents)| (path, contents.unwrap()))
            .collect())
    } else {
        Err(errs
            .into_iter()
            .map(|(path, err)| err.unwrap_err())
            .collect::<Vec<String>>()
            .join(",\n"))
    }
}

/// Load the templates associated with each taxonomy.
fn load_templates(_site_directory: &PathBuf, _config: &Config) -> Result<Paths, String> {
    unimplemented!()
}

/// Generate content from a configuration.
pub fn build(site_directory: PathBuf) -> Result<(), String> {
    // NOTE: this is almost certainly not what we *ultimately* want here, but
    // it'll do as a starting point. There's a lot of `into_iter` and `collect`
    // happening here, which forces us to eagerly materialize a lot of these
    // collections rather than being able to deal with them in a more
    // "streaming" fashion.
    let config = Config::from_file(&PathBuf::from(&site_directory))?;
    let markdown_paths = load_markdown_paths(&site_directory, &config)?;
    let contents = load_content(&markdown_paths)?;

    // TODO: build from config, if and only if specified and highlighting is
    // enabled. Also, extract and just do this once *not* at the top level
    // function.
    let theme_file = PathBuf::from("data/base16-ocean.dark.tmTheme");
    let theme = &ThemeSet::get_theme(theme_file).map_err(|err| format!("{:?}", err))?;

    let parsed_content: Vec<(&PathBuf, String)> = contents
        .into_iter()
        .map(|(path, content)| {
            enum State {
                InCodeBlock,
                Other,
            }

            impl State {
                fn new() -> State {
                    State::Other
                }
            }

            let mut state = State::new();

            let parser = Parser::new_ext(&content, pulldown_cmark::Options::all()).map(|event| {
                match (event, &state) {
                    // We start every code block with `<pre><code class="...">`
                    // so that we always have semantically correct HTML.
                    (Event::Start(Tag::CodeBlock(language)), State::Other) => {
                        state = State::InCodeBlock;
                        Event::Html(Owned(format!(r#"<pre><code class="{}">"#, language)))
                    }

                    // The closing tag must match the opening tag.
                    (Event::End(Tag::CodeBlock(_)), State::InCodeBlock) => {
                        state = State::Other;
                        Event::Html(Owned("</code></pre>".into()))
                    }

                    (Event::Text(s), State::InCodeBlock) => {
                        match &config.options.syntax {
                            SyntaxOption::Off => Event::Text(s),
                            SyntaxOption::TagOnly => {
                                // TODO: implement tag-only parsing with syntect
                                // let parsed_by_syntect = syntect::parsing::ParseState::new();
                                // Event::Html(Owned(syntect::html::tokens_to_classed_html(s, parsed_by_syntect, syntect::html::ClassStyle::Spaced)))
                                Event::Text(s)
                            }
                            SyntaxOption::Highlight(theme_name) => {
                                // TODO: full syntax highlighting on text
                                Event::Text(s)
                            }
                        }
                    }

                    // If we hit this, we *know* that we can't be here without
                    // there being a really bad bug in our code or
                    // pulldown_cmark (i.e. in our code!).
                    (Event::End(Tag::CodeBlock(_)), State::Other) | (_, State::InCodeBlock) => {
                        unreachable!(format!("Error parsing code blocks in {:?}", path));
                    }

                    (Event::Text(s), State::Other) => Event::Text(s),
                    (Event::Start(s), _) => Event::Start(s),
                    (Event::End(e), _) => Event::End(e),
                    (Event::InlineHtml(s), _) => Event::InlineHtml(s),
                    (Event::Html(h), _) => Event::Html(h),
                    (Event::SoftBreak, _) => Event::SoftBreak,
                    (Event::HardBreak, _) => Event::HardBreak,
                    (Event::FootnoteReference(s), _) => Event::FootnoteReference(s),
                }
            });

            // These numbers are derived from a brief survey of content in my
            // own website historically, including both code-heavy and
            // prose-heavy posts. It's probably worth exploring other corpora
            // to see if this is basically right.
            let length_bound_estimate = match config.options.syntax {
                SyntaxOption::Off | SyntaxOption::TagOnly => 2,
                SyntaxOption::Highlight(_) => 8, // TODO: check other pages
            };

            let mut html = String::with_capacity(content.len() * length_bound_estimate);
            cmark::html::push_html(&mut html, parser);

            (path, html)
        })
        .collect();

    for (path, content) in parsed_content {
        // TODO: extract this as part of the writing it out process.
        // TODO: set output location in config.
        let dest = Path::new("./tests/output")
            .join(
                path.file_name()
                    .ok_or(format!("invalid file: {:?}", path))?,
            )
            .with_extension("html");

        let mut fd = match File::create(&dest) {
            Ok(file) => file,
            Err(reason) => {
                return Err(format!(
                    "Could not open {} for write: {}",
                    dest.to_string_lossy(),
                    reason
                ));
            }
        };

        let result = write!(fd, "{}", content);
        if let Err(reason) = result {
            return Err(format!("{:?}", reason.kind()));
        }
    }

    Ok(())
}
