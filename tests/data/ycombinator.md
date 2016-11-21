---
Title: Y Combinators, how do they even work?
Subtitle: "A commentary on a blog post by Matt Might, or: learning out loud."
Date: 2016-06-19 09:20
Tags: javascript, software development, programming languages
---

<i class=editorial>I was reading [a post] by [Matt Might], a computer science professor at the University of Utah, about Y Combinators, and I was having a hard time tracking with some of it just by reading. The way I normally solve this problem is to write it out---and, optimally, to write it out in something roughly like [Literate Haskell] or [Literate CoffeeScript]. That's exactly what you'll find below; this is basically *commentary* on Might's original post.</i>

[a post]: http://matt.might.net/articles/implementation-of-recursive-fixed-point-y-combinator-in-javascript-for-memoization/
[Matt Might]: http://matt.might.net
[Literate Haskell]: https://wiki.haskell.org/Literate_programming
[Literate CoffeeScript]: http://coffeescript.org/#literate

<i class=editorial>A few other prefatory notes:</i>

1. <i class=editorial>Since this is commentary, I'm not focusing on explaining combinators in general. For a very helpful explanation, though, both of what combinators are and why you'd ever want to use them, [read this][p.se].</i>
2. <i class=editorial>The Y Combinator itself isn't all that useful for ordinary programming. It *is* really useful as a way of thinking about how programming *works*, and that's why I was reading about it and trying to figure out what was going on in Might's original post.</i>
3. <i class=editorial>This didn't actually all make sense to me until I also read Might's post, ["Equational derivations of the Y combinator and Church encodings in Python"][church-python]. Which is a crazy post. But kind of fun.
</i>

[p.se]: http://programmers.stackexchange.com/a/117575
[church-python]: http://matt.might.net/articles/python-church-y-combinator/

---

Note for background (this was new to me today): $λv.e$ is the function which maps v to e. In ECMAScript 2015 or later (hereafter just JS):

```js
const λv_e = v => e
```

The Y Combinator is a higher-order functional: it is a function which takes a functional/higher-order function. Quoting from Might:

> The Y combinator takes a functional as input, and it returns the (unique) fixed point of that functional as its output. A functional is a function that takes a function for its input. Therefore, the fixed point of a functional is going to be a function.

And a "fixed point" is an input to a function equal to the *output* of the function. (Not all functions have such.) A fixed point is where $f(x) = x$. He uses the example $x = x^2 - 1$, which has two solutions, two *fixed points*.

