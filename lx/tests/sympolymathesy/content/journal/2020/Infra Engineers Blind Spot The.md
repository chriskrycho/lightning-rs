---
title: The Infra Engineer’s Blind Spot
subtitle: Why I ended up down a rabbit hole instead of shipping.
summary: >
    I spent two months trying to set up the best build pipeline I could for my side project, instead of just building features. Why?
date: 2020-04-29T07:00:00-0600
tags:
    - software development
    - product development
    - productivity
    - CSS
    - SCSS
    - Elm
    - TypeScript
    - Rust
    - WebAssembly
qualifiers:
    audience: >
        Anyone who is interested in my side project <b><i>re</i>write</b>, who cares about product development in general, or who just wants a peek inside the strange world that is my mind.

---

Back in early February, I needed to set up the CSS tooling I wanted to use for building the web version of [<b><i>re</i>write</b>][rewrite]. Initially, this was just a matter of poking around looking at the options in the Elm space. I have my old standby of [SCSS], which I like quite well overall and am very comfortable with, having used it for the better part of a decade now. I was curious, though, if the alternative approaches the Elm community has explored were worth my time, so I started poking at them.

[rewrite]: https://rewrite.software
[SCSS]: https://sass-lang.com

*[SCSS]: Sassy CSS

I should have set myself a bounded time limit by which I needed to have made a decision and *shipped some CSS*. I did not. Reader, things went badly: Researching interesting technologies is fun! Researching interesting technologies is also *not shipping*. Researching interesting technologies might pay off if I can optimize my workflow on the app! *Researching interesting technologies is also **not shipping**.*

I *did* eventually conclude that I would be using SCSS—so far so good, though it took me about three weeks longer than it should have. But then things got worse. I started thinking about how I wanted to set up my build pipeline for this project, and started digging into whether I wanted or needed [webpack], and particularly whether I could do something with webpack that would make for a nice single build command that would spit out compiled Elm, CSS, and any TS glue code I needed to write for any [ports] for my Elm code. That led me down the rabbit hole of [generating TypeScript types for the Elm/TS interop story][elm-TS]. And *then* I ended up thinking: <i>Hey, I’m planning to use Rust for my core business logic, and I could set up a Rust-WebAssembly-TypeScript pipeline to support that; there’s even a feature where that would be useful as part of building out this early part of the app that I’m working on…</i>

[webpack]: https://webpack.js.org
[ports]: https://guide.elm-lang.org/interop/ports.html
[elm-TS]: https://github.com/dillonkearns/elm-typescript-interop

*[CSS]: Cascading Style Sheets
*[TS]: TypeScript

I looked up from my work. It was mid-April. Fully two months had passed and in the little bit of time I *had* spent on my side project, I had made a tiny bit of progress on the actual task I had started out trying to accomplish, and mostly tried boiling the ocean of setting up an end-to-end build pipeline for things I may eventually need but certainly do not need today. At the end of it I concluded that I can just add some simple Node scripts which do the build commands for SCSS and Elm. Those are the only ones I actually *need* right now. When I add more later, I can do the same for them.

It took me a little bit to figure out why this happened, and why even now as I sit here writing this post and chuckling to myself at how I got so far afield—really! A Rust-WebAssembly-TypeScript-Elm pipeline for a feature (that I should be doing server-side anyway when I do it)!—I still feel the urge to go build the perfect one-and-done webpack configuration. The answer is simple, even if it wasn’t obvious:

I keep getting derailed on my side project *product work* by things which are what my *day job* actually requires me to do. At LinkedIn, I’m an infrastructure engineer. My whole job is to improve developer productivity, to make the fundamentals and guts of our app better in ways that make it easier for our product developers to ship code. That means working on upgrades, tweaking our builds, investing in tooling, you name it. But when I’m doing product work on <b><i>re</i>write</b>, my job is to *ship the product*. When it comes to infrastructural work, I should do the *minimum* I can do to be productive and effective in making forward progress at any given point. In my day job, I’m always optimizing, always clearing out paths for people, always identifying problems and pain points and trying to minimize them. In this side project, I need to optimize a lot less: if the path is walkable, *walk it*.

