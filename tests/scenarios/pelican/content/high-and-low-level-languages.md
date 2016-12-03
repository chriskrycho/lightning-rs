---
Title: High- and Low-Level Programming Languages
Subtitle: Python, Ruby, C, C++, and... Rust. Hmm.
Date: 2015-08-07 20:00
Tags: programming languages, rust, python, ruby, c, cplusplus
...

It occurred to me while listening to [Edwin Brady] talk about [Idris] on the
[Type Theory Podcast],[^1] having just spent a few weeks starting to learn
[Rust]: "low-level" has at least two meanings in software. One is whether
something has manual memory management or is garbage collected, reference
counted, or otherwise manages memory itself. This is what people often mean when
they talk about C, C++, etc. as being "low-level" and languages like Python or
Ruby or C♯ being high-level.

[Edwin Brady]: https://edwinb.wordpress.com
[Idris]: http://www.idris-lang.org
[Type Theory Podcast]: http://typetheorypodcast.com
[Rust]: https://www.rust-lang.org

But then you toss in a language like [Rust], and things start to get a little
more complicated. Rust can do the same kind of direct memory management that
makes C or C++ a good language for things like writing operating system kernels.
[[1][1],[2][2],[3][3]] But it is also memory-safe, at least in ordinary usage.
Like C♯, you have to be explicit about any unsafe code, with the `unsafe`
keyword on any blocks that do memory management that isn't safe. And the vast
majority of Rust code *is* safe.

[1]: https://github.com/torvalds/linux
[2]: https://en.wikipedia.org/wiki/Architecture_of_Windows_NT
[3]: http://www.opensource.apple.com/source/xnu/xnu-2782.10.72/

More than that, though, Rust *feels* like a high-level language. It gives you
higher-kinded functions, generics, traits-based composition of types, hygienic
macros, and the implementation of many important parts the essentials of the
language in the library. If you need to patch something, or extend something,
you can do that in a straightforward way. In short, it gives you lots of good
abstractions like you would expect in a high-level language.

Rust is low-level in that you can write (and people are writing) systems-level
programs in it. It is high-level in that it lets you express things in ways
normally associated with languages like Haskell or OCaml or Python or Ruby. To
put it simply: it's *low-level* in its ability to address the computer, and
*high-level* in the abstractions it hands to a programmer. That's a powerful
combination, and I hope more languages embrace it in the years to come.


[^1]: Yes, I know that's insanely nerdy. What did you expect?
