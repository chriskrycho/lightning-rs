Title: Doing It Myself
Date: 2014-03-21 22:14
Author: Chris Krycho
Tags: software development
Summary: Working with Pelican&mdash;the static site generator I use for my blog currently&mdash;has reinforced my desire to write my own such software. Sometimes, you just have to do it yourself.

Last summer, I started work on a project I named [Step Stool][ss]---aiming to
make a static site generator that would tick of all the little boxes marking my
desires for a website generator. In due time, the project got put on hold, as I
started up classes again and needed to focus more on my family than on fun side
projects.

Come the beginning of 2014, I was ready to bit WordPress farewell once and for
all, though. While [Ghost][ghost] looks interesting, since I do all my writing
in Markdown files, there is something tempting about the canonical version of
the documents being the version on my computer (and thus also on my iPad and
iPhone and anywhere I have Dropbox and/or Git access). I did not have time at
the beginning of the year to finish writing Step Stool, and I knew as much,[^1]
so instead I moved to [Pelican][pelican] as a stop-gap. There were lots of good
reasons to pick Pelican: it has an active development community, fairly thorough
documentation,[^2] and it's in Python and uses Jinja2 templates---the same basic
approach I had taken with Step Stool, and the same toolset.

Unfortunately, while I have been glad to be away from WordPress, my experience
with Pelican so far has only reinforced my desire to get Step Stool done. There
are *lots* of little things that it does in ways that just annoy me. Many of
them have to do with configuration and documentation. On the latter, while the
documentation is *fairly* complete, there are quite a few holes and gaps. (Yes,
yes, open source software and anyone can add to the docs. That's great---it
really is---but if I'm going to use someone else's solution, it had better *just
work*. Otherwise, I'd rather spend my time getting my own going.)

For example, if you want to see how the pagination actually works, good luck
figuring it out from the documentation. You'll need to go looking at the way the
sample themes (yes, both of them) are implemented to start getting a feel for
it. Along the same lines, many of the objects that get handed to the templates
are not fully documented, so it is difficult to know what one can or cannot do.
I do not particularly want to spend my time adding debug print statements to my
templates just to figure out what options I have available.

The same kinds of things hold true for configuration options. Moreover, the
configuration is done through a Python module. While that makes the module
easier to integrate on the code side of things, it makes its actual content much
less transparent than one might hope. Python is not really well optimized for
writing configuration files---nor is any normal programming language.
Configuration is inherently declarative, rather than imperative.

This is not to say that Pelican is bad software. It is not. It is, however, a
fairly typical example of open source software implemented by committee. It has
holes (some of them serious), bumps, and quirks. Here is the reality: so will
Step Stool, though they will be the quirks that come from an individual
developer's approach rather than a group's. But the one thing I can guarantee,
and the reason I am increasingly motivated to get back to working on Step Stool.
And yes, I do have a couple other projects on my plate as well---contributions
to the Smartypants and Typogrify modules, my own [Spacewell typography project][spacewell],
and quite possibly a [Markdown Poetry extension ][md-poetry]. But I would like
very much to just get back to doing this myself. There is freedom in rolling my
own solution to things. I will not always have time to do these kinds of things; 
I figure I should do them when I can.

So here's to [Step Stool][ss], and---more importantly---to writing your own
software just to scratch that itch.

[ss]: http://step-stool.io
[ghost]: https://ghost.org
[pelican]: http://docs.getpelican.com/en/3.3.0/
[spacewell]: https://bitbucket.org/chriskrycho/spacewell
[md-poetry]: https://bitbucket.org/chriskrycho/markdown-poetry/

[^1]: I spent quite a bit of time tweaking my friend Vernon King's [Jekyll-powered site](http://www.vernonking.org), I got Winning Slowly off the ground,
including designing the site from scratch and implementing it (also in Pelican),
and I did some substantial redesign work on this site. That was more than enough
for my three week break---as evidenced by the fact that I didn't get to the sort
of 1.0 version of this site until just a week or so ago.

[^2]: Emphasis on "fairly." More on *that* in a moment as well.