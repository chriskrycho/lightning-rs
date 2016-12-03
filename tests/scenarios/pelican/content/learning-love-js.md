Title: Growing Up Together
Subtitle: Or: How I Learned to Stop Worrying and Love JavaScript
Author: Chris Krycho
Date: 2014-11-15 00:30
Tags: software development

A few years ago, you might have caught me in a grumpy moment grousing about
JavaScript. I distinctly did *not* like writing it. Every time I sat down to
deal with it, I found myself in a tangled mess of plain JavaScript, jQuery, and
DOM manipulations that inevitably left me tearing my hair out.[^1] I found it
difficult to write in the first place, and even harder to maintain in the long
run. I could not come up with good ways to organize it, especially because so
much of what I was doing was so thoroughly _ad hoc_ in nature. Cobble this
together over here; scrounge together those things over there; hope nothing
collides in the middle.

In the last four months, I have written several thousand lines of JavaScript,
and I have *loved* it.

For my latest major project, relaunching [HolyBible.com][hbc], I wrote the front
end in [AngularJS][ng] and the back end as an [Express][express] app (the most
popular [NodeJS][node] web framework). I've written gobs of tests in
[Jasmine][jasmine] (using [jasmine-node][jn] for server-side tests) and drawn on
tons of other open-source packages.

And I have *loved* it.

A small example: a moment ago, looking up the link for Jasmine, I noted that the
latest version released today. My response was, "Ooh---cool!"[^2]

What changed? Well, mostly I changed, but also JavaScript changed a bit. We both
grew up over the last four years. On the JavaScript side of things, a lot of
good design patterns and tools have come into play in that span. I'm sure there
were plenty of good, disciplined web developers writing clear, careful,
well-organized client-side JavaScript four years go. But in the interval, that
kind of JavaScript got a lot more prominent, in part because it has had help
from the rapid rise of server-side JavaScript in the form of Node.js and its
flourishing ecosystem of components and tools. Build tools like
[Browserify][browserify] and development tools like [LiveReload][lr] and
[Codekit][ck] have combined with best practices learned from those long years of
jQuery/DOM-manipulation hell so that these days, good JavaScript is a lot like
good programming in any other language: highly modular, carefully designed, and
well-organized.

In the same period of time, I have matured enormously as a developer (just
enough to see how far I still have to go, of course). At the point where I most
hated JavaScript, I also really struggled to see the utility of callbacks.
Frankly, it took me the better part of a month just to get my head around
it---most of the tutorials out there just assumed you understood them already,
and, well: I didn't. Functions as first-class members of a language was new to
me at that point. Fast-forward through several years of full-time Python
development, lots of time spent reading about software development and some
harder computer science concepts, and my perspective on JavaScript has shifted
more than a little. Closures are beautiful, wonderful things now. Functions as
arguments to other functions are delightful and extremely expressive. Prototypal
inheritance---trip me up though it sometimes still does---is a fascinating
variation on the idea of inheritance and one that I think I like rather better
than classical inheritance.[^io]

There are still things I don't love about JavaScript. Its syntax owes far too
much to the C family of languages to make me happy; I quite like the way that
CoffeeScript borrows from Python (white-space-delimited blocks, use of equality
words like `is` and boolean rules like `and` rather than `===` and `&&`
respectively, etc.). And I am looking forward to a number of features coming in
the next version of JavaScript---especially generators and the `const` and `let`
keywords, which will allow for *much* saner patterns.

But all of that is simply to say that I am now starting to know JavaScript
enough to know that its *real* issues aren't the surface-level differences from
the other languages with which I'm familiar. They're not even the warts I noted
here. They're things like the mix of classical and prototypal inheritance in the
way the language keywords and object instantiation work. But I don't mind those.
Every language has tradeoffs. Python's support for lambdas is pretty minimal,
despite the utility of anonymous functions, for example. But I *like* the
tradeoffs JavaScript makes.[^coffee]

In other words, I discovered the same thing so many other people have over the
last few years: JavaScript isn't just a good choice for utilitarian reasons.
Beneath that messy exterior is a gem of a language. I'm having a lot of fun with
it.

[browserify]: http://browserify.org
[lr]: http://livereload.com
[ck]: https://incident57.com/codekit/
[hbc]: https://holybible.com
[ng]: https://angularjs.org
[express]: http://expressjs.com
[node]: http://nodejs.org
[jasmine]: http://jasmine.github.io
[jn]: https://github.com/mhevery/jasmine-node
[iolang]: http://iolanguage.org
[iopost]: http://www.chriskrycho.com/2014/a-little-crazy.html

[^1]: Thus the early balding starting by my temples.

[^2]: My wife's bemused response: "Is that *another* language?" Take that as you will.

[^io]: The couple weeks I got to spend [playing][iopost] with [Io][iolang] certainly helped! Io's prototypal inheritance is semantically "purer" than JavaScript's, which is quite an improvement in my view. JavaScript's `new` keyword and the pseudo-classical object pattern it brings along can go rot in a bog.

[^coffee]: Truth be told, I like them even better from the perspective of CoffeeScript, which hides a lot of the rough edges of JavaScript and, as noted above, brings in quite a few things I like from Python. For my part, I intend to write as much CoffeeScript as possible going forward.
