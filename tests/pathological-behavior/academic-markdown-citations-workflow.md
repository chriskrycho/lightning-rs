---
title: Academic Markdown and Citations
subtitle: A workflow with Pandoc, BibTEX, and the editor of your choice.
date: 2015-07-26 13:50
tags: workflow, writing
summary: >
    Managing citations is painful—especially in plain text. But with a little
    setup, Pandoc and BibTEX can take a lot of the pain out of it, whether for
    Word documents or a static site generator.
modified: 2015-07-26 20:07
...

Much of my past few weeks were taken up with study for and writing and editing
[a paper] for one of my classes at Southeastern. I've been writing all of my
papers in Markdown ever since I got here, and haven't regretted any part of
that... except that managing references and footnotes has been painful at times.

[a paper]: http://www.chriskrycho.com/2015/not-exactly-a-millennium.html

Footnotes in Markdown look like this:

```markdown
Here is some text.[^fn]

[^fn]: And the footnote!
```

This poses no problems at all for normal footnotes. Academic writing introduces
a few wrinkles, though, which means that this has always been the main pain
point of my use of Markdown for writing papers.

Many academic citation styles (including the Chicago Manual of Style, on which
our seminary's [style guide] is based) tend to have a long version of the
footnote appear first, followed by short versions later. Nearly *all* academic
citations styles make free use of the ["ibid."] abbreviation for repeated
references to save space, time, and energy. Here is how that might look in
manually-written footnotes, citing the very paper in which I sorted this all
out:

```markdown
Some text in which I cite an author.[^fn1]

More text. Another citation.[^fn2]

What is this? Yet *another* citation?[^fn3]

[^fn1]: So Chris Krycho, "Not Exactly a Millennium," chriskrycho.com, July 22,
    2015, http://www.chriskrycho.com/2015/not-exactly-a-millennium.html
    (accessed July 25, 2015), ¶6.

[^fn2]: Contra Krycho, ¶15, who has everything *quite* wrong.

[^fn3]: ibid.
```

[style guide]: http://www.press.uchicago.edu/books/turabian/turabian_citationguide.html

["ibid."]: https://en.wikipedia.org/wiki/Ibid.

This seems straightforward enough, though it is a bit of work to get the format
right for each different kind of citation (articles, books, ebooks, electronic
references to articles...). Things *really* get complicated in the editing
process, though. For example, what if I needed to flip the order of some of
these notes because it became clear that the paragraphs needed to move around?
This happens *frequently* during the editorial process. It becomes particularly
painful when dealing with the "ibid."-type references, because if I insert a new
reference between two existing references, I have to go back in and manually add
all that the reference content again myself.[^footnotes]

Enter Pandoc and <span class='tex'>BibT<span class="texE">E</span>X</span>.

## Managing Citations
The idea of plain-text solutions to academic writing is not especially new; only
the application of Markdown to it is---and that, only relatively. People have
been doing this, and [documenting their approaches][healy], for quite a while.
Moreover, tools for managing references and citations have existed for quite
some time as well; the entire [L<span class="texA">A</span>T<span
class="texE">E</span>X][LaTeX] toolchain is largely driven by the concerns of
academic publishing, and as such there are tools in the <span class='tex'>L<span
class="texA">A</span>T<span class="texE">E</span>X</span> ecosystem which
address many of these problems.[^latex]

[healy]: http://kieranhealy.org/blog/archives/2014/01/23/plain-text/
[LaTeX]: http://www.latex-project.org

One such is <span class='tex'>BibT<span class="texE">E</span>X</span>, and the
later (more capable) <span class='tex'>BibL<span class="texA">A</span>T<span
class="texE">E</span>X</span>: tools for managing bibliographies in <span
class='tex'>L<span class="texA">A</span>T<span class="texE">E</span>X</span>
documents. The <span class='tex'>BibT<span class="texE">E</span>X</span>/<span
class='tex'>BibL<span class="texA">A</span>T<span class="texE">E</span>X</span>
approach to managing citations in a document is the use of the `\cite` command,
with the use of "keys" which map to specific documents: `\cite{krycho:2015aa}`,
for example.

This is not Markdown, of course. But other folks who have an interest in
Markdown and academic writing have put their minds to the problem already. Folks
such as Jon MacFarlane, the originator and lead developer of [Pandoc], perhaps
the single most capable text-conversion tool in existence. As it turns out,
Pandoc Markdown supports a [citation extension] to the basic markup. It's just a
variant on the <span class='tex'>BibT<span class="texE">E</span>X</span>
citation style that feels more at home in Markdown: a pair of braces and an `@`,
plus the citation key, like `[@krycho]`. Moreover, Pandoc knows how to use <span
class='tex'>BibT<span class="texE">E</span>X</span> libraries, as well as many
others, and [Citation Style Languages][csls] (<abbr>CSL</abbr>s) to generate
markup in *exactly* the format needed for any given citation style.[^citeproc]

[Pandoc]: http://pandoc.org
[citation extension]: http://pandoc.org/README.html#citations
[csls]: http://citationstyles.org

Instead of writing out all those citations details by hand, then, I can just
format my footnotes like this (assuming the citekey I had set up for the article
was `krycho:revelation:2015`):

```markdown
Some text in which I cite an author.[^fn1]

More text. Another citation.[^fn2]

What is this? Yet *another* citation?[^fn3]

[^fn1]: [@krycho:revelation:2015], ¶6.

[^fn2]: Contra [@krycho:revelation:2015], ¶15, who has everything *quite* wrong.

[^fn3]: [@krycho:revelation:2015].
```

This is much simpler and, importantly, has the exact same form for each
citation. Pandoc will take care of making sure that the first reference is in
the long form, later references are in the short form, and repeated references
are in the "ibid." form as appropriate. It even renders a properly sorted and
structured Works Cited section.[^styleset]

The slightly complex command I used to generate a Word document from a Markdown
file with citations (using my own <span class='tex'>BibT<span
class="texE">E</span>X</span> library and the Chicago Manual of Style
<abbr>CSL</abbr>) on the command line is:[^template]

```bash
$ pandoc revelation.md --smart --standalone \
--bibliography /Users/chris/Dropbox/writing/library.bib \
--csl=/Users/chris/Dropbox/writing/chicago.csl -o revelation.docx
```

To see an extended sample of this kind of usage in practice, take a look at the
[Markdown source] for the paper I wrote last week, using exactly this approach.
Every footnote that references a specific source simply has a cite key of this
variety. The header metadata includes a path to the bibliography file and a
<abbr>CSL</abbr>. (These could be configured globally, as well, but I chose to
specify them on a per-file basis so that if I want or need to use *different*
styles or a separate library for another file at a later time, I can do so with
a minimum of fuss. More on this below.)

[Markdown source]: http://www.chriskrycho.com/2015/not-exactly-a-millennium.txt

[Here] is the rendered result. You can see that it automatically generated
everything right down to the "ibid."-style footnotes. I made a few, fairly
minimal tweaks (replacing the search <abbr>URL</abbr> with an <abbr>ATLA</abbr>
database catalog reference and inserting a section break before the Works Cited
list), and turned the paper in---confident, for the first time since I started
seminary, that all of the references were in the right order and the right
format. With carefully formatted reference documents (with their own style
sets),[^reference] I was able to generate an actually *nice* <abbr>[PDF]</abbr>
version of the paper from another Word document, as well.[^pdf]

[Here]: /downloads/revelation.docx
[PDF]: /downloads/revelation-pretty.pdf

And, better yet, you don't even have to put citations in footnotes. As
[\@anjdunning] pointed out in a [tweet] response to the original version of this
post:

[\@anjdunning]: https://twitter.com/anjdunning
[tweet]: https://twitter.com/anjdunning/status/625415216575197184

> [\@chriskrycho] Don't put citekeys in a footnote: write everything as inline
> citations and it will also generate notes when asked by CSL def.  
> [∞](https://twitter.com/anjdunning/status/625415216575197184) July 26, 2015 17:19

[\@chriskrycho]: https://www.twitter.com/chriskrycho

In my standard example from above, then, you could simply do this:

```markdown
Some text in which I cite an author.[@krycho:revelation:2015, ¶6]

More text. Another citation.[Contra @krycho:revelation:2015, ¶15, who has
everything *quite* wrong.]

What is this? Yet *another* citation?[@krycho:revelation:2015]
```

This will generate the same markup for my purposes here; and as [\@anjdunning]
noted, it goes one step further and does what's appropriate for the
<abbr>CSL</abbr>. This might be handy if, for example, you wanted to use the
Chicago notes-bibliography style in one format, but switch to a simpler
parenthetical citation style for a different medium---or even if you had a paper
to submit to different journals with different standards. Having the citations
inline thus has many advantages.

Now, there are still times when you might want to split those out into distinct
footnotes, of course. That second one is a good candidate, at least for the way
I tend to structure my plain-text source. I find it useful in the case of
*actual* footnote content---i.e. text that I'm intentionally leaving aside from
the main text, even with reference to other authors---to split it out from the
main flow of the paragraph, so that someone reading the plain text source gets a
similar effect to someone reading the web or Word or <abbr>PDF</abbr> versions,
with the text removed from the flow of thought. In any case, it's quite nice
that Pandoc has the power and flexibility such that you don't *have* to.

Finally, you don't actually *need* the brackets around the citekey, depending on
how you're using the reference. If you wanted to cite the relevant author
inline, you can---and it will properly display both the inline name and a
reference (footnote, parenthetical, etc.) in line with the <abbr>CSL</abbr>
you've chosen. If I were going to quote myself in a paper, I would do something
like this:

```markdown
As @krycho:revelation:2015 comments:

> This was a hard paper to write.

```

This is *extremely* powerful, and while I didn't take advantage of it in my
first paper using these tools, you can bet I will be in every future paper I
write.

### All those references
Of course, as is probably apparent, managing a <span class='tex'>BibT<span
class="texE">E</span>X</span> library by hand is no joke. Entries tend to look
like this:

```tex
@book{beale:revelation:2015,
        Date-Added = {2015-07-20 21:16:02 +0000},
        Date-Modified = {2015-07-20 21:21:05 +0000},
        Editor = {G. K. Beale and David H. Campbell},
        Publisher = {William B. Eerdmans Publishing Company},
        Title = {Revelation: A Shorter Commentary},
        Year = {2015}}
```

While there is a lot of utility in having that data available in text, on disk,
no one wants to *edit* that by hand.[^noone] Gladly, editing it by hand is not
necessary. For this project, I used the freely available [BibDesk] tool, which
is a workable (albeit not very pretty and not *very* capable) manager for <span
class='tex'>BibT<span class="texE">E</span>X</span>:

[BibDesk]: http://bibdesk.sourceforge.net

![BibDesk -- open to the library for my Revelation paper](//cdn.chriskrycho.com/images/bibdesk.png "Not very pretty, but it does work")

Once I filled in the details for each item and set a citekey for it, I was ready
to go: BibDesk just stores the files in a standard `.bib` file on the disk,
which I specified per the Pandoc command above.

BibDesk gets the job done alright, but only alright. Using a citation and
reference management tool was a big win, though, and I fully intend to use one
for every remaining project while in seminary---and, quite possibly, for other
projects as well. Whether that tool is BibDesk or something else is a different
matter entirely. (More on this below.)

## To the web!
I wanted something more out of this process, if I could get it. One of the
reasons I use plain text as a source is because from it, I can generate Word
documents, <abbr>PDF</abbr>s, and *this website* with equal ease. However,
Python Markdown knows nothing of <span class='tex'>BibT<span
class="texE">E</span>X</span> or citekeys, to my knowledge---and since I render
everything for school with Pandoc, I have long wanted to configure [Pelican] to
use Pandoc as its Markdown engine instead of Python Markdown anyway.

[Pelican]: http://docs.getpelican.com/en/3.6.0/

As it happens, I actually set this up about a month ago. The process was pretty
simple:[^pelicanconf]

 1. I installed the [pandoc-reader] Pelican extension.
 2. I set the plugin path in my Pelican configuration file.
 3. I specified the arguments to Pelican I wanted to use.

The only additional tweaks necessary to get citation support were calling it
with the `'--filter pandoc-citeproc'` arguments, which lets it process any
bibliography data supplied in the header metadata for the files. Calling Pandoc
with `--bibliography <path to bibliography>` (as in my example above) is a
[shortcut] for calling it with `--metadata <path to bibliography>` *and* the
`--filter pandoc-citeproc` arguments. I could just supply the bibliography
directly in the call from Pelican, but this would limit me to using a single
bibliography file for *all* of my posts---something I'd rather not limit myself
to, since it might make sense to build up bibliographies around specific
subjects, or even to have smaller bibliographies associated with each project
(exported from the main bibliography), which could then be freely available
along with the contents of the paper itself.[^smarter] (On this idea, see a bit
more below under **The Future**.)

[pandoc-reader]: https://github.com/jstvz/pelican-pandoc-reader
[conf]: https://github.com/chriskrycho/chriskrycho.com/blob/ef3ecbca1765750392086355aeae026c1159d4b9/pelicanconf.py#L109
[shortcut]: http://pandoc.org/README.html#citation-rendering

One word of warning: Pandoc is much slower to generate <abbr>HTML</abbr> with
`--filter pandoc-citeproc` than *without* the filter, and the larger your site,
the more you will feel this. (The time to generate the site from scratch jumped
from about 10s to about 30s for me, with 270 articles, 17 drafts, 2 pages, and
1 hidden page, according to Pelican.) Pandoc has to process *every* article to
check for citations, and that's no small task. However, if you have Pelican's
content caching turned on, this is a one-time event. After that, it will only be
processing any new content with it; total generation time is back down where it
was before for me: the effort is all in generating the large indexes I use to
display the content for the landing pages and for category and tag archives.

And the result: that same paper, rendered to <abbr>HTML</abbr> [on my website],
with citations and works cited, generated automatically and beautifully.

[on my website]: http://www.chriskrycho.com/2015/not-exactly-a-millennium.html

### Other site generators
I don't know the situation around using Pandoc itself in other generators,
including Jekyll---I simply haven't looked. I do know, however, that there *is*
some tooling for Jekyll specifically to allow a similar workflow. If you're
using Jekyll, it looks like your best bet is to check out [jekyll-scholar] and
the [citeproc-ruby] project, which (like pandoc-citeproc) enables you to embed
citations and filter them through <abbr>CSL</abbr>s to generate references
automatically. As a note: you should definitely be able to get those working on
your own deployment sites, but I have no idea whether it's possible to do them
with the GitHub Pages variant of Jekyll. (If anyone who reads this knows the
answer to that, let me know on Twitter or App.net, and I'll update the post
accordingly.)

[jekyll-scholar]: https://github.com/inukshuk/jekyll-scholar
[citeproc-ruby]: https://github.com/inukshuk/citeproc-ruby

## The future
In addition to continuing to use <span class='tex'>BibT<span
class="texE">E</span>X</span> with BibDesk as a way of managing my citations in
the short term, I'm thinking about other ways to improve this workflow. One
possibility is integrating with [Scholdoc] as it matures, instead of [pandoc],
and maybe (hopefully, albeit unlikely) even contributing to it somewhat. I'm
also open to using other citation library tools, though my early explorations
with Mendeley and Zotero did not particularly impress me.

[Scholdoc]: http://scholdoc.scholarlymarkdown.com

There are substantial advantages for the applications (and thus for most users)
to maintaining the data in an application-specific format (e.g. an SQLite
database) rather than on the file system---but the latter has the advantage of
making it much easier to integrate with other tools. However, Zotero and
Mendeley both natively export to <span class='tex'>BibT<span
class="texE">E</span>X</span> format, and Mendeley natively supports [sync] to a
<span class='tex'>BibT<span class="texE">E</span>X</span> library (Zotero can do
the same, but via third-party [plugins]), so those remain viable options, which
I may use for future projects.

[sync]: http://blog.mendeley.com/tipstricks/howto-use-mendeley-to-create-citations-using-latex-and-bibtex/
[plugins]: https://zoteromusings.wordpress.com/tag/bibtex/

I also want to look at making my library of resources available publicly,
perhaps (a) as a standalone library associated with each project, so that anyone
who wants to can download it along with the Markdown source to play with as an
example and (b) as a general library covering my various reading and research
interests, which will certainly be irrelevant to most people but might
nonetheless provide some value to someone along the way. I'm a big fan of making
this kind of data open wherever possible, because people come up with neat
things to do with it that the original creators never expect. Not *everything*
should be open---but lots of things should, and this might be among them.

## Summary
I'm pretty happy with the current state of affairs, the aforementioned interest
in other reference managers notwithstanding:

  - I can set up the citations *once*, in a tool designed to manage references,
    instead of multiple times in multiple places.
  - I can use Pandoc and a <abbr>CSL</abbr> to get the citations formatted
    correctly throughout a paper, including generating the bibliography
    automatically.
  - I can use the same tooling, integrated into my static site generator, to
    build a web version of the content---with no extra effort, once I configured
    it properly the first time.

Perhaps most importantly, this helps me meet one of my major goals for all my
writing: to have a single canonical *source* for the content, which I will be
able to access in the future regardless of what operating system I am using or
what publishing systems come and go. Simple plain text files---Markdown---get me
there. Now I've put good tools around that process, and I love it even more.



[^footnotes]: Coming up with names for footnotes in Markdown can be painful in
    general for long documents. If you try to name them manually, like I do for
    posts on my website, you will very quickly end up wasting time on the names.
    If you try to number them, they will end up out of order in a hurry. My own
    [previous solution] to this problem quickly became unwieldy for larger
    papers, and required a *lot* of hand-editing. Gladly, I no longer deal with
    that manually. Instead, I do all my drafting in [Ulysses], where you just
    type `(fn)` and it creates a footnote automatically, and will move that
    footnote *object* around transparently as you edit, handling all the
    number-setting, etc. on its own.

[previous solution]: http://2012-2013.chriskrycho.com/web/markdown-and-academic-writing/
[Ulysses]: http://www.ulyssesapp.com

[^latex]: The irony of site for software which boasts that it is "a high-quality
    typesetting system" and looks like [*this*][LaTeX] is not lost on me...

[^citeproc]: If you used the installers on Pandoc's website, `pandoc-citeproc`
    comes with it. If you installed it via a package manager (e.g. by running
    `brew install pandoc`), it may not have, so you'll need to install it
    manually yourself (e.g. `brew install pandoc-citeproc`).

[^styleset]: All of the content, including the rendered footnotes and the
    bibliography, has sensible content types set on it: headers are headers,
    body text is body text, etc. You can then customize to match the
    specifications of your style guide. I have a Chicago/Turabian style set set
    up with the formatting rules to match.

[^template]: Actually, it was even hairier than this, because I also had a
    `--reference-docx path/to/template.docx` specified. If you think it's
    perhaps a bit too complex, well, I agree. I plan to turn that into a command
    line alias in pretty short order, because remembering it every time is just
    not going to happen.

[^reference]: Using the `--reference-docx` argument to Pandoc, you can hand it a
    document that already uses your desired style set, so you don't have to go
    in and apply it manually.

[^pdf]: I could have done that with Pandoc's
    <span class='tex'>L<span class="texA">A</span>T<span class="texE">E</span>X</span>
    <abbr>PDF</abbr> tools, as well, but didn't really feel like taking the time
    to tweak the <span class='tex'>L<span class="texA">A</span>T<span class="texE">E</span>X</span>
    template for it.

[^noone]: Probably someone does, but not me, and not most people!

[^pelicanconf]: If you're using Pelican, you can take a look at my Pelican
    configuration file [here][conf] to see the full configuration for using
    Pandoc this way.

[^smarter]: Optimally, I'd really just prefer to be able to set *all* of these
    arguments at a per-file level---i.e., not use `--filter pandoc cite-proc`
    unless the file actually specifies a bibliography. And I could hack Pelican
    to do that; I've actually already [messed around] with other, semi-related
    bits regarding Pelican and Pandoc's shared handling of <abbr>YAML</abbr>
    metadata. But I'd prefer to keep my installation as "vanilla" as possible to
    minimize the cost of setting things up again on a new machine or after a
    crash, etc.

[messed around]: https://github.com/liob/pandoc_reader/pull/5
