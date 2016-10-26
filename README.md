# Lightning

Yet another static site generator—but this one's written in Rust.

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

    There are other tools out there that I could *probably* bend to my will here, e.g. [Metalsmith]. But I'd really rather work out something new in Rust than spend time fighting with a plugin system in

3.  Because I want to see if I can make the fastest (or at least: *one* of the fastest) static site generators out there. When all is said and done, this should be as fast as [Hugo]. (That's a pretty high bar; Hugo is great, and if you want a static site generator *today*, especially if you don't have my quirky needs, that's what I would point you to.)

[POSSE]: https://indieweb.org/POSSE
[Pandoc]: http://pandoc.org
[Metalsmith]: http://www.metalsmith.io
[Hugo]: https://github.com/spf13/hugo

## Roadmap

N.b. this is my overall set of goals, with no specific ties to a timeline or a planned 1.0. I'll hammer that out when I get a little futher along.

- [ ] Define configuration scheme

    - [ ] **Support custom taxonomies**: not being limited to just having categories and tags, and pages being off in their own corner

        - [ ] binary (true/false)

        - [ ] tag-like: items may belong to multiple items

        - [ ] hierarchical: items may belong to parents/children but not siblings, e.g. something can be at `Tech/Programming` and thus belong to both Tech and Programming, but not simultaneously to `Art`

            I don't actually want or need this, but other users almost certainly will.

        - [ ] tag-like *and* hierarchical???

    - [ ] Support importing Jekyll/etc.

- [ ] Render Markdown

    - [ ] with [pulldown-cmark]

    - [ ] with [pandoc]
        - [ ] via [subprocess][cmd-pandoc]
        - [ ] as a library

    - [ ] via [Hoedown] bindings? (big maybe, but it has the upside of being very widely used b/c of Jekyll and such.)

    - [ ] optionally using [Syntect] for syntax highlighting

- [ ] Extensibility

    - [ ] via new commands, which can be installed and run _a la_ Git or Cargo commands (`cargo clippy` just runs the `cargo-clippy` binary)

    - [ ] via some other system of plugin? (I'm inclined against plugin approach because it forces people to spend their time gluing things together.)

What else should be on this list?

[pulldown-cmark]: https://crates.io/crates/pulldown-cmark
[cmd-pandoc]: https://crates.io/crates/cmd-pandoc
[Hoedown]: https://crates.io/crates/hoedown
[Syntect]: https://crates.io/crates/syntect
