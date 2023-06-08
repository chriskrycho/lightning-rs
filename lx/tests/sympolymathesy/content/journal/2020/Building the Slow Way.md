---
title: Building the Slow Way
subtitle: >
    Or, *part* of why <b><i>re</i>write</b> is taking a while: I’m in this for the long haul.
date: 2020-06-07T20:50:00-0600
updated: 2020-06-13T16:20:00-0600
qualifiers:
    epistemic: Talking myself through this, to be perfectly honest.
    audience: >
        People interested in my writing app project, in software development in general, and in “winning slowly” (as it were).
tags:
    - Elm
    - rewrite
    - Ember
    - TypeScript
    - JavaScript
    - software development

---

I spent a little time this weekend working on getting the <abbr>URL</abbr>s for the web app version of [<b><i>re</i>write</b>](https://rewrite.software) working. I’m about 80% of the way done with it. For the last bit, I’ll be asking a bunch of questions of folks more experienced than me with [Elm](https://elm-lang.org): I can see ways to solve a particular challenge, but I don’t *like* any of them. This is just part of the normal process of learning a new technology: things go a bit more slowly because you don’t *already* know what you’re doing.

I had the thought today—not for the first time—that perhaps I should drop Elm and switch to just doing [Ember](https://emberjs.com) and [TypeScript](https://typescriptlang.org): technologies with which I’m *deeply* familiar and *very* competent. I could undoubtedly get the web app up and working faster that way, and I don’t have any particular interest in needlessly reinventing the wheel. As I mulled on the question on my run earlier, though, I came back to the same decision I have every time I have thought on this.

I’m building this thing in Elm on the web and with SwiftUI on iOS and macOS, even though it is *definitely* slower to get to market than just shipping an Ember-TypeScript-Electron app, because speed-to-market is not really a primary concern for me. I have the luxury of working on this on the side, and I want the result of my efforts in building it to be a code base that will last a decade or two. More than that: a code base I will be glad to work in for a decade or two.

The net is that I am choosing to go slower now in order to be able to build more, better, faster, more stably later. I know just how great Ember and TypeScript can be, but I also know that as good as they are, they’re both fundamentally limited by JavaScript itself. As I [wrote](https://v4.chriskrycho.com/2018/javascript-is-c.html "JavaScript is C") in late 2018, I firmly and deeply believe that we can do *better* than JavaScript. With <b><i>re</i>write</b>, I am putting my money where my mouth is. If I’m wrong, I can change course later. But if I’m right, doing this the slightly-slower way now will yield *enormous* dividends over the next fifteen years.

When your timescale is not 6 months of runway but *as long as it takes to make it good* there are downsides: it can lead to paralysis, or the endless pursuit of unattainable perfection. But it also has the very great upside of being able to build something *well*. We don’t build highway bridges overnight, because they have to endure. Too often, we *try* to build software overnight, without considering how it, too, may have to endure.
