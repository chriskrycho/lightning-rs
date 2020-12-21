---
title: >
    Data Constructors, Part 1: Understanding by Implementing
subtitle: >
    Understanding an idea from Standard ML-like languages by implementing it in (boring) TypeScript.
summary: >
    Demystifying and explaining the idea of “data constructors”—from languages like Elm, Grain, Haskell, F#, OCaml, ReasonML, etc.—by implementing an example in TypeScript.
qualifiers:
    audience: >
        Software developers who already know a typed language with classes, such as Java, C#, or TypeScript, and who want to understand what’s happening in “data constructors” in typed functional programming languages like Haskell, Elm, ReasonML, etc.

thanks: >
    [Chris Freeman](https://github.com/cafreeman) first flagged up the need for this writeup, and bore with my initial halting attempts to explain it. Michael Ciccotti let me know about a rendering issue in the first version of this post I published. [Oscar Spencer](https://github.com/ospencer) caught a mistake in some Grain sample code and suggested adding some extra details on pattern matching to a footnote. (As always, all mistakes are my own!)

tags:
    - TypeScript
    - Elm
    - F#
    - Haskell
    - OCaml
    - ReasonML
    - Grain
    - functional programming
    - programming languages
    - type theory
    - software development

series:
    name: Data Constructors
    part: 1

date: 2020-09-05T15:45:00-0600
updated: 2020-10-21T20:05:00-0600

---

Today’s topic: <i>What is a “data constructor” in languages like [Elm](https://elm-lang.org), [Haskell](https://www.haskell.org), [F^♯^](https://fsharp.org), [OCaml](https://ocaml.org)/[ReasonML](https://reasonml.github.io), [Grain](https://grain-lang.org), etc.?[^swift-and-rust-too] When you see syntax like this (taken from [the Grain docs](https://grain-lang.org/docs/guide/data_types)[^grain-update])—</i>

```grain
data CabbageColor = Red | Green
data Veggie =
  | Squash
  | Cabbage(CabbageColor)
  | Broccoli

let redCabbage = Cabbage(Red)
```

<i>—what does it mean, and how can we understand it in terms of things we can do in languages like Java, C^♯^, and TypeScript?</i>

Even to people with quite a bit of experience in a variety of programming languages, the syntax here is different enough from *all* the C-related languages most working developers use that it can be hard to wrap your head around. In fact, one of the smartest developers I know got stuck on trying to make sense of this syntax recently. So in this post, I’m going to explain it using TypeScript, in the same terms that made it make sense to that developer. If you’re familiar with any modern language with classes, this will likely make sense to you!

We’re going to take this in two steps:

1. [What the Syntax Means](#what-the-syntax-means)
2. [Understanding—by Implementing](#understandingby-implementing)

If you get through the first section and still feel a bit confused, that’s okay—in fact, it’s almost the point. Take a breather, go get a drink and take a walk or something, and then come back and read the second section!

[^grain-update]: Since I wrote this, Grain has made a change to their syntax which makes this *much* more familiar-looking to developers used to C-like languages. This sample would now look like this:

    ```grain
    enum CabbageColor { Red, Green }
    enum Veggie {
        Squash,
        Cabbage(CabbageColor),
        Broccoli
    }

    let redCabbage = Cabbage(Red)
    ```

    I have chosen to leave the text of the post as is apart from this footnote because I think it's helpful in exposing people to ML-ish syntaxes… *and* it would be an enormous amount of work to rewrite the whole thing at this stage!

## What the Syntax Means

First, let’s get some terms defined: each of those `type ...` is declaring a type. The name of the two types are `CabbageColor` and `Veggie`. The items after the `=`, separated by `|` characters, are the *values* of the type, sometimes called the *variants*. This kind of type goes by a lot of names, including “sum types,” “union types,” “user-defined types,” “custom types,” and more. The key is that they define a type—`CabbageColor` or `Veggie`—where instances of that type are exactly and only one of the named values. So when we see this—

```grain
data CabbageColor = Red | Green
```

—it just means that if you have a `CabbageColor`, you know it will be one of the values `Red` or `Green`. Those values *only* exist in the context of `CabbageColor` If you want to use the names `Red` and `Green` for values otherwise, you’ll need some way to specify *which* `Red`, like `CabbageColor.Red` in some languages. This is very similar to enums in C-descended languages. In at least some C-descended languages, the boolean type is defined pretty much exactly the same way that `CabbageColor` is here: an enum with variants named `true` and `false`.

Using a basic custom type like this is pretty much like you’d expect:

```grain
let colorOne = Red     // type is CabbageColor
let colorTwo = Green   // type is *also* CabbageColor
```

We could write a type annotation on that to make it extra explicit, even though we don’t *need* to:

```grain
let colorOne: CabbageColor = Red
let colorTwo: CabbageColor = Green
```

Unlike enums in C-based languages, though, these types have a superpower: they can hold data. We can see this in the second type defined in the opening example:

```grain
data Veggie =
  | Squash
  | Cabbage(CabbageColor)
  | Broccoli
```

The second type, `Cabbage`, includes a `CabbageColor`. This is *not* something you can do with a traditional enum in C, Java, C^♯^, or TypeScript. (This is *also* the point we’re going to spend the rest of the post on!)

Creating an instance of `Veggie` looks like this:

```grain
let squash = Squash                 // type is Veggie
let redCabbage = Cabbage(Red)       // type is Veggie
let greenCabbage = Cabbage(Green)   // type is Veggie
let broccoli = Broccoli             // type is Veggie
```

Each of these is a `Veggie`. We could prove it by spelling out the types rather than letting the compiler infer them for us, like this:

```grain
let squash: Veggie = Squash
let redCabbage: Veggie = Cabbage(Red)
let greenCabbage: Veggie = Cabbage(Green)
let broccoli: Veggie = Broccoli
```

But what *exactly* is happening when we write `let squash = Squash` or, perhaps even more surprisingly, `let redCabbage = Cabbage(Red)`?

- `Squash` is a value of type `Veggie`, so you can simply write `let squash: Veggie = Squash` and everything works
- `Cabbage` is a function (which is still a kind of value!), which accepts a `CabbageColor` and returns a `Veggie`.

That second point means we could also write this:

```grain
let color = Red                   // type is `CabbageColor`
let cabbage = Cabbage             // type is `CabbageColor -> Veggie`
let redCabbage = cabbage(color)   // type is `Veggie`
```

For some of you, this might already make sense—but it’s still neat to see how you might reimplement it in another more commonly-used language. For others, this might still be hurting your head, and *that’s okay*. It took me quite a while for this all to make sense to me when I first encountered it! If your head is spinning a bit right now, that’s normal. Take a break and go enjoy some fresh air, drink some water, and let your brain relax for a few minutes. Or the rest of the day. Come back after that and read the next section, and things will probably click into place.

## Understanding—by Implementing

People with backgrounds in languages like Java, C^♯^, or TypeScript often find it hard to translate the syntax we’ve just walked through into concepts they know. That’s totally fair: it’s pretty different, and it’s not *just* new syntax, it’s also new language features tied to that syntax. In this section, we’ll see how we could implement the exact same semantics in a language that’s more familiar, and hopefully that will help make sense of things.

:::note

I’m using TypeScript here because it’s the language in this family I’m most familiar with, but I’m going to keep it to a minimal subset of TypeScript that is extremely close to what you might see in Java or C^♯^. I’ll be using footnotes here to talk about some details around TypeScript itself, where Typescript can let us more directly approximate the things happening in languages like Grain, Elm, etc. However, those are footnotes for a reason: you don’t *need* to read or understand them to get the point of the rest of this post!

:::

First, let’s see what it would look like to build a type that represents the `CabbageColor`. For this we can just use a standard `enum` type:

```ts
enum CabbageColor {
  Red,
  Green,
}
```

That’s it for that particular type. To get an instance of the type, we just do `CabbageColor.Red`:

```ts
let color = CabbageColor.Red;
```

As we’d expect, `color` is of type `CabbageColor`; we could easily have specified it (but don’t need to because of type inference in TypeScript, the same as in Grain):

```ts
let color: CabbageColor = CabbageColor.Red;
```

We can now use the normal `switch` statement semantics with this:[^default-case]

```ts
function describe(color: CabbageColor): string {
  switch (color) {
    case CabbageColor.Red:
      return "It's red!";
    case CabbageColor.Green:
      return: "It's green!";
}
```

Even better: TypeScript will guarantee we cover all the cases of the `enum` in this `switch` statement, because it’s directly related to the `return` type. If we later added `Yellow` to the mix, but didn’t add a `case` for it, TypeScript would report:

> Function lacks ending return statement and return type does not include 'undefined'.

<aside>

Unfortunately, there’s a small amount of runtime overhead to the result of creating an `enum` in TypeScript—it’s literally the only thing in the language like this. You might also be tempted to solve it by using a frozen object instead, like this:

```ts
const CabbageColor = Object.freeze({
  Red: 'red',
  Green: 'green',
});
```

That won’t get us the same benefits as an `enum` without a *bunch* of extra type machinery, though—I covered the relevant details in two posts back when it was introduced ([1](https://v4.chriskrycho.com/2016/keyof-and-mapped-types-in-typescript-21.html "keyof and mapped types in TypeScript 2.1"), [2](https://v4.chriskrycho.com/2017/typescript-keyof-follow-up.html "TypeScript keyof Follow-Up"))—so we’re better just using the built-in `enum` type. TypeScript’s own `const enum` types would be one good solution to both of these problems (albeit with their own tradeoffs); I’ll show those in detail in the next post. 

</aside>

We can’t do exactly this for the `Veggie` type, though: it would be fine for `Squash` and `Broccoli`, but `Cabbage` needs a `CabbageColor` to create it! That’s okay, though: we can still create a type that behaves the same way as the `Veggie` type does.

Let’s start with an empty `class` definition:[^classes]

```ts
class Veggie {
}
```

The first thing we’ll want to do is define the kind of veggie this represents. We can do that with another `enum` for the `kind` (and notice that the `kind` here is marked as `readonly` because the `kind` of a `Veggie` is fixed: squash cannot turn into cabbage, etc.):[^constructor-shorthand]

```ts
enum VeggieKind {
  Squash,
  Cabbage,
  Broccoli,
}

class Veggie {
  readonly kind: VeggieKind;

  constructor(kind: VeggieKind) {
    this.kind = kind;
  }
}
```

With this in place, we could actually construct a `Squash` or a `Broccoli` correctly:

```ts
let squash = new Veggie(VeggieKind.Squash);
```

This isn’t exactly what we see in the Grain example, but it’s a step in the right direction.

Next, we need to deal with the extra data associated with the type when we are working with cabbage: the `CabbageColor` we defined above. We can put that in the constructor, too:

```ts
enum CabbageColor { Red, Green }
enum VeggieKind { Squash, Cabbage, Broccoli }

class Veggie {
  readonly kind: VeggieKind;
  readonly color?: CabbageColor;

  constructor(kind: VeggieKidn, color?: CabbageColor) {
    this.kind = kind;
    this.color = color;
  }
}
```

Here, we have an *optional* `color` parameter: it can be left `undefined`. That makes sense: there is no `color` associated with a `Squash`. But it’s a problem, too: nothing currently prevents our end users from writing something like this:

```ts
let badSquash = new Veggie(VeggieKind.Squash, CabbageColor.Green);
```

We can solve this problem *and* get ourselves to something that looks a lot more like the syntax we’re aiming for in one fell swoop: by making our constructor `private` and providing other ways to create a `Veggie` which are guaranteed to be safe.

:::note

This is the part where people who are already familiar TypeScript with have to wait for the next post. There are ways we can make this *much* more type-safe. That's not the point of *this* post, though! Here, we’re intentionally sticking to a “lowest common denominator” implementation to get at how we can do this in *any* class-based language.

:::

We’ll start by adding a `static` constructor for each of these types.

```ts
enum CabbageColor { Red, Green }
enum VeggieKind { Squash, Cabbage, Broccoli }

class Veggie {
  readonly kind: VeggieKind;
  readonly color?: CabbageColor;

  private constructor(kind: VeggieKind, color?: CabbageColor) {
    this.kind = kind;
    this.color = color;
  }

  static Squash(): Veggie {
    return new Veggie(VeggieKind.Squash);
  }

  static Cabbage(color: CabbageColor) {
    return new Veggie(VeggieKind.Cabbage, color);
  }

  static Broccoli(): Veggie {
    return new Veggie(VeggieKind.Broccoli);
  }
}
```

At this point, we’ve isolated the things needed to make the type behave the way it should in these constructors, so now we can only use it correctly. If we type a version of the same bad code as before—

```ts
let badSquash = Veggie.Squash(CabbageColor.Red);
```

—the compiler will tell us:

> Expected 0 arguments, but got 1.

This is a good start! But we can do better.

For one thing, `Squash` and `Broccoli` don’t need to be methods at all. We don’t actually need to be able to make a *new* `Squash` instance every time, because this class doesn't actually have any state, or any way to change state. Not having any internal state that can change means it doesn’t actually matter if there is only ever one instance of `Squash` and one of `Broccoli` anywhere in our system. We can represent having multiple quantities of them by having more than one reference to the single value in a given array or other data structure, and because the type is stateless, that's totally fine. There will *never* be any bugs from having the same value used in different spots—because it's immutable.

Instead of having those individual constructors, then, we can just create a single static *value* for the `Squash` and `Broccoli` veggies.

```ts
enum CabbageColor { Red, Green }
enum VeggieKind { Squash, Cabbage, Broccoli }

class Veggie {
  private kind: VeggieKind;
  private color?: CabbageColor;

  private constructor(kind: VeggieKind, color?: CabbageColor) {
    this.kind = kind;
    this.color = color;
  }

  static Squash = new Veggie(VeggieKind.Squash);

  static Cabbage = (color: CabbageColor) =>
    new Veggie(VeggieKind.Cabbage, color);

  static Broccoli = new Veggie(VeggieKind.Broccoli);
}
```

The private constructor makes it so the only way to create a `Veggie` is using one of its public, `static` fields. Critically, all three of of them are just values—yes, including the `Cabbage` function. In fact, we could actually use static method syntax for `Cabbage` here, but I intentionally used the static property function for `Cabbage` to make it more obvious that these are *all* just values attached to the class! `Veggie.Squash` and `Veggie.Broccoli` are values whose type is `Veggie`. `Veggie.Cabbage` is a value whose type is a function which accepts a `CabbageColor` and returns a `Veggie`. But even though one of those values is a function, they’re still all just values.

Since `Squash`, `Cabbage`, and `Broccoli` are all just values, we can even bind any of them directly to a value in local scope:

```ts
let squash = Veggie.Squash;     // Veggie
let broccoli = Veggie.Broccoli; // Veggie
let cabbage = Veggie.Cabbage;   // (color: CabbageColor) -> Veggie
```

Again, the difference is simply the *type* each one has: `squash` and `broccoli` are already `Veggie` here, but `cabbage` is a function: `(color: CabbageColor) => Veggie`.

This same thing is true back in the ML languages, just with a different syntax:

```grain
squash = Squash      // Veggie
broccoli = Broccoli  // Veggie
cabbage = Cabbage    // CabbageColor -> Veggie
```

The difference here is that, since this is the *normal* way of constructing these types in languages like Elm, you don’t need to use the scoped class for it. You can imagine that it’s as if we had used capital names for those letters in our bindings and exported them all from a module:

```ts
export const { Red, Green } = CabbageColor;
export const { Squash, Cabbage, Broccoli } = Veggie;
```

Then we could import them and use them elsewhere:

```ts
import { Broccoli, Cabbage, Red } from 'veggies';

let broccoli = Broccoli;
let redCabbage = Cabbage(Red);
```

That’s exactly the same thing we’d see in Grain or any other ML-style language—just with TypeScript syntax instead!  

Summarizing so far:

1. All of these variants are *values*. This is why we can bind them, export them, etc.
2. The difference is what *type* each value is. `Veggie.Squash` and `Veggie.Broccoli` are both already `Veggie`s. `Veggie.Cabbage` is a function you can use to *build* a `Veggie` if you also supply a `CabbageColor`.

The only real difference in what we’ve done in TypeScript and what we’d see in that original example from Grain is that Grain has built-in language support for these things because they’re the default instead of something we’re building on top of other language constructs.

There *is* another difference, though, and it’s related to a downside in the code we’ve written. We can no longer use a `switch` statement to check this, because it’s too complicated a type for JavaScript’s very limited `switch` capability. All the SML-related languages I mentioned at the top have a feature called *pattern-matching* which supports working with these richer types:[^matching]

```grain
let describeColor = (color) => match (color) {
  | Red => "red"
  | Green => "green"
}

let describe = (veggie) => match (veggie) {
  | Squash => "It's a squash"
  | Cabbage(color) => "It's a " + describeColor(color) + " cabbage"
  | Broccoli => "It's broccoli"
}
```

While `match` is not built into JavaScript, we can build our own using a method on the class, and it’s not actually very hard!

What we want to end up with:

```ts
let describeColor = (color: CabbageColor): string => {
  switch (color) {
    case CabbageColor.Red:
      return "red";
    case: CabbageColor.Green:
      return "green";
  }
};

let describe = (veggie: Veggie): string => veggie.match({
  Squash: "It's a squash",
  Cabbage: (color) => `It's a ${describeColor(color)} cabbage`,
  Broccoli: "It's broccoli",
});
```

`describeColor` can just be exactly this: there’s no reason for us to reinvent the wheel when the built-in language tools JS gives us—here, a `switch` statement—will do just fine. For `describe`, working with a `Veggie`, though, implementing this `match` method gives us a *lot* more expressiveness, and we *need* something besides the JS `switch` to deal with `Cabbage`! Happily, the method implementation is fairly straightforward.

```ts
enum CabbageColor { Red, Green }
enum VeggieKind { Squash, Cabbage, Broccoli }

type Matcher<T> = {
  Squash: T;
  Cabbage: (color: CabbageColor) => T;
  Broccoli: T;
};

class Veggie {
  private kind: VeggieKind;
  private color?: CabbageColor;

  // SNIP: the constructors are the same!

  match<Output>(matcher: Matcher<Output>): Output {
    switch (this.kind) {
      case VeggieKind.Squash:
        return matcher.Squash;
      case VeggieKind.Cabbage:
        // SAFETY: we guarantee `color` is always defined with `Cabbage`
        // in the public constructors.
        return matcher.Cabbage(this.color!);
      case VeggieKind.Broccoli:
        return matcher.Broccoli;
    }
  }
}
```

*All* we’ve done here is require the caller to pass us an object with names which match the names of the `VeggieKind` variants. The values on that object are either values of the desired resulting type `T` in the case of `Squash` and `Broccoli`, or a function which takes a `CabbageColor` and returns that same resulting type of `T`. Within the body of the `match` method, we return whichever one corresponds to `this.kind`.

:::note

Notice the `// SAFETY: ...` comment I added when using the non-null assertion operator `!` with `this.color`. I borrowed this idea from the Rust community, which marks all uses of `unsafe` with these kinds of comments. I use it *any time* I write a cast in TypeScript, for the sake of whoever is maintaining the code in the future… including future *me*. It’s important to know what might make a cast unsafe! For a way to not need this comment at all by having better safety, you’ll want to read the *next* post.

:::

With the `match` method in place, we can now *use* that to work with any of the variants, exactly as I showed above with the `describe` function definition:

```ts
let describeColor = (color: CabbageColor) => {
  switch (color) {
    case CabbageColor.Red:
      return "red";
    case: CabbageColor.Green:
      return "green";
  }
};

let describe = (veggie: Veggie) => veggie.match({
  Squash: "It's a squash",
  Cabbage: (color) => `It's a ${describeColor(color)} cabbage`,
  Broccoli: "It's broccoli",
});
```

Once we have that function, we can do pretty neat things with it! For example, if we have a list of `Veggie`s, we can now `map` over them in whatever ways we like. And since we have `describe`, we don’t even have to explicitly invoke `Veggie.match` to describe a list of `Veggie`s:

```ts
let veggies = [
  Veggie.Squash,
  Veggie.Cabbage(CabbageColor.Red),
  Veggie.Squash,
  Veggie.Broccoli,
  Veggie.Broccoli,
  Veggie.Cabbage(CabbageColor.Green),
];

veggies
  .map((veggie) => describe(veggie))
  .forEach((desc) => {
    console.log(desc);
  });
// It's a squash
// It's a red cabbage
// It's a squash
// It's broccoli
// It's broccoli
// It's a green cabbage 
```

Here’s the final version of our class, showing how we can implement the original code from Grain in TS:

```ts
enum CabbageColor {
  Red,
  Green,
}

enum VeggieKind {
  Squash,
  Cabbage,
  Broccoli,
}

type Matcher<T> = {
  Squash: T;
  Cabbage: (color: CabbageColor) => T;
  Broccoli: T;
};

class Veggie {
  private kind: VeggieKind;
  private color?: CabbageColor;

  private constructor(kind: VeggieKind, color?: CabbageColor) {
    this.kind = kind;
    this.color = color;
  }

  static Squash = new Veggie(VeggieKind.Squash);

  static Cabbage = (color: CabbageColor) =>
    new Veggie(VeggieKind.Cabbage, color);

  static Broccoli = new Veggie(VeggieKind.Broccoli);

  match<Output>(matcher: Matcher<Output>): Output {
    switch (this.kind) {
      case VeggieKind.Squash:
        return matcher.Squash;
      case VeggieKind.Cabbage:
        // SAFETY: we guarantee `color` is always defined with `Cabbage`
        // in the public constructors.
        return matcher.Cabbage(this.color!);
      case VeggieKind.Broccoli:
        return matcher.Broccoli;
    }
  }
}

let describeColor = (color: CabbageColor) => {
  switch (color) {
    case CabbageColor.Red:
      return "red";
    case CabbageColor.Green:
      return "green";
  }
};

let describe = (veggie: Veggie) => veggie.match({
  Squash: "It's a squash",
  Cabbage: (color) => `It's a ${describeColor(color)} cabbage`,
  Broccoli: "It's broccoli",
});

let redCabbage = Veggie.Cabbage(CabbageColor.Red);
let description = describe(redCabbage);
```

And here’s the original Grain code again:

```grain
data CabbageColor = Red | Green
data Veggie =
  | Squash
  | Cabbage(CabbageColor)
  | Broccoli

let describeColor = (color) => match (color) {
  | Red => "red"
  | Green => "green"
}

let describe = (veggie) => match (veggie) {
  | Squash => "It's a squash"
  | Cabbage(color) => "It's a " + describeColor(color) + " cabbage"
  | Broccoli => "It's broccoli"
}

let redCabbage = Cabbage(Red)
let description = describe(redCabbage)
```

Our TypeScript code is definitely longer, because we had to *create* the ability to do what Grain does at the language level. However, doing so means we can actually see what Grain is doing quite clearly. In particular, the original syntax `Cabbage(Red)` confuses a lot of people who aren’t familiar with the syntax of languages like Grain. Having implemented it in TypeScript, though, we can see that `Cabbage` is just a function which takes an argument, `CabbageColor`, and returns a `Veggie`.

In fact, as we saw above, we can make it possible to write *exactly* the same thing in TypeScript to construct a `Veggie` as we do in Grain, by creating standalone versions of the “data constructors” for `Veggie` and `CabbageColor`:

```ts
const { Red, Green } = CabbageColor;

const Squash = Veggie.Squash; // Veggie
const Cabbage = Veggie.Cabbage; // (color: CabbageColor) => Veggie
const Broccoli = Veggie.Broccoli; // Veggie
```

Once we have these, building a red cabbage looks exactly the same as it did in Grain:

```ts
let redCabbage = Cabbage(Red);
```

And that’s it! We’ve now seen how you can implement the kinds of data constructors and pattern matching you see in languages like Elm, Grain, Rust, etc. in a language like TypeScript. I hope that helps make data constructors and pattern matching seem a bit less magical. The thing which makes them so great in languages with them built in is that you don’t have to reimplement that functionality yourself for every time: the language handles it for you!

In the next post, we’ll see how we can make this TypeScript implementation safer *and* cheaper—that is, how I would actually implement things in TypeScript!

## Appendix: “Point Free”

One bonus bit of material here—this is totally unnecessary for the rest of the post, but it’s a neat thing I enjoy, so I’m sharing it here.

Sometimes we end up using the same bit of pattern-matching behavior where we map from the `Veggie` variant into a particular output type over and over again. In that case, it’s convenient to extract a helper function for it. That can be particularly convenient when we’re working with methods like `Array.map` (or utilities like [lodash](https://lodash.com) or [Ramda](https://ramdajs.com)).

This is exactly what we did with `describe` above:

```ts
let describe = (veggie: Veggie) => veggie.match({
  Squash: "It's a squash",
  Cabbage: (color) => `It's a ${describeColor(color)} cabbage`,
  Broccoli: "It's broccoli",
});
```

However, this still leaves us with quite a bit of repetition in terms of naming the types, *especially* if we write it out fully like we did above:

```ts
let veggies = [
  Veggie.Squash,
  Veggie.Cabbage(CabbageColor.Red),
  Veggie.Squash,
  Veggie.Broccoli,
  Veggie.Broccoli,
  Veggie.Cabbage(CabbageColor.Green),
];

veggies
  .map((veggie) => describe(veggie))
  .forEach((desc) => {
    console.log(desc);
  });
```

Notice that we have the *same* pattern of doing `(veggie) => <some operation>` more than once. For this, I like to use a style called “point free function application.” The name isn’t all that illuminating, unfortunately, unless you have a specific background in certain fields of mathematical theory. (I happen to think those fields are pretty cool, but realistically most working programmers aren’t familiar with them.) “Point free” really just means passing the function directly to another function by name, without creating another anonymous function in between to invoke it (“pointed”).

```ts
// Pointed
let descriptions = veggies.map((veggie) => describe(veggie));

// Point free
let descriptions = veggies.map(describe);
```

I also like to name my functions in a way that works well in this kind of invocation, so that reading the line almost reads like a sentence. Here, I would pick the name `toDescription` instead of `describe`. Then the point-free invocation would read like this:

```ts
let descriptions = veggies.map(toDescription);
```

If we were using lodash, it would read *even more* like a sentence:

```ts
import { map } from 'lodash/map';

let descriptions = map(veggies, toDescription);
```

Now, because I’ve gotten used to this way of doing things, I really enjoy being able to just work with functions like this in general. In fact, I like it *so* much that I’d really prefer that if I’m *only* using a given `.match` invocation one place, I could just use `Veggie.match` directly instead of having to do `(veggie) => veggie.match({ ... })`.[^weird]

In other words, what if instead of this—

```ts
let descriptions = veggies.map((veggie) => veggie.match({
  Squash: "It's a squash",
  Cabbage: (color) => `It's a ${describeColor(color)} cabbage`,
  Broccoli: "It's broccoli",
}));
```

—we could skip the creation of the intermediate anonymous function and just write this instead?

```ts
let descriptions = veggies.map(Veggie.match({
  Squash: "It's a squash",
  Cabbage: (color) => `It's a ${describeColor(color)} cabbage`,
  Broccoli: "It's broccoli",
}));
```

And if we *did* have this ability, we could skip the `(veggie: Veggie) => veggie.match({ ... })` when defining `toDescription`, too:

```ts
let toDescription = Veggie.match({
  Squash: "It's a squash",
  Cabbage: (color) => `It's a ${describeColor(color)} cabbage`,
  Broccoli: "It's broccoli",
});
```

It turns out, this is actually quite easy to implement in modern JavaScript or TypeScript. We just create a `static` version of `match`, which can live right next to the class method. The key is that we make `Veggie.match` return another function which takes a `Veggie` and returns the output type from the matcher.

```ts
class Veggie {
  // SNIP: everything else is the same, and so is `match`; I've
  // left it here just so you can see it working right next to
  // the static method

  match<Output>(matcher: Matcher<Output>): Output {
    // SNIP: body is the same as before
  }

  static match<T>(matcher: Matcher<T>): (veggie: Veggie) => T {
    return (veggie) => veggie.match(matcher);
  }
}
```

That’s it: the code samples I wrote above all just work now!

We can actually go one better and use TypeScript’s overloading to make it so you can use this static version of `match` in either this “curried” form *or* a form that accepts a `Veggie` as its second parameter, in case you’re in a context where that makes things clearer, with an “overloaded” version of the function:

```ts
class Veggie {
  // SNIP: everything else is the same

  static match<T>(matcher: Matcher<T>): (veggie: Veggie) => T;
  static match<T>(matcher: Matcher<T>, veggie: Veggie): T;
  static match<T>(
    matcher: Matcher<T>,
    veggie?: Veggie
  ): T | ((veggie: Veggie) => T) {
    return veggie
      ? veggie.match(matcher)
      : (veggie) => veggie.match(matcher);
  }
}
```

Then we can use it in either mode, and TypeScript will resolve the type correctly:

```ts
let longDescFn = (veggie: Veggie) => Veggie.match({
  Squash: "It's a squash",
  Cabbage: (color) => `It's a ${describeColor(color)} cabbage`,
  Broccoli: "It's broccoli",
}, veggie);

let shortDescFn = Veggie.match({
  Squash: "It's a squash",
  Cabbage: (color) => `It's a ${describeColor(color)} cabbage`,
  Broccoli: "It's broccoli",
});

let descsFromLong = veggies.map(longDescFn);
let descsFromShort = veggies.map(shortDescFn);
assertDeepEqual(descsFromLong, descsFromShort);
```

In this case, there’s no particular value to adding that functionality, since it’s just the same as calling `veggie.match` instead of `Veggie.match` with the second argument. It *is* useful to understand the pattern, though, and the first variant where we *don’t* require the argument lets us create our `toDescription` function much more directly. Functions have become a thing we work with like any other value. While this takes some getting used to, it’s also an incredibly powerful tool to have in your toolbox!

:::callout

Enjoyed this? Check out [Part 2](/journal/data-constructors-part-2-better-typescript/), which covers how to reimplement this using more advanced features of TypeScript’s type system, resulting in more type safety *and* improved performance.

:::



*[SML]: Standard ML
*[TS]: TypeScript

[^swift-and-rust-too]: Note that pretty much everything I say here goes, with minor differences in details, for Swift’s and Rust’s `enum` types as well!

[^default-case]: You may notice that I don’t have a `default` case here. That’s on purpose. Because I specify the return type of the function as `string`, TypeScript will actually tell me if I don’t cover all the cases in the switch statement. TypeScript is smart enough to know that if we *don’t* cover all the cases, it *won’t* return a string.

    This comes for free in languages like Grain, in *all* contexts where you’re “matching” on a given item.

[^classes]: You can use classes for all sorts of things, and not all of them have to do with inheritance! In this case, it’s just going to be a convenient tool for building up the data structure (and one that will be familiar to developers from *many* languages). As a bonus, you could implement an actual language similar to the way I will build up this type in the rest of this post.

[^constructor-shorthand]: Here I’m using the normal JavaScript version of the `constructor` syntax, but for scenarios like this TypeScript provides a convenient shorthand:

    ```ts
    class Veggie {
      constructor(readonly kind: VeggieKind) {}
    }
    ```

    If I were building this data type myself, that’s the declaration I would actually use!

[^matching]: Two things to note about the example of pattern-matching here:

    1.  I’m taking a tiny liberty here with the Grain sample code and acting as if `+` does string concatenation. It… doesn’t yet. But that’s just because Grain is extremely young; at *some* point it’ll have something which does this and nicely!

    2.  Pattern matching functionality is even deeper and richer than I'm showing here. Matching can deal with *nested* types, too. In this case, I wouldn't actually (necessarily) break out `describe` and `describeColor` this way. Instead, I might just use a richer `match` expression:

        ```grain
        let describe = (veggie) => match (veggie) {
          | Squash => "It's a squash"
          | Cabbage(Red) => "It's a red cabbage"
          | Cabbage(Green) => "It's a green cabbage"
          | Broccoli => "It's broccoli"
        }
        ```
        
        If the type were further nested, we could further drill down in manually like this, “destructuring” the types as deeply as we need. This makes it *much* more powerful than a `switch` statement from JS/TS/Java/C^♯^  etc.

[^weird]: I admit, that might make me a little weird to some of you. That’s okay! I kind of enjoy being a little weird.



*[JS]: JavaScript