---
Title: CSS Fallback for OpenType Small Caps
Subtitle: A not-so-great solution which still gets the job done.
Date: 2015-10-19 20:00
Tags: design, software development, typography
...

<i class=editorial>I wrote this up as [a question on Stack Overflow][?] a bit
over a year ago. It has continued to get a fair bit of traffic, so I've
republished it here and cleaned it up a bit.</i>

[?]: http://stackoverflow.com/questions/24846264/css-fallback-for-opentype-small-caps/25172932#25172932


The Problem
-----------

Over the last year, I've worked on [a site][hb] where small caps are important:
setting the text of the Bible. In the Old Testament the name of God is
transliterated as `Lord` but in small caps---not "LORD" but <span
class="divine-name">Lord</span> (RSS readers will want to click through and see
this on my site). However, the state of OpenType small caps support at the
moment is… less than optimal. Safari (even up through Safari 9 on El Capitan,
from which I am typing this) still doesn't support the
`-webkit-font-feature-settings: 'smcp'` option, and a lot of the hits for this
website will be coming from mobile.

[hb]: //holybible.com

Unfortunately, "graceful degradation" is problematic here: if you specify both
`font-variant: small-caps` and `font-feature-settings: 'smcp'` in a browser that
supports the latter (e.g. Chrome), the `font-variant` declaration overrides it,
so the horribly ugly old-style version still comes into play. (Note: this is as
it should be per the [spec]: the `font-variant` declaration has a higher
priority than the `font-feature-settings` declaration). Given the current
implementations of `font-variant: small-caps`, though---shrunken capitals rather
than actual small capitals---the result is that using `font-variant: small-caps`
realists in not-so-gracefully degrading *everyone's* reading experience.

[spec]: http://www.w3.org/TR/css-fonts-3/#feature-precedence

In the past, I have exported the small caps as a distinct webfont and specified
them directly; see [this post][qml] for a simple example: the first line of each
paragraph is specified that way.

[qml]: http://www.chriskrycho.com/2014/learning-qml-part-1.html

While I *can* do the same thing here (and at least in theory could deliver a
pretty small typeface, since I really only need three characters: `o`, `r`, and
`d`), I'd prefer simply to enable sane fallbacks. As noted above, however,
that's not possible. I am *open to* but would very much prefer to avoid
server-side solutions (browser detection, etc.) as a point of complexity that is
better to minimize, especially given how rapidly browsers change. How else might
one solve this problem, and especially are there existing solutions for it?

In the future, `font-variant: small-caps`
will handle this nicely, as per [the spec] it should display a
small-capitals-variant of the typeface if the typeface supplies it. However, at
present, *no browser supports this* (at least, none that I can find!). This
means that instead, they all render fake small capitals simply by scaling down
actual capitals. The result is typographically unpleasant, and unacceptable on
this project.

[the spec]: http://www.w3.org/TR/css3-fonts/#small-caps


The Solution(s)
---------------

I spent a considerable amount of time researching this and wrestling with it.
After digging around as best I could, the top solutions for now are:

### `@supports`

Take advantage of the `@supports` rule in browsers. This is what I initially
opted to do on this project.[^1] You use the rule this way:

```css
.some-class {
    font-variant: small-caps;
}

@supports(font-feature-settings: 'smcp') {
    .some-class {
        font-variant: normal;
        font-feature-settings: 'smcp';
    }
}
```

(I've simplified by leaving out the prefixed versions; you'll need to add the
`-webkit-` and `-moz-` prefixes to get this actually working.) This has the
advantage that support for real small caps and support for the `@supports` rule
are very similar:

  - `@supports`: [Can I Use Feature Queries?][\@supports]: Chrome 31+, Firefox
    29+, Opera 23+, Android 4.4+, Safari 9+, Edge 12+, Chrome for Android
  - `font-feature-settings`: [Using Small Caps & Text Figures on the Web][ffs]:
    Chrome, Firefox, IE10+

[\@supports]: http://caniuse.com/#feat=css-featurequeries
[ffs]: http://usabilitypost.com/2014/05/10/using-small-caps-and-text-figures-on-the-web/

