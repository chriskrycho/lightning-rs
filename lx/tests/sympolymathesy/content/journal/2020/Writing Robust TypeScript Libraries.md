---
title: Writing Robust TypeScript Libraries
subtitle: >
  A subtler art than it might at first appear, if you intend to support JS or even loose mode TS.
summary: >
  When writing a TypeScript library, it’s helpful to validate all the data passed into the library—at least in development—so that your library robustly handles the kinds of bad data it may receive from JavaScript and loose-mode TypeScript consumers.
qualifiers:
  audience: >
    Software developers working with (or interested in) TypeScript, particularly those who are shipping libraries for other developers to use.

date: 2020-10-24T15:45:00-0600
updated: 2020-12-16T08:15:00-0600

tags:
  - software development
  - web development
  - programming languages
  - JavaScript
  - TypeScript

---

:::callout

The folks at InfoQ China translated this article into Chinese, you can read that translation [here](https://www.infoq.cn/article/LCkmyl8xdQA8W4BkHlHI)!

:::

When authoring a library in TypeScript, you often do not know how that library will ultimately be consumed. Even if you [warn](https://github.com/true-myth/true-myth#design-philosophy "True Myth: Design Philosophy") would-be users that you intend it *only* for TypeScript consumers, you are very likely to end up with JavaScript users at some point—either because they use it despite your warning or because they end up consuming it as a [transitive dependency](https://en.wikipedia.org/wiki/Transitive_dependency). This has an important consequence: you must design the library to be consumed by developers working in either language![^1]

The primary place this comes into play is function definitions and bodies. If you were writing for a pure TypeScript audience, you would simply define the types for the function and trust the compiler to handle the rest. If you were writing for a pure JavaScript audience, you would *document* those types, but treat the actual type within your function as `unknown` and check whatever your caller passed.

For example, given this code—

```ts
interface Person {
  age: number;
  name?: string;
}

function describe(person: Person): string {
  let name = person.name ?? 'someone';
  return `${name} is ${person.age} years old!`;
}
```

—a JS consumer of your library could call the `describe` function with *literally anything*. That goes from the almost-right—

```ts
describe({ name: "chris" })
```

—to the catastrophically wrong—

```ts
describe("potato");
```

—and of course to our favorite JS mistake:

```ts
describe(undefined);
```

(How else could we get one of our favorite JS errors, *undefined is not an object*?)

Now, it’s not that JS users of your library would do this on purpose. To the contrary! It’s that in any sufficiently large system, it’s *easy* to end up passing a wrong argument to a function somewhere in the system. It’s usually a matter of hard-to-avoid mistakes, like making a change in one spot and getting *most* of the other spots that needed to be updated, but missing one. The best-intentioned JS developers *will* send bad data into your beautifully-designed TS API.

Now, I skipped over something above. When I said—

> If you were writing for a pure TypeScript audience, you would simply define the types for the function and trust the compiler to handle the rest.

—I intentionally left aside the fact that the TypeScript compiler allows an *enormous* range of strictness, from checking you at a level that is really no different from JavaScript up to nearly as strict as anyone could like.[^2] That means that even TypeScript callers should be treated with the same level of trust as JavaScript callers: for all you know, they’re throwing `any` around like there’s no tomorrow and are gleefully ignoring all the places things might in fact be `null` or `undefined`. To return to the sample code above:

```ts
interface Person {
  age: number;
  name?: string;
}

function describe(person: Person): string {
  let name = person.name ?? 'someone';
  return `${name} is ${person.age} years old!`;
}
```

A TypeScript consumer operating with *no* strictness flags enabled could very well call `describe` like this:

```ts
function cueTheSobbing(data: any) {
  describe(data);
}

cueTheSobbing({ breakfastOf: ["eggs", "waffles"] });
```

Or this:

```ts
describe(null);
```

Or this:

```ts
describe({ age: null })
```

That is: *most* of the ways that a JS caller could get it wrong, a TS caller with the strictness settings turned off could *also* get it wrong.[^3] (You can see all of these “working” in [this TypeScript playground](https://www.typescriptlang.org/play?noImplicitAny=false&strictNullChecks=false&strictFunctionTypes=false&strictPropertyInitialization=false&strictBindCallApply=false&noImplicitThis=false&noImplicitReturns=false&alwaysStrict=false#code/JYOwLgpgTgZghgYwgAgArQM4HsTIN4BQyxycA5hAFzIgCuAtgEbQDcRJIc9EA-NRmCigybAL4ECAEwgIANnCgoYtEAjDAcyaRgRDmACgAOmHNXRRsIAJT9BwtlIg69EfXVmyrbbbuAGV0jCgEJJejs5+rnikFNTussiiVhLKquqaCLQQACoAFhAAyliMjML6knBgcFb47MQ+LuWV1WISmTn5RSVlAOSyWAm0UMi5WBghPWHteYXFpSBk+tGMinAA1vACALJw0gDyMNQA2j0QZGQYPQA0yD0A7nAwMLJOPQC6iVNZM13zi-FeIA)!) This means that the best-intentioned *TypeScript* consumer may *also* call your library with bad data. And depending on what other libraries *they’re* relying on, it may not even be their fault, because this kind of thing can happen anywhere in the dependency graph.

So if the problem is that we can’t trust the data, what should we do about it? One option would to make all parameters to the function actually be `unknown`, and specify how it should behave with [JSDoc](https://jsdoc.app). That, however, would lose us a great deal of the utility TS offers. We wouldn’t get completions or type errors even internally when interacting with the function, and neither would any of our library’s consumers. But as we’ve just seen, we can’t rely on the type definitions to provide safety within the body of a function, either. We can combine these approaches, though: specify the type definition, *and* treat the data that comes in as if it were actually `unknown`. This does come with runtime overhead—we’ll come back to the tradeoffs around that in a few paragraphs. For now, we can just start by seeing how to check the types.

First, we’ll author our code as if we were actually going to get truly unknown data from callers, since we’ve established that that’s exactly what we might get. Once we finish validating the data safely for `unknown`, we’ll be able to replace it with `Person` and everything should continue working, but now we’ll have a guarantee that it works for *any* data thrown at it.

```ts
function describe(person: unknown): string {
  let name = person.name ?? 'someone';
  return `${name} is ${person.age} years old`;
}
```

This has type errors ([playground](https://www.typescriptlang.org/play?#code/JYOwLgpgTgZghgYwgAgArQM4HsTIN4BQyxycA5hAFzIgCuAtgEbQDcRJIc9EA-NRmCigybAL4ECMWiARhgOZABMIGBEOYAKAA6Yc1aQGsQWAO4gAlPnbEANhDA0uKALzIdUbCAB0nbsh48yADk2Nw4EEFsJMhQ9rRQuAAGACR4vhCiyMAYyKnunl7kGcgAnhBwHshYNoqJYgRAA)), since the `person` type here could be `undefined` or `"potato"` or anything else. We can use TypeScript’s notion of [type narrowing](https://www.typescriptlang.org/docs/handbook/2/narrowing.html) to make this safe. However, narrowing from `unknown` to a specific object type is a little bit funky, because if you simply check if `typeof somethingUnknown === 'object'`, it will narrow it to the type `{}`, which means it will *not* include any of the possible types we need. We’ll start by defining an `isObject` helper function which will give us the right semantics:

```ts
function isObject(
  maybeObj: unknown
): maybeObj is Record<string | number | symbol, unknown> {
  return typeof maybeObj === 'object' && maybeObj !== null;
}
```

We also need a way to check if the object as a given key on it. It would be nice if [the `in` operator](http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/in) worked this way, but unfortunately [it does not](https://github.com/microsoft/TypeScript/issues/21732). We could do this inline, too, but that would require a cast every time. We can call that `has`, similar to [the `Object.hasOwnProperty` method](http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/hasOwnProperty). Since this *also* needs to check against the same set of types that our `isObject` type returned—all the valid types for indexing an object in JS—we’ll extract that into a new `Key` type here as well. This `has` helper’s return type tells the type system that the item passed in has its initial type *and* that it includes the key we checked for, if the body is true.

```ts
type Key = string | number | symbol;

function has<K extends Key, T>(
  key: K,
  t: T
): t is T & Record<K, unknown> {
  return key in t;
}
```

We can combine those into a [type guard](https://www.typescriptlang.org/docs/handbook/2/narrowing.html) to check whether a given object is a person:

```ts
function isPerson(value: unknown): value is Person {
  return (
    isObject(value) &&
    has('age', value) && typeof value.age === 'number' &&
    (has('name', value) ? typeof value.name === 'string' : true)
  )
}
```

Next, we can assemble all of these into a simple check at the top of our function, and throw a useful error if it isn’t valid. (You can see this working in [this playground](https://www.typescriptlang.org/play?#code/JYOwLgpgTgZghgYwgAgArQM4HsTIN4BQyxycA5hAFzIgCuAtgEbQDcRJIc9EA-NRmCigybAL4ECYAJ4AHFAGkIU5AF5kAoSDLIAPjQbMou9VKZYANmwIxaIBGGA5kwDAHlGAKwj2AFPThSzO4e1LYA1iBYAO4gAJTU-oEQwc4YyABK3lhQACYAPIpSADTI4ZExAHz47MRQEGC0ULjSclgwyIlBnqoqagDkWJ7eYH3IAGRjHQFdHsgAhL365pYE4ta29o64ABZwGAXIEAAekCA5aYUlACoVPmFK1PIlAG5w5rRUyFfxyK-vKC4vuMMllcgUSmVoiAqoQSMg6g0msh7spQL83h8xBIbHYHE4XOgoNgQD4-h9QiAIlCfmSAWlCcTqnCEY1cD4anDUsFhqSMRBYuMxhy4bsMD4+uQIH0XnyBRNkC0IG10f8AHSSnr9OhMaCjCbCkg+UXizjcaUqj4CngK2RK9q01WmlC9foaYSjaiCS0c2JY9a4rbIHIQDAIITMHxyIk4ClUmIC2EkYDtHxzAmYHCRjNxBMG4hgbZQaI0CBRZAAUSgRag4oABsHQ+GILX4RAAI60YB1NJSLC0BVYZAyPZpODIWsMnC1vqxDlrOHmeo0LjOofZx0r5A8a19bDcHBStjM+qs8cAEjwTtEqWQF6jxPVFGvUggcCJyAsOVrWKAA).)

```ts
function describe(person: unknown): string {
  if (!isPerson(person)) {
    throw new Error('`describe` requires you to pass a `Person`');
  }

  let name = person.name ?? 'someone';
  return `${name} is ${person.age} years old`;
}
```

Now that we have this in place, we can update the type of `person` here to be `Person` to make the experience better for TypeScript consumers. Everything still type-checks, as it should ([playground](https://www.typescriptlang.org/play?#code/JYOwLgpgTgZghgYwgAgArQM4HsTIN4BQyxycA5hAFzIgCuAtgEbQDcRJIc9EA-NRmCigybAL4ECYAJ4AHFAGkIU5AF5kAoSDLIAPjQbMou9VKZYANmwIxaIBGGA5kwDAHlGAKwj2AFPThSzO4e1LYA1iBYAO4gAJTU-oEQwc4YyABK3lhQACYAPIpSADTI4ZExAHz47MRQEGC0ULjSclgwyIlBnqoqagDkWJ7eYH3IAGRjHQFdHsgAhL365pYE4ta29o64ABZwGAXIEAAekCA5aYUlACoVPmFK1PIlAG5w5rRUyFfxyK-vKC4vuMMllcgUSmVoiAqoQSMg6g0msh7spQL83h8xBIbHYHE4XOgoNgQD4-h9QiAIlCfmSAWlCcTqnCEY1cD4anDUsFhqSMRBYuMxhy4bsMD4+uQIH0XnyBRNkC0IG10f8AHSSnr9OhMaCjCbCkg+UXizjcaUqj4CngK2RK9q01WmlC9foaYSjaiCS0c2JY9a4rbIHIQDAIITMHxyIk4agMnAC2EkYDtHxzAmYHCRjNxBMG4hgbZQaI0CBRZAAUSgRag4oABsHQ+GILX4RAAI60YB1NJSLC0BVYZAyPZpODIWtxkC1vqxDlrOHmeo0LjOofZx0r5A8a19bDcHBStjM+qs8cAEjwTtEqWQF6jxPVFGvUggcCJyAsOVrWKAA)).

```ts
function describe(person: Person): string {
  if (!isPerson(person)) {
    throw new Error(
      `'describe' takes a 'Person', but you passed ${JSON.stringify(person)}`
    );
  }

  let name = person.name ?? 'someone';
  return `${name} is ${person.age} years old`;
}
```

This is so useful that TypeScript supports a generalization of this pattern of throwing when a condition does not hold: [assertion functions](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-3-7.html#assertion-functions). We can write a general form like this:

```ts
function assert(
  predicate: unknown,
  message: string
): asserts predicate {
  if (!pred) {
    throw new Error(message);
  }
}
```

With the assertion function in place, our `describe` function gets even simpler:

```ts
function describe(person: Person): string {
  assert(
    isPerson(person),
    `'describe' takes a 'Person', but you passed ${JSON.stringify(person)}`
  );

  let name = person.name ?? 'someone';
  return `${name} is ${person.age} years old`;
}
```

So far so good! We now guarantee that no matter who calls `describe`, whether from JS, or from loosely typed TS, or from some other language entirely, it will always do the “right” thing, by providing an actionable error to the caller if they did something wrong. However, depending on our constraints, this kind of runtime validation could be too expensive to be viable. In a browser, the extra code we send across the wire adds up: it’s more to download and more to parse, both of which slow down our app. In *any* environment, it’s extra runtime checks every time we interact with our `describe` function.

<aside>

For this reason, you should generally do this kind of data checking at the edge of your system, so that *within* your system, everything can be well-typed, but you only have to pay the costs once. (Alexis King’s post [Parse, Don’t Validate](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/) covers this wonderfully.) That pattern works better for apps than for libraries, though, since libraries often have a very large of number of relatively arbitrary entry points.

</aside>

One option is to leverage some compilation (or “transpilation”) smarts to provide these checks during development but *not* in production builds. Babel allows you to turn given functions into noops, making them not-quite-free but *extremely cheap*. For example, Ember CLI supplies a Babel plugin that turns Ember’s `assert` function (which is typed nearly identically to the `assert` I defined above) into no-ops. You can combine this with any bundler that can do dead-code elimination to remove all the unused helpers as well!

The downside to this approach is that production errors will have worse error messages and be harder to debug. The upside is that you will ship much less code and pay much lower runtime costs in production. To make code relying on this kind of `assert`-stripping work *well*, end users need to combine it with good end-to-end test coverage of any given feature, UI component, etc. But that is true regardless: types and tests eliminate different classes of bugs, and are best used in conjunction!

:::callout

Thoughts and comments? [Email me](mailto:hello@chriskrycho.com?subject=Writing%20Robust%20TypeScript%20Libraries) or comment on [HN] or [Lobste.rs].

:::

[HN]: https://news.ycombinator.com/item?id=24882225
[Lobste.rs]: https://lobste.rs/s/j696bv/writing_robust_typescript_libraries

*[JS]: JavaScript
*[TS]: TypeScript
*[API]: application programming interface
*[CLI]: command line interface
*[UI]: user interface

[^1]: Other languages also consume TS libraries these days, including ReScript and ReasonML, PureScript, Elm, and others.

[^2]: “Nearly,” I say, because I personally would like a *truly* strict mode. I sometimes wish for a `strict: 11` variant in the future which requires that *every* type from a non-strict-TypeScript library be checked before being trusted, along with a commitment to soundness in the type system.

    Given that a sound type system is [explicitly a non-goal for TypeScript](https://github.com/Microsoft/TypeScript/wiki/TypeScript-Design-Goals#non-goals), I’m not holding my breath. There’s a reason I sometimes look very longingly at the type systems of ReasonML, Elm, Haskell, PureScript, F^♯^, etc.

[^3]: For this reason, I actually think that if you’re not using `strict: true`—or the equivalent with individual flags for phased adoption of new versions of the compiler—you’re actually better off just using JavaScript with [JSDoc](https://jsdoc.app) annotations and a `jsconfig.json` file.