He starts out with the total recursion form---also known as the "crash all the things!" form---of the Y-combinator. (I'm using letters to denote the version of the combinator; this is Y-naive.)

```js
const Yn = (F) => F(Yn(F))  // all the recursing!
```

"Crash all the things"... because of one pesky little detail: it calls itself immediately, and so recurses infinitely. Which is actually kind of a problem.

Might then asks: What if we transformed this a bit? He notes that we can *transform* with lambda calculus to expand what we're doing, so:

<figure>
$Y(F) = F(λx.(Y(F))(x))$
</figure>

(I haven't done this kind of thing since undergraduate math work I did for physics, but as I was thinking about it, it made sense. I'm used to trying to *remove* extraneous variables when dealing with software, but in this case we're using it as a tool for transforming the equation into a form that is *equivalent* but *expressed differently*.)

And $λx.(Y(F))(x)$ is equivalent to the fixed point. It's the function which takes $x$ as an argument and results in $Y(F)(x)$; but $Y(F)$ is just another argument, so this looks just like our original $f(x) = x$, but with $Y(F)$ substituted for $f$. Can we write this in JS?

Here's my implementation, using modern JS; note that it still recurses. (I'm calling this updated Y-transformed, so `Yt`.)

```js
const Yt = (F) => F((x) => Yt(F)(x))
```

His version:

```js
function Y(F) { return F(function(x) { return Y(F)(x); }); }
```

Mine and his are equivalent; here's his version transformed to modern JS:

```js
const Y = (F) => F((x) => Y(F)(x))
```

Might then says:

> Using another construct called the U combinator, we can eliminate the recursive call inside the Y combinator, which, with a couple more transformations gets us to:

I hated it when profs (or books!) did this when I was in college, and it frustrates me here, too. I want to *see* the transformation. I really wish Might didn't skip how the U combinator works or what transformations he applies, because then he jumps to this form:

<figure>
$Y = (λh.λF.F(λx.((h(h))(F))(x))) (λh.λF.F(λx.((h(h))(F))(x)))$
</figure>

Writing this out in JS is going to be a real bear. More to the point, I don't know how he got to it; now I need to go look up the U Combinator it seems.

...which I've [now done][U Combinator]. So:

> In the theory of programming languages, the U combinator, $U$, is the mathematical function that applies its argument to its argument; that is $U(f) = f(f)$, or equivalently, $U = λ f . f(f)$.

[U Combinator]: http://www.ucombinator.org

-   That is, the U Combinator is the case where you apply a function to itself: $U(f) = f(f)$---you can see that in the result there, where the first expression is the same as the argument handed to it (and both are functions). It's also there in the $h(h)$ calls.
-   The transformations are just transforming from a function-argument for to a lambda form, I think. The kind of thing where you go from `function a(b) { return c }` to `var a = function(b) { return c }` in JS. (Better, in *modern* JS, to `const a = (b) => c`.)

I'll return to that in a moment. First, writing up the JS. The innermost term is (repeated) $λx.((h(h))(F))(x)$, so we'll start by writing this out.

```js
const λ_inner = (x) => (h(h)(F))(x)
```

We need the definition of $h$ next; this comes from further out, the transformation $λh.λF.F(λ_inner)$ (where we're substituting the `λ_inner` we just wrote to make this a bit easier to get our heads around).

Remembering that each "." in the equation represents a mapping, i.e. a JS function call, we have this (writing it with function definitions starting new lines to clarify):

Here's what I came up with as a fairly direct translation into JS:

```js
const Y = (
  (h) =>
    (F) => F((x) => (h(h)(F))(x))  // substituting λ_inner from above
) (
  (h) =>
    (F) => F((x) => (h(h)(F))(x))  // substituting λ_inner from above
)
```

His (note that things are aligned as they are so that it's clear which functions match up):

```js
var Y = function (F) {
 return (function (x) {
  return F(function (y) { return (x(x))(y);});
  })
        (function (x) {
  return F(function (y) { return (x(x))(y);});
  }) ;
} ;
```

His transformed to modern JS:

```js
const Y = (F) => (
  (x) => F((y) => x(x)(y))
) (
  (x) => F((y) => x(x)(y))
)
```

His and mine are not *quite* the same (though I know they're equivalent because they both work). I really wish he'd explained how he got *this* substitution as well! More importantly, I wish he'd been consistent in his notation; changing variable names is... frustrating when you're trying to follow someone's work.

<i class=editorial>When I get stuck on something like *this*, the way I figure it out is by writing out how the substitutions would work at each step. See below.</i>

In any case, now that we have the Y combinator, we can use it with `FactGen`, a functional which, if you pass it the factorial function, passes back the factorial function. `FactGen` itself isn't recursive. But with the Y Combinator, it builds a function which is *not* recursive; it doesn't reference itself anywhere. It just needs the right kind of "factory": a function which returns *another* funtion which itself *is* recursive. Here's a standard recursive factorial implementation (identical to the one Might supplies, though modernized):

```js
const FactGen =
  (fact) =>
    (n) => n === 0 ? 1 : n * fact(n - 1)
```

You call that like this:

```js
Y(FactGen)(5)  // 120
```

The `Y(FactGen)` call gets back a function which then runs on whatever input you hand it (a fairly standard pattern with curried arguments), so you could also write it like this:

```js
const factorial = Y(FactGen)
factorial(5)  // 120
```

But I'm still not sure how his and mine are equivalent.

A note: wrapping things in `(...)` in JS defines that wrapped content as a distinct *expression*. As long as the type of a given expression is a function, it can be called with an argument. So `(function() {})()` or `(() => {})()` takes a no-operation function and immediately executes it.

So in his Y combinator, the substitution goes like this:

```js
const Y = (F) => (  // F is FactGen
  // x is the identical function passed as argument below
  (x) =>
    // Run FactGen by taking the function below as its `fact`
    // argument.
    F(
      // `y` is the argument passed to the result of Y, e.g.
      // `fact(5)`. Recall that `x` is the function below; we
      // call it with itself. Calling x(x) will get the actual
      // factorial function returned by `FactGen`.
      (y) => x(x)(y)
    )
// We close the *expression* which defines the outer function,
// and call it with this next expression as an argument.
) (
  // and x here is the same function, passed as argument
  (x) =>
    // Again, run `FactGen` with this function as its argument.
    F(
      // `y`, again, will be the integer. `x(x)` again will be
      // the actual factorial function.
      (y) => x(x)(y)
    )
)
```

This is pretty funky! But it works; the two anonymous functions call *each other* rather than recursing directly.

In mine, it goes like this, instead:

```js
const Ymine = (
  // Where in Might's example, the `x` function was where the
  // U Combinator was applied, here (because I followed the
  // original notation he gave) it's `h`. So it's `h` which is
  // the same function handed back and forth as argument
  // to itself.
  (h) =>
    // `h` takes a functional, which takes `FactGen` as its
    // parameter. This is similar to the outermost function in
    // Might's version.
    (F) =>
      // As in Might's version, we call `FactGen` here.
      F(
        // The form is *similar* but not identical to his,
        // because of the extra call structure. `h(h)(F)` is the
        // factorial function.
        //
        // Note that then he has `y` where I have `x`; my `x`
        // and his `y` are just the result of the computation
        // (in this case, the integer factorial).
        (x) => (h(h)(F))(x))
) (
  // This is identical to the above; it's using the U Combinator.
  (h) => (F) => F((x) => (h(h)(F))(x))
)
```

This is how his simplification worked: instead of generating the factorial function each time, it generated it just the once and then *used* it.

I still couldn't *do* the simplification he did myself. It'll take more practice using and thinking about combinators and combinatorial logic before I get there, but that's okay. That's how learning works.

And that's enough playing with combinatorials for now. (Except that I'm kind of tempted to see if I can go implement the U or Y combinators---or both---in Rust.)

---

<i class=editorial>If you're curious how I worked this out... I expanded the JS representations of the final forms ([here's the code][JS]) and then stepped through the result in my JavaScript dev tools, watching how the function calls worked and what the values of each intermediate value were. It's fascinating, and well worth your time.</i>

[JS]: //www.chriskrycho.com/extra/ycombinator.js
