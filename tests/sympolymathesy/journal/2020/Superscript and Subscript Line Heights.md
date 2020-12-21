---
title: Superscript and Subscript Line Heights
subtitle: A tip for better typography.
date: 2020-04-08T20:15:00-0600
updated: 2020-04-08T21:55:00-0600
summary: >
    Superscripts and subscripts should not affect the vertical rhythm of the text.
qualifiers:
    audience: People who want their websites, apps, etc. to look right.
tags:
    - typography
    - design
    - web design

---

Superscripts and subcripts should not affect the vertical rhythm of the text. That is, they should not affect its [<i>leading</i>][leading]. As a prime example common in blogs and similar websites: the superscript style used for footnotes should not make lines with footnotes taller than lines *without* footnotes.

[leading]: https://en.wikipedia.org/wiki/Leading

:::note

Two comments before I dive into the rest of the piece:

- Folks reading along in RSS, you may want to [click through](https://v5.chriskrycho.com/journal/superscript-and-subscript-line-heights/): it’s possible these examples will not display correctly in your reader!
- Everything I say here about how text should *look* applies equally to native apps or to print, but the mechanics will be wildly different than they are on the web.

:::

Here’s an example of how the default browser rendering will handle superscripts (it has the same effect for subscripts, but since superscripts are much more common, I’m focusing on those):

:::callout

This is just some running text which includes a superscript. It doesn’t say anything meaningful, it is just designed to run long enough that in every layout,<sup style="line-height: var(--line-height) !important; font-family: var(--serif) !important; font-size: smaller !important; font-weight: 300;">1</sup> the superscript appears in the middle of the running text, so that the way that the offset is visible is clear.

:::

Notice that the line with the superscript is pushed down and away from the preceding line—not *dramatically*, but *noticeably*.

Now, here’s the same layout, using a *corrected* style (with no other tweaks to the font):

:::callout

This is just some running text which includes a superscript. It doesn’t say anything meaningful, it is just designed to run long enough that in every layout,<sup style="line-height:0 !important; font-family: var(--serif) !important; font-size: smaller !important; font-weight: 300;">1</sup> the superscript appears in the middle of the running text, so that the way that the offset is visible is clear.

:::

This still doesn’t quite look *right*—the superscript nearly crashes into the line above it—but the lines are all the same height now, which is an important first step. You can accomplish this consistently (no matter what else is happening in your styles) by setting the [`line-height` property][lh] to `0` in your [CSS] declarations for `sub` and `sup`, so that they don’t have any effect on the layout of each line:

```css
sub, sup {
  line-height: 0;
}
```

However, as we saw above, we need to combine that with other tweaks to make things look *just right*—and the smaller the value of the `line-height` for your running text, the more you have to take care here to keep superscripts and subscripts from colliding with preceding or following lines respectively. For best results, tweak the combination of the size of the item via `font-size` and its layout relative to the other text with `vertical-align`.

On this site, I have also swapped out font family to make them a little more visually distinctive. Here’s how `sup` items are styled today on this site (with some comments added):

```css
sup {
  /* the first rule: don't let it affect line-height */
  line-height: 0;

  /* decreases the size, using the site's typographical scale */
  font-size: ms(-2);

  /* tweak the vertical position so it looks just so */
  vertical-align: 0.558em;

  /* visually distinguish via contrasting font and weight */
  font-family: var(--sans);
  font-weight: bold;

  /* Make sure *not* to use "ordinal" numbers */
  font-variant-numeric: lining-nums;
}
```

And here’s the resulting effect:

:::callout

This is just some running text which includes a superscript. It doesn’t say anything meaningful, it is just designed to run long enough that in every layout,<sup>1</sup> the superscript appears in the middle of the running text, so that the way that the offset is visible is clear.

:::

Much improved!

[CSS]: https://developer.mozilla.org/en-US/docs/Web/CSS 
[lh]: https://developer.mozilla.org/en-US/docs/Web/CSS/line-height

*[RSS]: really simple syndication
*[CSS]: cascading style sheets