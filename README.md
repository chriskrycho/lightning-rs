# Lightning (`lx`)

[![Build Status](https://travis-ci.org/chriskrycho/lightning-rs.svg?branch=master)](https://travis-ci.org/chriskrycho/lightning-rs)

Yet another static site generator—but this one's written in Rust. (And therefore, not to be confused with the [*other* lightning] static site generator: that one is written in Python.)

[*other* lightning]: https://github.com/borismus/lightning

## Status

This currently ***does not work***. If you need a site generator that does, I can point you to [Hugo], which is great, speedy, and pretty well-documented. If you want one specifically in Rust, check out [Cobalt], which is young but already works, and looks to be fairly compatible with Jekyll sites right out of the box. ([Why am I building something else if Cobalt already exists?](#why))

Today, Lightning *builds*, passes the tests I've written, and even correctly loads the configuration file. But running `lx build` will *not* do what you expect: it'll convert all the Markdown in a config-specified directory, but it won't render it into templates in any way. Keep your expectations low on how fast this will develop, and you won't be disappointed.

[Hugo]: https://gohugo.io
[Cobalt]: http://cobalt-org.github.io

### Goals

This project's main goals are:

- speed
- ease of use, even for more complex ways of structuring a site
- good out-of-the-box defaults, but with human-readable and -writable configurability
- straightforward *import* from other systems (though see comment below)
- extended Markdown functionality like processing citations

It is an explicit non-goals to be an exact drop-in replacement for any other generator. Supporting the patterns other generators use for ease of import is good; requiring that everyone conform to e.g. Jekyll's, Hugo's, or any other generator's patterns as a result is *not* good. It should be easy to migrate in Jekyll/Hugo/etc. content; but you will never have to format the titles of your posts in any particular way.

### Roadmap

N.b. the below is my overall set of goals. For the 1.0 roadmap, see the [milestone](https://github.com/chriskrycho/lightning-rs/milestone/1) and the [tracking issue](https://github.com/chriskrycho/lightning-rs/issues/3).

- [ ] Define configuration scheme

    - [x] **Support custom taxonomies**: not being limited to just having categories and tags, and pages being off in their own corner

        - [x] binary (true/false)

        - [x] tag-like: items may belong to multiple items

        - [x] hierarchical: items may belong to parents and children e.g. something can be at `Tech/Programming` and thus belong to both Tech and Programming

            - [x] hierarchical *and exclusive*, i.e. if something is in the category `Tech` it *cannot* be in the category `Art`. I don't actually want or need this, but other users almost certainly will.

    - [ ] Support importing content from other generators' basic setups.
        
        This really means, make sure the configuration can support the configuration patterns for popular generators. This is not so much a *formal support* issue (though being able to `lx create --from-jekyll` would be cool) as it is a *make sure this is well-covered by the implementation* issue. Other generators to cover, in order:

        - [x] from [Pelican] – a must for the obvious reason that I want to be able to import my existing sites.

        - [ ] from [Jekyll] – a high priority given the sheer popularity of the generator

        - [ ] from [Hugo] – because it's basically a direct competitor if I ever get this thing to where I want performance-wise

- [ ] Render Markdown

    - [ ] with [pulldown-cmark]

    - [ ] with [pandoc]
        - [x] via [subprocess][cmd-pandoc]
        - [ ] as a library

    - [ ] via [Hoedown] bindings? (big maybe, but it has the upside of being very widely used b/c of Jekyll and such.)

    - [x] optionally using [Syntect] for syntax highlighting

- [ ] Templating 

    - [ ] Taxonomy-specific views

    - [ ] Standalone pages

    - [ ] Fully customizable "formats" to enable e.g. link-blogging, podcasting, slide shows, etc.

- [ ] Server mode

    It's nice to be able to generate everything statically, but depending on the site it may *also* be nice to have an actual server application, whether for generating content or simply for serving it in a non-static fashion if so desired. (There's a lot of thought that would need to go into figuring out what this flow would look like.)

- [ ] Generate RSS

    - [ ] support podcast elements for RSS

    - [ ] render template not only into rendered content but also RSS/Atom

- [ ] Embrace parallelism!

    - [ ] Via threading?

    - [ ] Via [futures-cpupool] or similar? 

- [ ] Extensibility

    - [ ] via new commands, which can be installed and run _a la_ Git or Cargo commands (`cargo clippy` just runs the `cargo-clippy` binary)

    - [ ] via some other system of plugin? (I'm inclined against plugin approach because it forces people to spend their time gluing things together.)

- [ ] Supply (and make it easy to extend) a `create` command and interface.

    It's hard to overstate how much utility I get out of the `ember generate` family of commands, or how much I use the hacked-together Python CLI I use to generate stubs for posts. And both Jekyll and Hugo have tools for this, and it's very handy.

- [ ] Watchers – I want to be able to tweak content and regenerate it on the fly, or especially to be able to tweak a template and have it rebuild on the fly. (This may be a good thing to integrate with the **Server Mode** noted above.)

What else should be on this list?

[Pelican]: http://docs.getpelican.com/en/stable/
[Jekyll]: http://jekyllrb.com
[pulldown-cmark]: https://crates.io/crates/pulldown-cmark
[cmd-pandoc]: https://crates.io/crates/cmd-pandoc
[Hoedown]: https://crates.io/crates/hoedown
[Syntect]: https://crates.io/crates/syntect
[futures-cpupool]: https://docs.rs/futures-cpupool/0.1.2/futures_cpupool/

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

# Building
cmake is a dependency. Install it and make sure the path to it is in .bash_profile, .profile or .bashrc.