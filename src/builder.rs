//! Generate the site content.

// Standard library
use std::borrow::Cow::{Borrowed, Owned};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

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

// TODO: this is an obvious candidate for a `HashMap`, so use that instead.
type LoadTuple<'a> = (&'a PathBuf, Result<String, String>);

// Instead of using `.collect` at the end of these, I *should* be able to use
// something like `Result<impl Iterator<HashMap<PathBuf, String>, String>`, even
// if the type isn't *quite* taht.
fn load_content<'p>(paths: &'p [PathBuf]) -> Result<Vec<(&'p PathBuf, String)>, String> {
    let (contents, errs): (Vec<LoadTuple<'_>>, Vec<LoadTuple<'_>>) = paths
        .into_iter()
        .map(|path| {
            let content = std::fs::read_to_string(path).map_err(|e| format!("{:?}", e));
            (path, content)
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
            .map(|(_path, err)| err.unwrap_err())
            .collect::<Vec<String>>()
            .join(",\n"))
    }
}

/// Load the templates associated with each taxonomy.
fn load_templates(_site_directory: &PathBuf, _config: &Config) -> Result<Paths, String> {
    unimplemented!()
}

// TODO: put this somewhere else when extracting all the parser logic below into
// its own module.
#[derive(Debug)]
enum ParseState<'s> {
    CodeBlock(&'s syntect::parsing::SyntaxReference),
    PlainTextBlock,
    NonCode,
}

impl<'s> std::default::Default for ParseState<'s> {
    fn default() -> ParseState<'s> {
        ParseState::NonCode
    }
}

enum Syntax {
    Highlight(syntect::highlighting::Theme),
    TagOnly,
    Off,
}

impl Syntax {
    fn try_from(config_syntax: &SyntaxOption) -> Result<Syntax, String> {
        match config_syntax {
            SyntaxOption::Highlight(theme) => {
                let theme_file = PathBuf::from("data").join(theme);
                let theme = ThemeSet::get_theme(theme_file).map_err(|err| format!("{:?}", err))?;
                Ok(Syntax::Highlight(theme))
            }
            SyntaxOption::TagOnly => Ok(Syntax::TagOnly),
            SyntaxOption::Off => Ok(Syntax::Off),
        }
    }
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

    let configured_syntax = Syntax::try_from(&config.options.syntax)?;

    // TODO: generate these! Best move: on build, generate a `.packdump`, since
    // that seems to be what Syntect knows how to deal with.
    let syntax_dir = PathBuf::from("data/syntaxes");
    let syntax_set =
        syntect::parsing::SyntaxSet::load_from_folder(syntax_dir).expect("can't load syntaxes");

    let parsed_content: Vec<(&PathBuf, String)> = contents
        .into_iter()
        .map(|(path, content)| {
            let parser = Parser::new_ext(&content, pulldown_cmark::Options::all()).scan(
                ParseState::default(),
                |state, event| match event {
                    Event::Start(Tag::CodeBlock(ref language)) => match state {
                        ParseState::NonCode => {
                            *state = match syntax_set.find_syntax_by_name(&language) {
                                Some(syntax) => ParseState::CodeBlock(syntax),
                                None => ParseState::PlainTextBlock,
                            };

                            // We start every code block with `<pre><code class="...">`
                            // so that we always have semantically correct HTML. However, we
                            // don't add the language class if the language isn't set.
                            let content = if language.len() > 0 {
                                Owned(format!(r#"<pre><code class="{}">"#, language))
                            } else {
                                Borrowed("<pre><code>")
                            };

                            Some(Event::Html(content))
                        }
                        ParseState::CodeBlock(_) | ParseState::PlainTextBlock => {
                            unreachable!("Bad event/state: {:?} with {:?}", event, state);
                        }
                    },

                    Event::End(Tag::CodeBlock(_)) => match state {
                        ParseState::CodeBlock(_) | ParseState::PlainTextBlock => {
                            *state = ParseState::NonCode;
                            Some(Event::Html(Borrowed("</code></pre>")))
                        }
                        ParseState::NonCode => {
                            unreachable!("Bad event/state: {:?} with {:?}", event, state);
                        }
                    },

                    // When we are in a text block, we *may* be in a code block,
                    // so we may also need to do whatever highlighting is
                    // specified.
                    Event::Text(s) => {
                        match state {
                            ParseState::CodeBlock(syntax) => match &configured_syntax {
                                Syntax::Highlight(theme) => {
                                    let highlighted = syntect::html::highlighted_html_for_string(
                                        &s,
                                        &syntax_set,
                                        &syntax,
                                        theme,
                                    );

                                    Some(Event::Text(Owned(highlighted)))
                                }
                                Syntax::TagOnly => {
                                    // TODO: implement tag-only parsing with syntect
                                    unimplemented!()
                                }
                                Syntax::Off => Some(Event::Text(s)),
                            },
                            ParseState::PlainTextBlock => Some(Event::Text(s)),
                            ParseState::NonCode => Some(Event::Text(s)),
                        }
                    }

                    // TODO: accumulate footnote references and always put them
                    // at the end. Also, generate back-links for them, unless or
                    // until pulldown-cmark does so.
                    Event::FootnoteReference(s) => Some(Event::FootnoteReference(s)),

                    Event::Start(s) => Some(Event::Start(s)),
                    Event::End(e) => Some(Event::End(e)),
                    Event::InlineHtml(s) => Some(Event::InlineHtml(s)),
                    Event::Html(h) => Some(Event::Html(h)),
                    Event::SoftBreak => Some(Event::SoftBreak),
                    Event::HardBreak => Some(Event::HardBreak),
                },
            );

            // These numbers are derived from a brief survey of content in my
            // own website historically, including both code-heavy and
            // prose-heavy posts. It's probably worth exploring other corpora
            // to see if this is basically right.
            let length_bound_estimate = match &config.options.syntax {
                SyntaxOption::Off | SyntaxOption::TagOnly => 2,
                SyntaxOption::Highlight(_) => 8, // TODO: check other pages
            };

            let mut html = String::with_capacity(content.len() * length_bound_estimate);
            cmark::html::push_html(&mut html, parser);

            (path, html)
        })
        .collect();

    // TODO: extract this as part of the writing it out process.
    let output_dir = site_directory.join(&config.directories.output);

    if output_dir.exists() && !output_dir.is_dir() {
        return Err(format!(
            "A file with the same name as the output directory {:?} already exists!",
            output_dir
        ));
    }

    if !output_dir.exists() {
        std::fs::create_dir(&output_dir).expect(&format!(
            "could not create output directory {:?}",
            &output_dir
        ));
    }

    for (path, content) in parsed_content {
        let file_path = path
            .file_name()
            .ok_or(format!("invalid file: {:?}", path))?;

        let dest = output_dir.join(file_path).with_extension("html");

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
