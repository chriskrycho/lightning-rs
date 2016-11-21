---
Title: Language Design
Tags: programming languages, objective-c, swift
Category: tech
Date: 2016-06-20 07:00 
Status: draft
---

Over the past few months, I've watched a number of long-time Mac and iOS developers discussing the ongoing transition in Apple's platforms from the old, Objective C, to the new, Swift. And... frankly, it's a weird conversation to watch.

I come at this as a relative outsider, but one with a substantial personal interest in the platform: I use it day in and day out, and I have a fairly large project I'm in the planning phases on which I hope to ship on both Mac and iOS in the next three or four years. And everything which follows should be taken with that as a huge grain of salt. These developers have many times my experience writings apps---every one of them, much less as a group.

But I still think a lot of the things they're throwing out as "concerns" and "problems" for Swift over Objective C just... aren't.

In the last seven years, I've non-trivial written software professionally in C, C++, assembly, Fortran, PHP, Python, and JavaScript. I've dabbled to greater or lesser degrees with Java, Lisp, OCaml, Ruby, Io, Haskell, F♯, CoffeeScript, Swift, and an awful lot of Rust. Software I've written has powered web applications and flown on aircraft. It's been written against the waterfalliest of processes on the one hand, and come through my own agile-as-can-be, TDD approach as a one-man consulting shop. So while those guys have a lot of experience... it's actually possible I've seen a wider set of *kinds* of programming languages than some of them. That has upsides and downsides. The big downside is that I don't have the sheer depth of knowledge in any one of those languages that others do. The big upside is that I am fairly familiar with the tradeoffs that come with strong and weak typing, and with dynamic and static typing, and with imperative, object-oriented, and functional programming. I can (and hopefully will) be far, *far* more familiar with those tradeoffs than I am now.

But when I see smart developers poo-pooing type safety, functional programming idioms, and so on; when I see developers thinking you can't have things calculated dynamically at runtime because the type system is strong; when in short I see developers conflate *what they're used to* with *how things need to be done*... it makes me sad.

The software we ship is eating the world. If we can find ways to improve our tools and processes so that the software is less buggy, while coming out even or even ahead in terms of productivity, *we should*. And Swift is that kind of advancement over Objective C, full stop.

Yes, there are things you can do in Objective C that you can't do in Swift. But I have yet to hear an example of such a thing which isn't a serious source of bugs. Sure, you can monkey-patch/swizzle methods on an existing class. And yeah, you can use that to patch your way around a given bug in a framework or a library. But at what cost? Ask the Ruby on Rails community what happens when anyone anywhere can monkey-patch core functionality. (Or ask yourself, if you work with OS X or iOS.) Type safety has a cost, of course, but it also comes with benefits. And those benefits are much higher in a language like Swift than in a language like Java (especially the Java of a decade ago, but that of today as well). Good type inference gets you a lot of the productivity you're used to with dynamic types, and generics get you a lot of the rest. But you also get some guarantees at *compile* time that things won't break. You get to get deterministic errors at build time instead of trying to chase down some hard-to-identify bug arising from a type *almost* but *not quite* doing what you need it to.

Look: there are things to quibble with in Swift's design. [I have done just that, at length.][rust-and-swift]

[rust-and-swift]: http://www.chriskrycho.com/rust-and-swift.html
