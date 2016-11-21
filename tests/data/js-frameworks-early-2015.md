---
Title: Unsurprisingly, In Flux
Subtitle: A Quick Look at the State of JS Frameworks in Early 2015
Summary: >
    The state of JavaScript frameworks today is a scale, really, from
    not-at-all-monolithic to totally-monolithic, in roughly this order:
    Backbone – React & Angular – Ember – Meteor.
Author: Chris Krycho
Date: 2015-04-08 16:05
Modified: 2015-08-28 19:50
Tags: software development, javascript, angularjs, emberjs, react
...

<i class="editorial">This started as a [series of posts] on App.net. I
[resolved] a while ago that if I was tempted to do that, I should just write a
blog post instead. I failed at that resolution, but at a friend's [suggestion],
am adapting it into a blog post anyway. You can see the posts that prompted it
[here][\@keita] and [here][\@jws].</i>

[series of posts]: https://alpha.app.net/chriskrycho/post/57102562
[resolved]: http://www.chriskrycho.com/2014/a-few-theses-on-blogging.html
[suggestion]: https://alpha.app.net/jws/post/57108281
[\@keita]: https://alpha.app.net/keita/post/57096585
[\@jws]: https://alpha.app.net/jws/post/57096838

---

  - The state of JavaScript frameworks today is a scale, really, from
    not-at-all-monolithic to totally-monolithic, in roughly this order: Backbone
    -- React & Angular -- Ember -- Meteor.

  - Backbone and related library Underscore are really collections of common JS
    tools and patterns you can use to write apps, but they're not *frameworks*,
    per se. You'll write all your own boilerplate there.

  - React and Angular supply much *more* of the functionality, but Angular is a
    "meta-framework" that aims to do *some* boilerplate but let you construct
    your own custom app framework.

  - Angular is very powerful, but it's kind of like Git: wires are exposed; you
    have to understand a *lot* about the internals to get it to do what you
    want. Its routing functionality is pretty limited out of the box, too---so
    much so that there's a near-standard third-party router.

  - React, as I understand it, supplies a paradigm and associated tools oriented
    primarily at view state management, though with capabilities via extensions
    for routing, etc. These tools are *extremely* powerful for performance in
    particular. It's not a full framework, and the docs expressly note that you
    can *just* use React for the view layer with other tools if you want.

  - In any case, Angular and React do *different* things from each other, but
    both do substantially more than Backbone.

  - Ember is a full framework, strongly emphasizing shared conventions (with a
    lot of common developers from Rails). It's perhaps less adaptable than React
    or Angular, but is much more full-featured; you have very little boilerplate
    to do.

  - Meteor is like Ember, but does server-side Node as well as client-side
    stuff, with the goal being to minimize code duplication, sharing assets as
    much as possible.

  - Of all of those, Ember has easily (easily!) the best-explained roadmap, most
    articulate leadership, and best development path. They are also aggressively
    adopting the best features of other frameworks wherever it makes sense.

  - Angular is currently in flux, as Google has announced Angular 2.0 will be
    basically a completely different framework; there will be *no* direct
    migration path for Angular 1.x apps to Angular 2.0+. Total rewrite required.

  - Ember uses a steady 6-week release schedule with very careful regression
    testing and semantic versioning, with clear deprecation notices and upgrade
    paths, and is therefore both rapidly iterating *and* relatively stable for
    use.

  - If you just need a set of tools for enhance functionality on otherwise
    relatively static pages, Backbone+Underscore is a great combo. If you
    already have a bunch of things in place but want a dedicated view layer,
    React is good.[^react]

  - If you're writing a new, full-on web *application* (SPA, or organized in
    whatever other way), I think Ember is the very clear winner at this point. I
    have good confidence in their leadership and they're firing on all
    cylinders.

Regarding Angular, [\@mikehoss][\@mikehoss] [posted][posted]:

> For the record they are doing that to make it more mobile-friendly. The Ang1
> has abysmal performance on mobile. Besides a time machine, this maybe the best
> option. And Miško is a bit of a jerk.

[\@mikehoss]: https://alpha.app.net/mikehoss
[posted]: https://alpha.app.net/mikehoss/post/57105656

I can't speak to his comment about Miško (Miško Hevery, one of the leads on
AngularJS), but I agree about Angular itself: the rewrite needs to happen.
Angular 1.x is a mess---as are its docs. It's just not a good time to be using
1.x for any new projects.

I'll add to these points that I've used Angular for the last 9 months on
HolyBible.com development. As I noted: the documentation is pretty rough, and in
a lot of cases you really do have to understand what the framework is doing and
how before you can get it to do the things you want. This is, in one sense,
exactly the *opposite* of what I'm looking for in a framework---but it makes
sense given Angular's goal of being a meta-framework.

Rather like Git, though, which was originally going to be infrastructure for
version control systems which would have their own interface, but eventually
just had a "good enough" interface that we're all now stuck with, Angular is
being used *as* a framework, not just as a *meta-framework*, and it's
unsurprisingly not great for that.

---

<i class="editorial">Take this for what it's worth: not the final word (by a
long stretch) on JavaScript frameworks, but rather the perspective of one guy
who notably _hasn't used all of the frameworks_, but has spent some time looking
at them. Moreover, I haven't particularly edited this; it's more a summary in
the kind of short-form posts that I originally created than a detailed analysis.
The only things I've done are expand some of the notes on Angular and React, and
add the footnote on React.</i>

[^react]: I *really* don't know a ton about React, but I do think a lot of what
    I do know about it is cool from a programming perspective. From a designer
    perspective, however, it's a bit of a pain: React's "JSX" domain-specific
    language is *much* less friendly to developers than standard HTML, and
    therefore than either Ember or Angular, both of which implement their
    templating via HTML templating languages. There's a substantil tradeoff
    there: React's model is interesting not only academically but in practice
    because of the performance results it produces. It's worth note, though,
    that others have recognized this and are adopting it to varying degrees;
    notably, Ember is incorporating the idea of minimizing changes to the DOM by
    keeping track of state and updating only differences, rather than refreshing
    the whole tree, in the new rendering engine (HTMLBars) they're rolling out
    over the past several and future several releases.
