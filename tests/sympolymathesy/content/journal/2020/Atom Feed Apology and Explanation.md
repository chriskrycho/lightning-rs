---
title: An Atom Feed Apology
subtitle: >
    Doubly embarrassing for having now messed up *both* of my feeds.
qualifiers:
    audience: >
        people who are annoyed that all my feed items just showed up in their readers again. I’m *really* sorry, everyone.
date: 2020-01-11T09:35:00-0600
tags:
    - blogging
    - Atom feed
    - Nunjucks
    - site meta
summary: >
    While fixing another issue with my Atom feed, I discovered that I was rendering bad item IDs. It’s fixed now; sorry it happened.

---

Stop me if [you’ve heard this before][json-feed]: I made a mistake in setting up the [Atom feed][atom] for this site, specifically around the IDs. A bunch of you probably just got a bunch of feed items in your reader again, and this post just exists as a spot for me to say “I’m very sorry!”—because I am. It’s extremely annoying when that happens.

This was just [a dumb mistake][commit] that was easily fixed, but it meant the IDs were fragile. I found it because I realized the titles were rendering badly in the [mailing list][mailchimp] I have set up for this feed. It’s fixed now, and hopefully—*hopefully!*—this is the last time for the life of this implementation that you’ll hear anything from me about these feeds!

[json-feed]: https://v5.chriskrycho.com/journal/json-feed-apology-and-explanation/
[atom]: https://validator.w3.org/feed/docs/atom.html
[commit]: https://github.com/chriskrycho/v5.chriskrycho.com/commit/afec3cfc87de4bbecf8df1c23b59f27f83f586e6
[mailchimp]: https://us7.campaign-archive.com/home/?u=eeb568ecf8f5d9f2a2a1c9f24&id=bccf52306b