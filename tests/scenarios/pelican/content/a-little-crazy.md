---
Title: A Little Crazy
Author: Chris Krycho
Date: 2014-04-29 19:30
Summary: >
    I am going to write a static site generator in Io. Oh, and also the Markdown
    parser and HTML templating engine required to go with it.
Tags: software development
...

I'm going to do something a little crazy, I've decided. I'm going to go ahead
and do like I wrote [a bit back][do-it-myself], and make [Step Stool][ss]
actually a thing over the course of the rest of the year. Not so crazy. What is
a bit nuts is the way I've decided to go about that process. In short: as close
to the hardest way possible as I can conceive.

---

Over the last couple weeks, I've been spending a fair bit of time toying with
[Io][io]. It's a neat little language, very different in its approach to a *lot*
of things than the languages I've used previously. My programming language
history is very focused on the "normal" languages. The vast majority of real-
world code I've written has been in one of C, PHP, or Python. I've done a good
bit of Javascript along the way, more Fortran than anyone my age has any
business having done, and a little each of Java and Ruby. Like I said: the
normal ones. With the exception of Javascript, all of those are either standard
imperative, object-oriented, or mixed imperative and object-oriented languages.
Python and Ruby both let you mix in a fair bit of functional-style programming,
and Javascript does a *lot* of that and tosses in prototypal inheritance to
boot.

But still: they're all pretty mainstream, "normal" languages. Io isn't like that
at all. For one thing, it's hardly popular in any sense at all. Well-known among
the hackers[^1] I know, perhaps, but not popular by any measure. It's small. And
it's very *alien* in some ways. It's [prototypal inheritance][proto], not normal
inheritance. Courtesy of [Javascript][js-proto], I have a *little* familiarity
with that, but it's definitely still not my default way of thinking about
inheritance. Python's inheritance model (the one I use most frequently) is
*essentially* the same as that in C++, Java, PHP, and so on---it's normal
class-driven inheritance. Io goes off and does full-blown prototypal
inheritance; even just the little I've played with it has been fun.

Io also does a bunch of other things a *lot* different from the other languages
I've used. First, there are no keywords or---formally speaking---even operators
in the language. Every action (including ones like `+` or `for`) is simply a
message. Every value is an object (so `1.0` is just as fully an object as an
arbitrarily-defined `Person`). The combination means that writing `1 + 2` is
actually just interpreted as the object `1` receiving the `+` message carrying
as its "argument" the `2` object (really just the message contents). This is
*completely* different at a deep paradigm level from the normal object-oriented
approach with object methods, even in a language like Python where all elements
are objects (including functions). The net result isn't necessarily particularly
different from calling methods on objects, but it is a *little* different, with
have some interesting consequences. Notably (though trivially---or at least, so
it seems to me at this point), you can pass a message to the null object without
it being an error. More importantly, the paradigm shift is illuminating.

Io also has far more capabilities in terms of concurrency than any of the other
languagues with which I'm familiar, because it actively implements the [Actor
Model][actor], which means its implementation of messaging instead of object
method calls can behave in concurrent ways. (I'd say more if I understood it
better. I don't yet, which is one of the reasons I want to study the language.
Concurrency is very powerful, but it's also fairly foreign to me.) It's also
like Lisp in that its code can be inspected and modified at runtime. I've wanted
to learn a Lisp for several years for this kind of mental challenge, but the
syntax has always just annoyed me too much ever to get there. Io will give me a
lot of its benefits with a much more pleasant syntax. It has coroutines, which
are new to me, and also helpful for concurrency.[^2]

The long and short of it is that the language has a ton of features not present
in the languages I have used, and---more importantly---is *paradigmatically*
different from them. Just getting familiar with it by writing a goodly amount
of code in it would be a good way to learn in practice a bunch of computer science concepts
I never had a chance to learn formally.[^3]

---

By now, as long as I've rambled about Io, you've probably figured out where I
was going in that first paragraph. I've decided to stretch my brain a bit and
write Step Stool in Io. There are bunches of static site generators out there in
Python already, many of them quite mature. (This site is running on [one of
them][pelican] as of the time I write this post---it's quite solid, even its
quirks and limitations occasionally annoy me.) The point of Step Stool has
always been twofold, though. First, I've wanted to get to a spot where I was
really running my own software to manage my site, letting me do whatever I want
with it and guaranteeing I always understand it well enough to make those kinds
of changes. Second, I've just wanted to *learn* a whole bunch along the way.
Third, it's right there in the website link: [step-stool.io][ss]! How could I pass
up such an opportunity?

It is that second goal that has pushed me to do this crazy project this crazy
way. It's crazier than just teaching myself a language in order to do the
static site generator itself, too, because there are a few other pieces missing
that I'll need to write to make this work... like a Markdown implementation and
an HTML templating language. I've never written anything remotely like either
before, so I'm going to take the chance to learn a *lot* of new things. For the
Markdown implementation, rather than relying on regular expression parsing
(like most Markdowns do), I'm going to use a Parsing Expression Grammar.
That will certainly be more efficient and reliable, but---more importantly---it
is also outside my experience. I have yet to start thinking through how to
tackle the HTML templating language implementation (though I know I am going to
make it an Io implementation of [Slim][slim], which I quite like).

In any case, I'm going to be taking a good bit longer to get Step Stool
finished. That is all right: I am going to learn a ton along the way, and I am
quite sure I will have a blast doing it. And that is *exactly* what these kinds
of projects are for.

I'll post updates as I go, with the things I'm learning along the way.
Hopefully they'll be interesting (or at least entertaining).

[^1]: Hackers in the original sense of the world. Not "crackers", but people
    who like hacking on code, figuring things out the hard way.

[^2]: Python 3.5 is actually adding coroutines, and I'm excited about that.
    I'll feel much more comfortable with them there having used them in Io, I'm
    sure!

[^3]: I got here backwards, as it were---by way of an undergraduate degree in
    physics. I don't regret that for a second: I got a much broader education
    than I could have managed while getting an engineering degree, and most
    importantly learned *how to learn*: easily the most important skill anyone
    gains from any engineering degree.

[actor]: http://en.wikipedia.org/wiki/Actor_model
[do-it-myself]: http://www.chriskrycho.com/2014/doing-it-myself.html
[io]: http://iolanguage.org
[pelican]: https://github.com/getpelican
[proto]: http://en.wikipedia.org/wiki/Prototype-based_programming
[js-proto]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Inheritance_and_the_prototype_chain
[slim]: http://slim-lang.com
[ss]: http://step-stool.io
