---
title: Colophon
subtitle: Or, how this site is made.
---

## Privacy

I currently use [Fathom] for lightweight analytics on the site. I picked them because they don’t track you.

[Fathom]: https://usefathom.com

I also use [Adobe Fonts] for one of the typefaces on the site (see [below](#typography)), and Adobe tracks font usage. If you find a type-face that has some of the same character as Cronos Pro, [shoot me an email][cronos-email]. I will seriously consider replacing it if I find something I like equally well; I *loathe* Adobe’s approach to font licensing *and* to privacy.

[cronos-email]: mailto:hello@chriskrycho.com?subject=Cronos%20Pro%20alternative

## Implementation

I built this version of the site with [Eleventy]. You can find the entirety of the implementation (and indeed the entire *history* of the implementation) [on GitHub][repo]. I'm using it with the following plugins:

- <b>typeset:</b> my own implementation of a plugin for [typeset], heavily inspired by [eleventy-plugin-typeset].

- <b>[markdown-it] plugins:</b>
    - [abbreviations](https://github.com/markdown-it/markdown-it-abbr)
    - [anchor](https://github.com/valeriangalliat/markdown-it-anchor)
    - [definition list](https://github.com/markdown-it/markdown-it-deflist)
    - [div](https://github.com/kickscondor/markdown-it-div)
    - [footnote](https://github.com/markdown-it/markdown-it-footnote)
    - [implicit-figures](https://github.com/arve0/markdown-it-implicit-figures)
    - [superscript](https://github.com/markdown-it/markdown-it-sup)

- <b>spacewell:</b> a little tool I built a few years ago to insert hair spaces around em dashes and thin spaces with non-breaking spans around number-separating en dashes; the source is colocated with the rest of the site.

[Eleventy]: https://www.11ty.io
[repo]: https://github.com/chriskrycho/v5.chriskrycho.com
[typeset]: https://typeset.lllllllllllllllll.com
[eleventy-plugin-typeset]: https://github.com/johanbrook/eleventy-plugin-typeset
[markdown-it]: https://github.com/markdown-it/markdown-it

## Typography

Perhaps my favorite part of web design, and also the part with which I spend the most part *fussing*.

| Context  | Typeface |
| -------- | -------- |
| Body text | [Sabon], designed by Jan Tschichold in the mid-1960s as a Garamond revival. In my opinion, the most beautiful Garamond in existence. Licensed via [Fonts.com]. |
| Headings  | [Cronos], designed by Robert Slimbach in 1996. A nice contrast to Sabon with its digital-era roots. The typeface I’ve been using on my site the longest at this point! Licensed via [Adobe Fonts].[^adobe] |
| Code      | [Hack], designed by Chris Simpkins in 2015 as an extension of the Deja Vu/Bitstream Vera lineage. Licensed in parts under the <abbr title="Massachusetts Instititue of Technology">MIT</abbr> License, the public domain, and Bitstream Vera License (see details [here][hack-license]). |

[Sabon]: https://www.myfonts.com/fonts/linotype/sabon/
[Fonts.com]: https://www.fonts.com
[Adobe Fonts]: https://fonts.adobe.com
[Cronos]: https://www.myfonts.com/fonts/adobe/cronos/
[Hack]: https://sourcefoundry.org/hack/
[hack-license]: https://github.com/source-foundry/Hack/blob/master/LICENSE.md

[^adobe]: Longtime readers may recall (and new readers may be curious about) my [deep frustrations with this situation][cronos-writeup]. Nothing there has changed—but I ended up paying for Adobe's Lightroom package when I [picked back up photography][photographying], and it comes bundled. So here we are.

[cronos-writeup]: https://v4.chriskrycho.com/2019/cronos-follow-up.html
[photographying]: https://v4.chriskrycho.com/2019/photography-ing-again.html

## Inspiration

While working on this design, I took more-or-less-direct inspiration in a variety of ways from some of my favorite current or previous designs around the web:

- [Tim Brown](https://tbrown.org)

- [Ethan Marcotte](https://ethanmarcotte.com)

- [Trent Walton](https://trentwalton.com)

- [Craig Mod](https://craigmod.com)—a truly wonderful site, but I actually loved the *previous* design iteration even more.

- [Jen Simmons](https://jensimmons.com)—as with Mod’s current site, I really like what she’s doing on her site now—it’s all sorts of fun layout-wise—but it’s quite distinct from what she was doing when I was stealing ideas from her late in 2018!

- [Reda Lemeden](https://redalemeden.com)

- [Jason Santa Maria](http://jasonsantamaria.com/)—from whom, if I recall, I originally got [the idea](https://v4.chriskrycho.com/2019/my-final-round-of-url-rewrites-ever.html) of a versioned website.

## Copyright and License

All content is {% copyright build.date, 'content' %}. All custom software components are {% copyright build.date, 'implementation' %}.

[LICENSE.md]: https://github.com/chriskrycho/v5.chriskrycho.com/blob/master/LICENSE.md