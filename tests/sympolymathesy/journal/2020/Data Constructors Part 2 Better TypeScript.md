---
title: >
    Data Constructors, Part 2: Better TypeScript
subtitle: >
    A deep dive on more idiomatic TypeScript implementations of ML-style data constructors.
summary: >
    Building more idiomatic and better-performing versions of ML-style data constructors in TypeScript.
qualifiers:
    audience: >
        Software developers who already know TypeScript, and want to dig a little deeper. And *preferably* developers who have read the [the previous post](/journal/data-constructors-part-1-understanding-by-implementing/)!

thanks: >
    [Chris Freeman](https://github.com/cafreeman) first flagged up the need for this pair of posts, and bore with my initial halting attempts to explain these ideas to him.

tags:
    - functional programming
    - programming languages
    - TypeScript
    - type theory
    - Grain

series:
    name: Data Constructors
    part: 2

date: 2020-10-13T21:35:00-0600
updated: 2020-10-15T21:15:00-0600

---

In the [first post](https://v5.chriskrycho.com/journal/data-constructors-part-1-understanding-by-implementing/) in this two-part series, I showed how you can use fairly standard TypeScript (or Java or C^♯^) to implement the idea of "data constructors" from the Standard ML (SML). That post covered everything you need to know to understand what they are and how they work. However, I intentionally used a minimal subset of TypeScript’s features to make it as approachable as possible for readers who aren’t TS experts, or who are coming from other languages like Java or C^♯^. TypeScript provides tools we can use to implement the same idea more robustly *and* with better performance, though. In this post, I’ll explore two of those—with the hope that you come out with a better idea of how to do interesting things with some advanced elements of TypeScript’s type system!

*[SML]: Standard ML
*[TS]: TypeScript

## The existing implementation

First, let’s briefly review the existing implementation so that we’re on the same page about our starting point.

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
```

First, we have an `enum` for both `CabbageColor` and for `VeggieKind`. Second, we define a class with public constructors in the form of static methods or values, whose job it is to uphold the constraints that our `Veggie` have the right shape—for example, avoiding ending up with `kind: VeggieKind.Broccoli` *and* `color: CabbageColor.Red` instead of `color: undefined`. Finally, we have a `match` function which lets us "pattern-match" on the type we have.

There are a number of type safety issues with this implementation. First off, the *only* thing which guarantees we do the right thing with our `Squash`, `Cabbage`, and `Broccoli` constructors is… that we’ve checked it very carefully (and maybe written some tests). This function has to use the `!` non-null assertion operator because the class as written cannot guarantee that `color` is always defined when `kind` is `VeggieKind.Cabbage`. *We* can see that, but we can’t *prove* it, for the same reason that we have to rely on the correct behavior of our static constructors to keep things correct in the first place.

This approach has some performance issues as well. These are small in the grand scheme of things, but if we were building a *lot* of custom types like this, they might add up. First off, each `enum` here creates a fairly complicated object. Second, we need a class field for each value we could *ever* care about on the type. If we end up with something more complicated than just dealing with a single `CabbageColor` property—say, if we needed to store *multiple* other properties--this could end up adding up to a lot of extra "slots" on the class, which cost memory whether in use or not, and it could end up making it harder for the JavaScript VM’s just-in-time (JIT) compiler to optimize the class overall because of the inconsistent shapes (a problem sometimes called "megamorphism").

So let’s see how to fix these!

*[VM]: virtual machine
*[JIT]: just-in-time

## Some advanced TypeScript features

We can make this implementation lighter-weight *and* more robustly type-safe by leaning into a set of "fancy" features in TypeScript:

1. [Const enums](#1-const-enums)
2. [Literal types](#2-literal-types)
3. [Union types](#3-union-types)
4. [Tuple types](#4-tuple-types)

### 1. Const enums

A `const enum` declaration works much the same as a normal `enum` declaration in Typescript, but it has one critical difference: normal `enum` declarations have a (surprisingly complicated) compiled output that still exists at runtime, while `const enum`s are compiled out of existence entirely, replaced by the constant value they represent. (By default, that’s a number, though you can give it a string value as well.)[^1]

Given this `const enum` declaration and usage--

```ts
const enum ConstEnum { A, B }

function useConstEnum(x: ConstEnum) {
  console.log(x);
}

useConstEnum(ConstEnum.A);
```

--here’s the compiled output (as of TS 3.9.2):

```js
function useConstEnum(x) {
    console.log(x);
}

useConstEnum(0 /* A */);
```

Here is the same code implemented with a plain `enum` instead of a `const enum`:

```ts
enum RegularEnum { A, B }

function useRegularEnum(x: RegularEnum) {
  console.log(x);
}

useRegularEnum(RegularEnum.A);
```

And here is the corresponding output from TS 3.9.7:

```js
var RegularEnum;
(function (RegularEnum) {
    RegularEnum[RegularEnum["A"] = 0] = "A";
    RegularEnum[RegularEnum["B"] = 1] = "B";
})(RegularEnum || (RegularEnum = {}));

function useRegularEnum(x) {
    console.log(x);
}

useRegularEnum(RegularEnum.A);
```

Notice that there is much more code present at runtime for the plain `enum`. Most obviously, it comes with a declaration of a fancy object type. (This is makes it so that you can write `RegularEnum.A` and get out `0` *or* type `RegularEnum[0]` and get out `"A"`.[^2]) Second, note that the call `useRegularEnum(RegularEnum.A)` still refers to that fancy object type:

```js
useRegularEnum(RegularEnum.A);
```

Recall that the compiled call looked like this instead for the `const enum`:

```js
useConstEnum(0 /* A */);
```

This is how TypeScript gets rid of the runtime object representing a `const enum`--it just substitutes in the concrete values each lookup represents. This means that we can have a *much* lower cost for the enums we’re using for `CabbageColor` and `VeggieKind`. They will ultimately just be integers used inline, which means they will have extremely low memory costs, and using them does not involve an object lookup! These are small wins in any individual point in a codebase, but over a large app or in hot paths in a library, they can become quite meaningful.

After switching to `const enum`, the `CabbageColor` and `VeggieKind` declarations in the implementation look like this:

```ts
const enum CabbageColor { Red, Green }
const enum VeggieKind { Broccoli, Cabbage, Squash }
```

The compiled output for those is *nothing at all*! When we use them later, they’ll just be compiled into integers: `0` for `CabbageColor.Red` and `1` for `CabbageColor.Green` and so on.[^3]

### 2. Literal types

Next up, let’s talk about *literal* types. TypeScript allows us to specify that the type of something is an exact, specific value. For example:

```ts
type MyName = 'Chris Krycho';
```

If I specify that a given value is of the type `MyName`, it can *only* have *exactly* that value--no other string (or anything else) allowed:

```ts
let myNameBad: MyName = 'Christopher Krycho'; // TYPE ERROR
let myNameGood: MyName = 'Chris Krycho';
```

Any value you can write out as a *literal* in JavaScript can be a literal type in TypeScript. So, for example, I could get *incredibly* specify about describing myself with a type:

```ts
type MeWhenWritingThisPost = {
  name: 'Chris Krycho';
  age: 33;
  hairColor: 'brown';
};
```

Then I could specify a value (in any location: a standalone variable, a field on an object, etc.) to have exactly this type, and the compiler will enforce it! In the case of the `Veggie` example, we’ll apply this in conjunction with our newly redefined `const enum` types: both `enum` and `const enum` values are *literal values* and can be used as *literal types*. For example, we could specify that a given function can *only* operation on squash:

```ts
function onlySquash(squash: VeggieKind.Squash) {
  // ...
}

onlySquash(VeggieKind.Squash);   // Okay!
onlySquash(VeggieKind.Cabbage);  // Not okay!
onlySquash(VeggieKind.Broccoli); // Not okay!
```

### 3. Union types

The next feature we need to make our better-performing, more type-safe implementation is *union types*. This is actually the same kind of thing we were implementing in the first place from languages like Grain, Elm, Haskell, F^♯^, OCaml/ReasonML, etc.! Here, we’re going to use TypeScript’s tuples in a way that those languages *cannot* do; below, we’ll implement another version which is closer in some ways, but further in others, from the thing we see in those languages.

A union, in TypeScript, is *any* set of one or more types separated by `|` characters, representing that the type can be any *one* of the items in the set. So, for example, to specify `number` *or* `string`, we could write:

```ts
type NumOrStr = number | string;
```

That’s very similar to what we saw in the previous post for Grain’s `data` definitions, but with a key difference: this is operating on *existing* types, not just defining values of a *new* types.[^4]

Union types can be as complex as we like, and they can include literal types and tuple types and every other kind of type in the language. For example, I could define a set of kinds of vegetables I dislike by using a subset of `VeggeKind` literals:

```ts
type GrossVeggies = VeggieKind.Squash | VeggieKind.Cabbage;
```

Then a function could specify that it accepts *only* `GoodVeggies` as an argument:

```ts
function eat(veggies: GrossVeggies): string {
  switch (veggie) {
    case VeggieKind.Squash:
      return 'always ends up mushy';
    case VeggieKind.Cabbage:
      return 'the second worst part of cole slaw';
  }
}

eat(VeggieKind.Squash);   // "always ends up mushy"
eat(VeggieKind.Cabbage);  // "the second worst part of cole slaw"
eat(VeggieKind.Broccoli); // ‼️ TYPE ERROR
```

With everything we know so far, we actually have enough to solve our *type safety* problems: we could do it with object literals, using data shaped like this:

```ts
type VeggieData =
  | { kind: VeggieKind.Squash }
  | { kind: VeggieKind.Cabbage, color: CabbageColor }
  | { kind: VeggieKind.Broccoli };
```

This union type uses the literal types from `VeggieKind` to distinguish each case, and *only* includes `color` when the `kind` is specifically `VeggieKind.Cabbage`. However, while this gets us *most* of the way to where we want, we can do a little better yet on the performance front by using tuple types!

### 4. Tuple types

TypeScript uses JavaScript arrays to represent *tuples*: structured data similar to objects, but without runtime key/value associations.[^5] JavaScript already uses this pattern in a number of places, including [the `Object.entries` API](http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/entries). The syntax to define a tuple type looks like this:

```ts
type ThreeTuple = [string, number, boolean];
```

This is different from the syntax for an array which contains `string`, `number`, and `boolean`:

```ts
type MixedArrayLiteral = (string | number | boolean)[];
type MixedArrayGeneric = Array<string | number | boolean>;
```

When you have a tuple type, the position you index at corresponds to the type in that position in the tuple:

```ts
let threeTuple: ThreeTuple = ["hi", 12, true];

// These all type-check!
let first: string = threeTuple[0];  // 👍
let second: number = threeTuple[1]; // 👍
let third: boolean = threeTuple[2]; // 👍

// These will *not* type-check!
let firstBad: boolean = threeTuple[0]; // ‼️ TYPE ERROR
let secondBad: string = threeTuple[1]; // ‼️ TYPE ERROR
let thirdBad: number = threeTuple[2];  // ‼️ TYPE ERROR
```

With an array, each of these would have the type `string | number | boolean` and we would have to *check* which it was, using the `typeof` operator.

A tuple has exactly and only the length of the type defined. If we tried to access or set `threeTuple[3]`, it would be a type error--unlike with an array, which has an indefinite length.

We can also combine tuple types with literal types, to specify that only a particular value is allowed:

```ts
type Hello12 = ["hello", 12];

// This will type-check!
let allowed: Hello12 = ["hello", 12];  // 👍

// These will *not* type-check
let badFirst: Hello12 = ["greetings", 12]; // ‼️ TYPE ERROR
let badSecond: Hello12 = ["hello", 32345]; // ‼️ TYPE ERROR
let badBoth: Hello12 = ["goodbye", 98765]; // ‼️ TYPE ERROR
```

*[API]: application programming interface

## Two new implementations

With these tools in hand, we can now see a couple of ways we could approach improving both the type safety and performance of our original implementation.

### a. Just make it safer

With all of these pieces in place, we can now see how to build a version of the implementation which has the best-possible-for-JS performance characteristics.

First, we define the `CabbageColor` and `VeggieKind` types exactly as before, with `const enum`:

```ts
const enum CabbageColor { Red, Green }
const VeggieKind { Broccoli, Cabbage, Squash }
```

Then we define the underlying data for the `Veggie` type as a union of literal tuple types:[^6]

```ts
type VeggieData =
  | [VeggieKind.Squash]
  | [VeggieKind.Cabbage, CabbageColor]
  | [VeggieKind.Broccoli];
```

This particular type definition brings together *all* of the concepts discussed above, so let’s see it in practice before we pull it into the `class Veggie` definition, just to help keep things clear.

```ts
let veggieData: VeggieData;

// These will all type-check!
veggieData = [VeggieKind.Squash];  // 👍
veggieData = [VeggieKind.Broccoli];  // 👍
veggieData = [VeggieKind.Cabbage, CabbageColor.Red];  // 👍
veggieData = [VeggieKind.Cabbage, CabbageColor.Green];  // 👍

// These will *not* type-check:
veggieData = []
veggieData = [VeggieKind.Broccoli, CabbageColor.Green];  // ‼️ TYPE ERROR
veggieData = [VeggieKind.Cabbage];  // ‼️ TYPE ERROR
veggieData = [VeggieKind.Squash, 23];  // ‼️ TYPE ERROR
veggieData = [CabbageColor.Red, VeggieKind.Cabbage];  // ‼️ TYPE ERROR
```

We can now use this in conjunction with our class and `private` field to define our `Veggie` class. First, instead of defining the `kind` and `color` as separate fields, we can give it a private `data` field which uses the new `VeggieData` type.

```ts
const enum CabbageColor { Red, Green }
const enum VeggieKind { Broccoli, Cabbage, Squash }

type VeggieData =
  | [VeggieKind.Broccoli]
  | [VeggieKind.Cabbage, CabbageColor]
  | [VeggieKind.Squash];

class Veggie {
  private readonly data: VeggieData;

  private constructor(data: VeggieData) {
    this.data = data;
  }

  static Squash = new Veggie([VeggieKind.Squash]);

  static Cabbage = (color: CabbageColor) =>
    new Veggie([VeggieKind.Cabbage, color]);

  static Broccoli = new Veggie([VeggieKind.Broccoli]);

  match<Output>(matcher: {
    squash: Output,
    cabbage: (color: CabbageColor) => Output,
    broccoli: Output,
  }): Output {
    switch (this.data[0]) {
      case VeggieKind.Squash:
        return matcher.squash;
      case VeggieKind.Cabbage:
        return matcher.cabbage(this.data[1]);
      case VeggieKind.Broccoli:
        return matcher.broccoli;
    }
  }
}
```

This has a few critical differences from what we defined in the first post:

1. As with our first pass using objects instead of tuples, the types are defined in a way that means they can never be invalid. In the previous implementation, the only thing that made sure we never ended up with a `kind` of `Squash` *and* a `CabbageColor`, or with a `kind` of `Cabbage` but *without* a `CabbageColor`, was careful programming and double-checking ourselves. In the new scenario, we *cannot* create a `Veggie` with those invalid combinations, because our types won’t let us!

2. Our private constructor now takes the `VeggieData` type. This means that even the calls from our `static` definitions for `Squash`, `Cabbage`, and `Broccoli` cannot accidentally pass in the wrong thing, either! Now, this type is one we would leave private to this module, because it’s *never* something an end user would care about. Within this module, though, it lets us turn the compiler into a tool for guaranteeing that we are doing exactly and only what we intend to do.

3. Our `switch` statement can now just index directly into the tuple for the `Cabbage` scenario. Before, we needed the `!` non-null assertion operator when touching `color`, because we had no way to guarantee at the type system level that `color` was never set when it shouldn’t be and always set when it should be. Since we’ve defined `data` to be the union type `VeggieData`, though, TypeScript *knows* that `color` is always defined when `this.data.kind` is `VeggieKind.Cabbage`.

4. Similarly, if we tried to access `this.data.color` in the `case` branches for `VeggieKind.Squash` or `VeggieKind.Broccoli`, we would see a type error like this:

	> Property 'color' does not exist on type 'kind: VeggieKind.Squash;'.

5. Performance-wise, this is just about as cheap as it can get. We’re using integers to represent the different options here, and last time we even minimized the total number of these that will be constructed over the life of the app.[^7]

6. From the perspective of a *user* of the class, nothing has changed! Our public contract is identical with what we had in the previous implementation--but it’s now much more robust and we know that the type-checker will have our backs if we need to make a refactor here in the future.

*[JS]: JavaScript

:::note

It’s important to understand what types do *not* (and cannot!) buy us here as well as what they *do* buy us. They’ve helped us guarantee that we always have a valid shape to our data. But if we wire up our static constructors incorrectly, we will still end up with broken code. Nothing about our types here stops us from writing this horribly broken implementation, after all:

```ts
class Veggie {
  private data: VeggieData;

  constructor(data: VeggieData) {
    this.data = data;
  }

  static Squash = new Veggie({ kind: VeggieKind.Broccoli });

  static Cabbage = new Veggie({ kind: VeggieKind.Squash });

  static Broccoli = new Veggie({
    kind: VeggieKind.Cabbage,
    color: CabbageColor.Green
  });
}
```

This is perfectly legal from a *type* perspective, but it’s completely wrong: `Veggie.Cabbage` is a `Squash`! However, it’s also worth note that this possibility only exists because we’re cobbling together this functionality in TypeScript. If we were working in a language like Grain, Elm, Haskell, F^♯^ , OCaml/ReasonML, Rust, etc., we would *not* have this problem, because data constructors are *built into the language*!

:::

### b. Something totally different

We could also use the same technique with union types and tuple literal types, but eliminate the `class` entirely. In place of the class, we can carefully design a set of module exports to provide the same basic interface, but purely in terms of functions and values. Instead of the `Veggie` class with its private constructors, we can rename `VeggieData` to `Veggie` and expose standalone values for `Squash`, `Cabbage`, and `Broccoli` as well as for the `CabbageColor` values:

```ts
const enum CabbageColor { Red, Green }
const enum VeggieKind { Squash, Cabbage, Broccoli }

export const { Red, Green } = CabbageColor;

export type Veggie =
  | [VeggieKind.Squash]
  | [VeggieKind.Cabbage, CabbageColor]
  | [VeggieKind.Broccoli];

export const Squash = [VeggieKind.Squash];

export const Cabbage = (color: CabbageColor): Veggie =>
  [VeggieKind.Cabbage, color];

export const Broccoli = [VeggieKind.Broccoli];

export function match<Output>(veggie: Veggie, matcher: {
  squash: Output;
  cabbage: (color: CabbageColor) => Output;
  broccoli: Output;
}): Output {
  switch (veggie[0]) {
    case VeggieKind.Squash:
      return matcher.squash;
    case VeggieKind.Cabbage:
      return matcher.cabbage(veggie[1]);
    case VeggieKind.Broccoli:
      return matcher.broccoli;
  }
}
```

We could import it and use this implementation like this:

```ts
import { Cabbage, Squash, Broccoli, Red, Green, match } from './veggie';

let veggies = [Cabbage(Red), Squash, Broccoli, Squash, Cabbage(Green)];

let descriptions = veggies.map((veggie) => match(veggie, {
  squash: 'green, yellow, you name it!',
  cabbage: (color) => `this cabbage is ${color}`,
  broccoli: 'always just green',
});
```

This is the same kind of performance we had with the `class`… but with *even less* overhead, since there is no class sitting around. (A class doesn’t cost much, though, and this decrease is rather trivial given the optimizations we’ve already made.) Notice, though, that at this point we have very nearly the same syntax we had in the original example from Grain--we just had to do a bit more work to create the names to get there:

```Grain
data CabbageColor = Red | Green 
data Veggie =
  | Squash
  | Cabbage(CabbageColor)
  | Broccoli
```

```Grain
import * from './veggie';

let colorToString = (color) => match (color) {
  | Red => 'red'
  | Green => 'green'
}

let veggies = [Cabbage(Red), Squash, Broccoli, Squash, Cabbage(Green)]

let descriptions = List.map((veggies) => match (veggies) {
  | Squash => 'green, yellow, you name it!'
  | Cabbage(color) => 'this cabbage is ' + colorToString(color)
  | Broccoli => 'always just green'
});
```

### Evaluation

There is one significant downside to this implementation compared to the class implementation. We have coupled it to the use of the tuple types as values, by exposing that information to end users. In the `class`-based implementation, the structure of the data was `private`, and in fact we could have used a [private class field](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Classes/Private_class_fields) to make it not only type-private but runtime-private, so that end users could not see the details even if they wanted to.

What’s more, because we’ve exposed those implementation details, we’ve actually given away our control over the values in the system. It’s now perfectly legal from a type system perspective for a user to create their own instance of a red cabbage manually, using the substituted values of the `const enum`:

```ts
let redCabbage: Veggie = [1, 0];
```

Users *shouldn’t* do this, but that doesn’t mean they *won’t*. The same goes for the "tuples" we now expose to them: users *shouldn’t* treat those as arrays with all their normal operations and transformations, but they *can*.

Given the extremely low overhead that a single class provides, I think the balance here leans strongly toward using a class. Remember, `Veggie` is effectively a singleton because it cannot be constructed apart from the constructors we supplied! And in fact, we could go further and make it *impossible* for outside callers to construct it apart from those constructors. Instead of exporting the class, we could just export the constructors (a pattern we’ve seen already for *other* reasons in this example):

```ts
class Veggie {
  // ...
}

export const { Squash, Cabbage, Broccoli } = Veggie;
export type Veggie = typeof Veggie;
```

In the end, I think this pattern is preferable: it gives us the best balance of developer ergonomics, safety, and performance. This actually surprised me a bit: when I started writing up these two posts, I fully expected to end up preferring the `class`-less approach, but evaluating the tradeoffs led me to like the `class`-based approach a bit better![^8]

## Bonus: one last optimization

Finally, for *both* implementations, there’s one further (tiny) optimization we could make performance-wise. As I noted in the previous post in this series when discussing the `Squash` and `Broccoli` constructors on `Veggie`, we can freely reuse those values because they’re immutable, so we don’t have to worry about changes to them confusing our system. The same actually goes for our `Cabbage` variant! We *know* that we there will only ever be two options there: the two colors. Accordingly, we can define *values* for those and use them directly instead of creating new arrays every time the constructor is used:

```ts
class Veggie {
  private static RedCabbage =
    new Veggie([VeggieKind.Cabbage, CabbageColor.Red]);
  private static GreenCabbage =
    new Veggie([VeggieKind.Cabbage, CabbageColor.Green]);

  static Cabbage = (color: CabbageColor): Veggie =>
    color === CabbageColor.Red
      ? Veggie.RedCabbage
      : Veggie.GreenCabbage;
```

With that, there are only ever a *total* of four actual `Veggie` instances in the entire system, with the following `Data`:

1. `[VeggieKind.Squash]`
2. `[VeggieKind.Cabbage, CabbageColor.Red]`
3. `[VeggieKind.Cabbage, CabbageColor.Green]`
4. `[VeggieKind.Broccoli]`

This is as cheap as things can get! On the one hand, it’s quite delightful that we *can* get here with TypeScript by thinking through every possible optimization point in our system. On the other hand, it’s the kind of thing we could just get for *free* if we were using a language that had these ideas built in from the outset.

[^1]: Note: only available when using TS to compile your code! If you’re using Babel to compile and only using TS to type-check, this doesn’t work. Compiling out `const enum` declarations requires having information about more than one file; Babel explicitly only works to transform the syntax of a single file.

[^2]: Why this is necessary, I don’t know. I have never found a compelling use case for it!

[^3]: You might worry about whether this means that you can also substitute `VeggieKind.Broccoli` for `CabbageColor.Red`, since they’d both just have the value `0` at runtime. The answer is *no*: unlike most places in TypeScript, where the ultimate "shape" is the only thing which matters, enums are treated as distinct types based on their *name*. You can see this distinction in practice in [this playground](https://www.typescriptlang.org/play?#code/MYewdgzgLgBApmArgWxgeTHGBvGBBAGhgCEYBfAKFElgRRgBUB3EHfI0yigM0TGCgBLcDG4gQACgAeALnSYAlDkq9+QkQCMAhgCdpc5iCXYuYyYYB0eBQG4K2vRjhXbFIA).

[^4]: TypeScript’s types are *sets* in the mathematical sense. A lot of otherwise-surprising things about the type system flow out of the set-natured-ness of the types. For example, when you see `any`, you’re actually seeing the *everything* set--which means that if you ever see any other type combined with `any` in a union, like `number | any`, it’s pointless. You can think of it this way: if we were talking about sets of *numbers*, and we said "This value can be 1 or any number," the first bit doesn’t matter, since "1" is included in "any number." The same thing goes with `any`.

[^5]: I qualify *runtime* key-value associations because TypeScript 4 is introducing the ability to use labels with tuples. As with essentially all TypeScript features--except non-`const` enums!--these have no existence at runtime.

[^6]: With TypeScript 4.0, we could actually use labeled values for this tuples. The result would be quite expressive while maintaining exactly the semantics we need *and* the nice performance characteristics of tuple types:

	```ts
	type VeggieData =
	  | [kind: Kind.Squash]
	  | [kind: Kind.Cabbage, color: CabbageColor]
	  | [kind: Kind.Broccoli]
	````

	I’ve left the code sample without those, but if I were writing this in a codebase *today*, that’s how I would write it!

[^7]: Strictly speaking, we can actually go slightly further on that front, by statically creating the cabbage variants as well:

	```ts
	const RedCabbageData: VeggieData = [VeggieKind.Cabbage, CabbageColor.Red];
	const GreenCabbageData: VeggieData = [VeggieKind.Cabbage, CabbageColor.Green];
	```

	Then we could use that in our static constructor:

	
    ```ts
	class Veggie {
	  // ...
	  static Cabbage = (color: CabbageColor) =\>
	    color == CabbageColor.Red
	      ? RedCabbageData
	      : GreenCabbageData;
	}
	```

[^8]: Fellow functional programming fans: consider this your friendly reminder that classes are just a useful language construct, and don’t *inherently* require you to use or support using them for inheritance!