This isn't perfect: since IE10/11 don't implement `@supports`, you miss one
browser---sort of. At this point, IE is a legacy browser, and Edge has had
`@supports` available from the start. Thus, this gets you most of the way there,
and it should be future-facing: this should progressively enhance the site
nicely. The normal (bad, but functional) small caps are displayed in the
meantime, and when browsers eventually get around to using OpenType small caps
by default for `font-variant: small-caps`, this will continue to work just fine.
It's "progressive enhancement" and it'll work nicely for most purposes.[^2]

### Typeface subsetting

As mentioned above, one can create a subset of the typeface that includes only
small capitals. This is what I have done for the small caps on this site; see
the example in the first paragraph.

To pull this off, you'll need to start by subsetting the typeface. You can do
this manually with a font tool, or (the simpler way) you can use FontSquirrel's
custom subsetting tool in their [webfont generator][generator]. (***Note:*** You
*must* check the license and confirm that the typeface in question allows this
kind of modification. See below.) In the web font generator, first upload the
file you wish to modify. Then choose the **Expert** radio button. Most of the
settings you can leave as they are; they're good sane defaults. Midway down the
page you'll see **OpenType Flattening** options. Here, select only "Small Caps".
Run the generator. The result will be a complete replacement of the normal
lowercase letters with the small caps set.[^3]

[generator]: http://www.fontsquirrel.com/tools/webfont-generator

In that case, you can simply apply a style to the elements you want to have
small capitals, e.g.:

```css
.divine-name {
    font-family: 'my_typeface_smcp', 'my_typeface', serif;
}
```

The major advantage to this approach is consistency: that typeface is going to
display on every browser out there, back to IE5.5, as long as you deliver it
correctly using the various hooks required by `@font-face`.

There are a few disadvantages to this approach, though:

 1. It means delivering another font file. In my case, this would be an
    acceeptably low size (since I actually only need four characters), but it's
    still something to consider in general. It is in any case another HTTP
    request, which is going to further slow the page load time or at least give
    you some flash of unstyled text when it reloads.

 2. It may violate the licenses of the typefaces in question. For at least one
    of the fonts I used on this project, it *does*: the license explicitly
    forbids rebuilding the font using tools like FontSquirrel. (FontSquirrel was
    the tool I used for this approach before, and it works quite well.) This is
    a make-or-break issue for using a subset of a typeface to accomplish the
    goal. That being said, if you have a good reason to do it, you may be able
    to get support from the vendor (especially if they're a small shop). For the
    project that prompted this question, I was able to do just that with a nice
    email—the designer is a great guy.

The other major reason not to do it this way is that it has a significantly
higher maintenance cost. If at any point you need to change or update the
typeface, you have to go through the subsetting process all over again. By
contrast, the first option will simply *work*, though admittedly not as
pleasantly as one might hope, and will not only continue to work but will
actually improve over time as browsers increase their implementation of the CSS3
standard.


Conclusion
----------

I opted for the second solution on HolyBible.com---typography was one of the
driving differentiators for the site, so I prioritized it and did the necessary
legwork for it. In general, though, the first option should work well for most
sites. In any case, both ways work, though the first one is a *better* example
of progressive enhancement. And we can all look forward to the day when true
small-caps support is available on every browser, right?



[^1]: For various reasons (especially see note 2 below), I actually opted for
    the second approach outlined here, which is the same approach I was trying
    to avoid. Alas.

[^2]: Issues remain: even in the latest Chrome (46 as of the time of this post),
    using the `font-feature-settings: 'smcp'` approach has some issues. For
    example, if you turn on `letter-spacing` (a fairly common
    [recommendation][butterick] for small caps), the small caps will revert to
    normal lowercase letters.

[butterick]: http://practicaltypography.com/letterspacing.html

[^3]: From the FontSquirrel blog post that introduced the feature:
    
    > If you have a font with OpenType features, you can now flatten some of
    > them into your webfont. For instance, some fonts have small caps built in,
    > but they are completely inaccessible in a web browser. By selecting the
    > "Small Cap" option, the Generator will replace all the lowercase glyphs
    > with the small cap variants, giving you a small cap font. Please note that
    > not all OpenType features are supported and if the font lacks OpenType
    > features, using these options won't create them.
