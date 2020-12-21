---
title: Ember Octane is a New Mental Model
subtitle: >
  …not a 1:1 translation from Ember Classic—but that’s often a big win!
summary: >
  Don’t expect a 1:1 translation from Ember Classic to Ember Octane. Do expect that things might get better because there is no 1:1 translation.
image: https://cdn.chriskrycho.com/file/chriskrycho-com/images/ember-octane.png

date: 2020-12-07T10:15:00-0700
updated: 2020-12-07T13:20:00-0700

qualifiers:
  audience: >
    Software developers working with Ember Octane; also anyone interested in software design tradeoffs and large migrations in general.

templateEngineOverride: md

tags:
  - Ember
  - JavaScript
  - mental models

---

:::note

Over the weekend I wrote up the following on [an Ember RFC discussion](https://github.com/emberjs/rfcs/pull/669#issuecomment-739320902). Since it’s intentionally *not* specific to that RFC, I’m republishing it here for the broader Ember community to see, with only the slightest tweak to the intro to give more context. (It might be a bit less polished than usual, as a result!)

:::

For what it’s worth, as the Octane migration lead for LinkedIn.com, I see these kinds of issues—existing designs that just don’t translate directly from Classic to Octane—all the time as folks migrate from Ember Classic patterns—and precisely *because* of that experience, I strongly agree with [@pzuraq] and others on the framework team that we don’t want to provide primitives which let you “go around” the autotracking system’s design and primitives. Accordingly, and without digging into that specific example, I’d like to offer a general comment here in hopes that it’ll be useful more generally for the community thinking around migrating to Octane.

[@pzuraq]: https://www.pzuraq.com

Fundamentally, Octane entails a new programming model—one that is *not* a direct translation of the old model into some new syntax. That means that some patterns that you’re used to using in the Classic paradigm simply do not work in the new paradigm (though not very many). However, in literally every single one of those I’ve hit over the last year, I’ve ultimately been very happy to end up doing the work of substantially rethinking and reworking whatever abstraction or implementation was at hand.

In practice, that has often meant a process something like this:

1. Take a big step back and consciously choose to set aside the existing implementation’s constraints as inputs.
2. Write down the actual constraints on the design: the problem it needs to solve, along with any *hard* constraints on the API design.
3. Create a *new* Octane design that solves the problem.
4. If necessary, because the old design was fundamentally coupled to Ember Classic idioms, design a migration path to the new API.

(4) has been rare, but has occasionally happened. For example, over the summer we took a system that made heavy use of `Evented` and reworked it into a new approach that funneled each event into a single piece of tracked data, and let the rest of the system derive from that single source of truth. This isn’t something that we could do under the existing API, because the existing API was just *everyone using the events willy-nilly*. (The famous reason React exists—keeping message state in sync—hit our app in a *lot* of places as a result! And now… it doesn’t.) Migrating all consumers to the new design was and is a slow process, but everywhere we do it, we’re *really* happy we did.

The single most common example of this kind of rewrite for us is figuring out how to replace uses of `didReceiveAttrs`. The easy, “translation”-style path is to grab `{{did-insert}}` and `{{did-update}}` and keep the implementation otherwise the same. And for the short term, this is sometimes reasonable. However, in every case we’ve hit, there was also a much better solution that involved rewriting in terms of derived state, using a custom modifier for managing DOM interactions, or a combination of the two. And when we make that investment, the result has consistently required *less code* that was *easier to understand and test* to solve the same problem than the original code or even a “translation” from Classic to Octane.

This process is both harder and slower than just trying to translate directly from Ember Classic into Ember Octane syntactically, to be sure! It has also paid off handsomely, though.

All of this to say: it’s very common to find your existing <abbr title="application programming interface">API</abbr>s and solutions to a problem aren’t Octane-friendly. One response is to say this is a problem with Octane. Depending on your views on software design, that might be fair! Another, though, is to say that it’s possible the abstraction would benefit from being reworked substantially. This is my POV, and again: every single place we’ve hit like this in our app has been *improved* by the rewrite—often massively so.

That doesn’t mean Octane is perfect. For one, there are still gaps: this RFC addresses one, and the work to get resources and effects addresses another. For another, observable-based systems (like Ember Classic) and incremental computation systems (like Ember Octane) simply have different tradeoffs and affordances—and you might prefer the former! But the fact that a pattern that worked in Classic doesn’t work in Octane doesn’t mean that Octane is *wrong* or even that it is *missing* something. It does mean that it makes fundamentally different tradeoffs in the design space than Classic did.

Me, I prefer the Octane flavor of the tradeoffs, and when you hit spots like this I encourage you to rethink the design. You may find yourself in a place that surprises you with how much cleaner and more maintainable it is! You may also occasionally find a spot where the tradeoffs are a little worse (hasn’t happened to me yet, but I’m sure it’s possible). But either way, the fundamental design is the way it is—and breaking the core abstraction would be much worse.

*[RFC]: request for comments
*[POV]: point of view
*[API]: application programming interface
