---
title: A JSON Feed Apology and Explanation
subtitle: >
    All my best efforts and this is still where we end up!
date: 2019-11-28T23:15:00-0600
updated: 2019-12-15T09:35:00-0600
tags:
    - TypeScript
    - JavaScript
    - software design
    - blogging
    - JSON Feed
    - site meta
qualifiers:
    audience: >
        people who care about the details of web publishing.

---

To my very great annoyance, I realized today that I managed to ship a broken version of JSON Feed with this version of my site.

*[JSON]: JavaScript Object Notation

For those of you who don’t care about any of the details: it’s fixed now!

For those of you who *do* care about the details, the rest of this post is a deep dive into what went wrong and why, and how I fixed it.

---

The feed was broken because every item had the same value for the `id` field: the URL for the whole site, `https://v5.chriskrycho.com`. The feed items were ending up with this same `id` despite my intent to the contrary because, in this specific place, I called the constructor of Node's `URL` type with the arguments in the wrong order. The function takes a path first, and then the base path for the URL second. In this specific case, I called it with the base path first, and the relative path second.

As a result, every single time the constructor was simply returning the base path… which just happens to be the base path for my whole website. Every JSON Feed item was therefore ending up with that, and *only* that, as as its `id`… which meant it was treating the feed as if it never updated (though it had an increasing number of items in it at the same URL).

This was exactly the opposite of what I wanted, all just because I accidentally inverted the argument order for this invocation. In my defense, however, inverting the argument order of a function which takes two strings as arguments is a pretty easy mistake to make! The fact that it is so easy to get this wrong is, in my opinion, a fairly significant failing of the API’s design. The `URL` constructor just takes two strings as its argument, and the order for the two strings is *not obvious at all*! This arises out of the dynamic nature of the constructor. Quoting Node’s docs for the constructor arguments, where the constructor is `constructor(input[, base])`:

> - `input`: The absolute or relative input URL to parse. If `input` is relative, then `base` is required. If `input` is absolute, the `base` is ignored.
> - `base`: The base URL to resolve against if the `input` is not absolute.

This single constructor is actually an *overloaded* constructor: it does different things depending on the inputs you pass it. However, the first thing you pass it is *just a string*. (The `base` argument can be, but does not have to be, an existing `URL` instance; it too may be just a string.) It’s really, really easy to miss the fact that the function has totally different behavior depending on the contents of that string—that it’ll silently ignore the second argument if the first one happens to be a fully-formed URL itself, for example.

A better API would account for these discrete use cases by separating them out. Instead of having a single constructor which has to handle both of these scenarios, the API could supply two static constructors: `withBase` and `fromAbsolute`:

```ts
class URL {
  static withBase(base: string | URL, relativePath: string): URL;
  static fromAbsolute(path: string): URL;
}
```

This would entirely eliminate the possibility of confusion in building the class instance. When you want a version with a base URL, you just use `withBase`; when you want one to handle absolute paths, you just `fromAbsolute`; if you need a graceful fallback, you can write that yourself, or another static constructor could be supplied. The point here in any case is that you can design the API from the outset not to lead people into these kinds of mistakes.

Now, if you go poking at my site's source, you'll also notice that I didn't call `new URL` directly! The Node type’s constructor function can throw an exception if you give it invalid arguments. In my case, I didn’t want that—instead, I wanted to log errors and just return the path from the root, without the domain, if it didn't work for some reason. That wrapper, named `absoluteUrl`, uses [True Myth]—specifically [its `tryOrElse` function][tryOrElse]—to safely provide a reasonable value for all URLs on the site:

```ts
import { Result } from 'true-myth'
import { URL } from 'url'
import { logErr, toString } from './utils'

const absoluteUrl = (path: string, baseUrl: string): string =>
  Result.tryOrElse(logErr, () => new URL(path, baseUrl))
    .map(toString)
    .unwrapOr(path)
```

This works great! …except that it has the exact same API design problem that the original Node API has. I had an opportunity here, and originally missed it, to make this API more robust by designing it to eliminate one of these failure cases by passing an object constructor instead of just a pair of arguments:

```ts
type Components = {
  path: string,
  baseUrl: string | URL
}

const absoluteUrl = ({ path, baseUrl }: Components): string =>
  // ...
```

In my day job, that's *exactly* what I’d do, in fact. However, there’s a challenge to doing that way here: I use this same `absoluteUrl` function as a template helper… and therefore I need to be able to pass it arguments as regular function arguments, *not* as an object. Alas.

[True Myth]: https://github.com/true-myth/true-myth
[tryOrElse]: https://true-myth.js.org/modules/_result_.html#tryorelse

*[API]: application programming interface
*[URL]: universal resource link
