---
Title: Static Site Generators and Podcasting
Subtitle: Publishing podcasts the hard way.
Tags: software development, podcasting, python, rust
Date: 2016-02-28 12:50
...

Presently, I publish both [Winning Slowly] and [New Rustacean][^others] using
what is admittedly a bit of a quirky approach. It works well for me, and I think
it's worth documenting for other nerdy types out there, but if you're just
getting going with podcasting and you're looking for the easy way to do it, let
me warn you: *this isn't it*. Something like [SoundCloud] and a blog for show
notes, or [WordPress] with [Blubrry PowerPress] is what you want instead. This
approach works *extremely* well for statically-generated sites, however, and I
imagine a few people out there might find it useful.

[Winning Slowly]: http://www.winningslowly.org/
[New Rustacean]: http://www.newrustacean.com/
[SoundCloud]: https://soundcloud.com/for/podcasting
[WordPress]: https://wordpress.org
[Blubrry PowerPress]: https://wordpress.org/plugins/powerpress/


The short version
-----------------

  - Generate the feeds with [Feeder].
  - Generate the site statically with something else (and it *really* doesn't matter what).
  - Copy the feed into the generated site.

[Feeder]: http://reinventedsoftware.com/feeder/


The long version
----------------

I generate the sites themselves with [Pelican] and [`cargo doc`][NR::e001],
respectively. I was already comfortable with Pelican because it's what I use to
generate *this* site (with a few [tweaks] to the standard configuration,
especially using [Pandoc] rather than the Python Markdown implementation), so I
ran with it for building the Winning Slowly site, and it has worked quite well
for building the site itself. It just gets built locally and deployed via
[GitHub Pages].

[Pelican]:http://docs.getpelican.com/en/3.6.3/
[NR::e001]: http://www.newrustacean.com/show_notes/e001/index.html
[tweaks]: https://github.com/chriskrycho/chriskrycho.com/blob/master/pelicanconf.py
[Pandoc]: http://pandoc.org/
[GitHub Pages]: https://pages.github.com/

However, it does not have built-in support for generating [podcast feeds],
even just the general case with enclosures. [iTunes podcast support] would have
taken a lot of work to add.[^pelican] Instead, I chose to build the RSS feed
semi-manually. *Semi*-manually, because doing it totally manually is a recipe
for making mistakes. XML is many things, but "easy to write correctly by hand"
is not one of them. I use [Feeder] to manage the feeds, and *it* makes sure that
the enclosure and iTunes elements are set correctly.

[podcast feeds]: https://en.wikipedia.org/wiki/RSS_enclosure
[iTunes podcast support]: https://itunespartner.apple.com/en/podcasts/overview

The biggest upside to this is that I can use Pelican without modification to how
it generates feeds (apart from optionally turning them off entirely). It just
[copies] the feed I generate to the output file during its normal build process.
As suggested above, I also *don't* generate the other feeds which Pelican
supports, as we have no need for them; we only care about the podcast feed.

[copies]: https://github.com/WinningSlowly/winningslowly.org/blob/master/pelicanconf.py#L99

This process works equally well, with very little modification, for New
Rustacean. In that case, I'm generating the content by running Rust's
documentation tool, `cargo doc`[^rustdoc] to render the "API docs" which serve
as show notes. Notice the family resemblance between [my "show notes"] and, say,
the [Diesel docs], which are both generated the same way. This is *not* a normal
way of building a podcast website; you can hear me explain why I did it this way
in [New Rustacean e001: Document all the things!][NR::e001] In any case, I just
take the show note-relevant parts of the documentation and put it in Feeder,
generate the feed, and [copy that as part of the build process][copy].

[my "show notes"]: http://www.newrustacean.com/show_notes/
[Diesel docs]: http://sgrif.github.io/diesel/diesel/index.html
[copy]: https://github.com/chriskrycho/newrustacean.com/blob/master/Makefile#L32

That's it!


[^others]: And, incidentally, [Sap.py] and my [sermons] feed.

[^pelican]: If I stick with Pelican long-term, I might look into adding it
    anyway, but honestly, I don't love Pelican. The reasons have little to do
    with Pelican for itself, and a lot more to do with my particular and
    somewhat peculiar needs. That's a post for another day. In any case, I'm
    likelier to use another generator---even one I write myself!---than to do the
    work to make Pelican do what I want.

[^rustdoc]: Technically, Rust's documentation tool is `rustdoc`, which
    `cargo doc` wraps around. I never actually use `rustdoc` directly, though.

[Sap.py]: http://www.sap-py.com
[Sermons]: http://www.chriskrycho.com/sermons.xml
