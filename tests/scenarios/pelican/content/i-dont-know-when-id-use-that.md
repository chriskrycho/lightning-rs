---
Title: “I Don't Know When I'd Use That”
Subtitle: Learning new languages as a way of stretching your brain.
Date: 2016-01-17 10:00
Tags: software development, programming languages, rust, haskell
...

I was reading an interesting Stack Overflow [discussion] of the value of
[higher-kinded types][HKTs] (hereafter <abbr>HKTs</abbr>), and noted someone
repeatedly commenting, "But when would you use this in a *real app*?" To put it
the way another [blog post] about <abbr>HKTs</abbr> (in Rust), they are "a
feature people do not really know what to do with."

[discussion]: http://stackoverflow.com/questions/21170493/when-are-higher-kinded-types-useful
[HKTs]: http://stackoverflow.com/questions/6246719/what-is-a-higher-kinded-type-in-scala
[blog post]: https://m4rw3r.github.io/rust-and-monad-trait/

Don't get me wrong: I'm sympathetic to that desire for concrete examples. I'm
interested in these kinds of things not primarily for their intellectual value
but for their pragmatic value (though I don't think those two are as distinct as
many people do). I'd *also* love to see some more real-world examples in those
discussions. All too often, the discussions of types in Haskell end up being
quite abstract and academic---no surprise, given the language's origin. But I'm
also aware that quite often it's difficult to see how a given kind of
abstraction is useful without jumping into a language which has that abstraction
available and *using* it.

People often get turned off by Haskell (and other similarly high-abstraction
languages like Scala) because of challenging terms like *monad*, *applicative*,
*functor*, and so on. And again: I get that. To grok Haskell, you need to wrap
your head around a lot of *math* ideas---mainly various properties of *sets*.

But I remember feeling the same way six years ago when I started playing with
JavaScript and jQuery and every tutorial out there simply assumed existing
familiarity and comfort with functions as arguments or return values. Coming
from the world of Fortran and C, my head ached for weeks as I tried to make
sense of what I was seeing. Even when I finally got it, *I didn't like it*. Over
the last several years, though, I've become increasingly comfortable and even
reliant on closures, composition of functions to transform data, and so on as I
worked regularly in Python and JavaScript.

That experience has taught me that my current inability to see the utility of a
given abstraction means little about the abstraction. It's primarily an
indicator of my own inexperience.

To the question of the utility <abbr>HKTs</abbr> in general---in Haskell, Rust,
or somewhere else---I don't have the knowledge myself (yet) to supply a good
answer. Heck, I can't even *explain* them very well. ([Other people can,
though!][scala]) But I can say that reading [_Maybe Haskell_] showed me clearly
that such things can be very useful. Even if I am not yet comfortable using that
tool, I see how learning to use it would be profitable in the long-term. And
like any good tool, even if you don't need it every day... when you want it, you
*really* want it.

[scala]: http://adriaanm.github.io/research/2010/10/06/new-in-scala-2.8-type-constructor-inference/
[_Maybe Haskell_]: https://gumroad.com/l/maybe-haskell
