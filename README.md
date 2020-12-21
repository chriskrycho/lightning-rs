# Lightning (`lx`)

An opinionated—I dare say idiosyncratic—take on the static site generator, written in Rust,[<sup>[1]</sup>](#notes) and built for the very specific needs of my own website publishing. **I do not intend to make this a general-purpose tool.** It is public because I default to making things public, but this project is distinctly *personal*, and in it I will experiment freely and build *exactly* what I want over time.

Accordingly, I'm not really looking for collaborators, and I will not be taking feature requests. (If it works for you, that’s great, and if you happen to spot and fix a bug, I won't argue, though!)

[Cobalt]: https://cobalt-org.github.io

## Goals

This project's main goals are:

- speed
- ease of use, even for more complex ways of structuring a site
- good out-of-the-box defaults (‘zero-config’, ‘convention over configuration’, etc.), but with human-readable and -writable configurability

It is an explicit non-goal to be an exact drop-in replacement for any other generator, or indeed to be useful for anyone but me!

## Roadmap

- [ ] Render Markdown

    - [ ] with [pulldown-cmark]
    - [ ] with [Syntect] for syntax highlighting

- [ ] Templating

    - [ ] Taxonomy-specific views
    - [ ] Standalone pages
    - [ ] Fully customizable "formats" to enable e.g. link-blogging, podcasting, slide shows, etc.

- [ ] Generate RSS

    - [ ] support podcast elements for RSS
    - [ ] render template not only into rendered content but also RSS/Atom

- [ ] Server mode

    It's nice to be able to generate everything statically, but depending on the site it may *also* be nice to have an actual server application, whether for generating content or simply for serving it in a non-static fashion if so desired. (There's a lot of thought that would need to go into figuring out what this flow would look like.)

    - [ ] Watchers – I want to be able to tweak content and regenerate it on the fly, or especially to be able to tweak a template and have it rebuild on the fly.
    - [ ] SCSS integration


- [ ] Embrace parallelism!

    - [ ] Via threading, e.g. with Rayon
    - [ ] Via `async`/`.await`?

- [ ] Supply (and make it easy to extend) a `create` command and interface. `lx create note`, `lx create journal` etc.

[pulldown-cmark]: https://crates.io/crates/pulldown-cmark
[Syntect]: https://crates.io/crates/syntect

## Why?

1.  Because I've spent the last half decade fighting with different solutions, and ultimately found all of them wanting for my personal site needs—which are, in a word, *quirky*.

    The short version is: my online presence includes everything from academic papers in theology to series on programming languages and from the [POSSE]-style source of my microblogging to poetry to music I've written.

    I need a combination of things no other single static site generator provides, including:

    -   custom taxonomies, allowing overlapping/non-hierarchical relationships beyond a single kind of 'tag': something might need to live in both **Art** and **Family** as top-level subjects, while going specifically in *Poetry* and *Cat*, while also being filed specifically as ***Writing*** rather than, say, audio. That kind of overlapping categorization exists in *very* few other tools.

    -   citation processing (probably, at least initially, via [Pandoc])

    -   speed: I have a steadily growing site, and I do *not* want to be spending thirty-plus seconds to generate it when I just want to write a blog post. This means two things:

        1. It needs to be fast—*really* fast—right out of the gate.
        2. It should ultimately include a caching strategy, or possibly even a database, but should always be writable via plain text files.

2.  Because I really like writing Rust.

    There are other tools out there that I could bend to my will here, e.g. [Eleventy][11ty], which I [*have*][v5] bent to my will. But I'd really rather work out something new in Rust than spend time fighting with a plugin system in

3.  Because I want to see if I can make the fastest (or at least: *one* of the fastest) static site generators out there. When all is said and done, this should be as fast as Hugo or Zola or similar.

[POSSE]: https://indieweb.org/POSSE
[Pandoc]: http://pandoc.org
[11ty]: http://www.metalsmith.io
[v5]: https://v5.chriskrycho.com/journal/how-i-publish-this-site/

---

## Notes

1. And therefore, not to be confused with the [*other* lightning][py-lightning] static site generator: that one is written in Python.

[py-lightning]: https://github.com/borismus/lightning
