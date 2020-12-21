---
title: Scripting Affiliate Links in Alfred
subtitle: Perhaps weirdly, I haven’t done much of this before!
date: 2020-02-15T21:50:00-0600
updated: 2020-02-15T22:02:00-0600
tags:
    - Alfred
    - scripting
    - writing
    - programming
    - scripting
    - automation
summary: >
    I built an Alfred workflow to generate Alibris affiliate links tonight. It's convenient, and I learned a bit because this is new to me!
qualifiers:
    audience: >
        Others who might be interested in making their lives easier by automating repetitive (and meaningless) tasks that are a regular part of their work.
---

This evening, I found myself manually using a really terrible affiliate-link-builder UI to generate links to Alibris for [the latest issue of my newsletter][atss], and decided it was time to take the plunge and actually start automating some of these things so I could save time when doing it in the future.

<aside>

Weirdly, despite being a programmer by trade, I have not really spent a lot of time on this kind of personal automation in the past. I got into programming by way of visual design—HTML and CSS—*not* by way of either games or automation like so many of my peers. The majority of the automation I’ve done has been on iOS/iPadOS, and those mostly only as workarounds for things that are more difficult than they should be!

</aside>

The result of half an hour of messing with this was a *super* simple [Alfred] workflow. I can just launch Alfred (by typing <kbd>⌘</kbd><kbd>⌥</kbd><kbd>Space</kbd>), type `aalink`, and paste a URL, and then immediately hit <kbd>⌘</kbd><kbd>V</kbd> in whatever writing app I’m using—and boom, the link is as it should be!

Under the hood, Alfred takes in that given URL, encodes it, attaches the relevant parameters for the deep links, and copies the result to the clipboard so that I can immediately paste it into:

![Alfred workflow for generating affiliate links](https://cdn.chriskrycho.com/file/chriskrycho-com/images/alfred-affiliate-workflow.png)

I had a basic version of this tiny little workflow done in about five minutes. The rest of that half hour I spent figuring out how to make it *shareable for this blog post*. (You can snag it [here][workflow]!) Honestly, having taken 30 minutes to build this and then get it to the point where I could actually export it to share it with you will has likely put me in [XKCD #1319] territory for the foreseeable… but I think I’m actually okay with that even if so, because I now have another tool in my toolbox for this kind of thing in the future. That tool actually made itself useful immediately: I realized that I *also* have a small but *very* repetitive task for updating the URL for every item I put in my CDN on [Backblaze B2][B2] to point to `cdn.chriskrycho.com` instead of the Backblaze-specific URL. I did *that* in a matter of a *minute*.

I don’t know if I’ll ever be one of those people who spends a *lot* of time automating things, but I *am* genuinely glad to have set this up and to have put the tool in my toolbox!

…now to get back to drafting the rest of the newsletter that sent me down this rabbit hole in the first place.

[atss]: https://buttondown.email/chriskrycho/archive/revision-is-a-permanent-state-of-affairs-across/
[Alfred]: https://www.alfredapp.com
[workflow]: https://cdn.chriskrycho.com/file/chriskrycho-com/workflows/Alibris%20Affiliate%20Link%20builder.alfredworkflow
[XKCD #1319]: https://www.xkcd.com/1319/
[B2]: https://www.backblaze.com/b2/cloud-storage.html

*[UI]: user interface
*[HTML]: hyper-text markup language
*[CSS]: cascading style sheets
*[URL]: universal resource locator
*[CDN]: content delivery network