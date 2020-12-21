---
title: Async Data and Autotracking in Ember Octane
subtitle: >
  Digging into the `load` helper and `AsyncData` type I introduced in an earlier post.
summary: >
  Digging into the `load` helper and `AsyncData` type I introduced in an earlier post, as a way of showing how to think about auto-tracking and asynchronous data.
qualifiers:
  audience:
    Software developers working with Ember Octane.

date: 2020-08-28T15:15:00-0600
updated: 2020-08-29T09:45:00-0600

tags:
  - JavaScript
  - software development
  - web development
  - Ember
  - auto-tracking

templateEngineOverride: md

---

Last week, [I described](https://v5.chriskrycho.com/journal/migrating-off-of-promiseproxymixin-in-ember-octane/ "Migrating Off of PromiseProxyMixin in Ember Octane") the use of a `load` helper and associated `AsyncData` type to move away from Ember’s `ObjectProxy` and `PromiseProxyMixin`. In this post, I’ll dig into the implementation of `load` and `AsyncData`. When you get to the end of this post, you should not only understand how this particular helper and data type work, but also have a better idea of how to think about both handling asynchronous data in JavaScript in general *and* how to put that to practice in Ember Octane with autotracking specifically.

<!-- omit in toc -->
## Overview

- [Philosophy](#philosophy)
    - [1. Async data is just data](#1-async-data-is-just-data)
    - [2. Handling all data states is important](#2-handling-all-data-states-is-important)
- [Implementation](#implementation)
    - [Make a helper](#make-a-helper)
    - [Modeling the states](#modeling-the-states)
    - [Updating the state](#updating-the-state)
        - [Add state change methods to `AsyncData`](#add-state-change-methods-to-asyncdata)
        - [Connect the `Promise` and `AsyncData` in `load`](#connect-the-promise-and-asyncdata-in-load)
    - [Return the same `AsyncData`](#return-the-same-asyncdata)
- [Conclusion](#conclusion)
- [Appendix: TypeScript](#appendix-typescript)

## Philosophy

Before we dig into the details of how `load` and `AsyncData` work, it’s worth understanding the philosophy behind them. When some colleagues and I built these helpers, it was with two key ideas in mind—so it’s worth understanding these ideas as we work through the implementation:

1. *Async data is just data*
2. *Handling all data states is important*

### 1. Async data is just data

The whole point of this data type is to make it possible for end users to call `load` on a `Promise` and then interact with it exactly like any other piece of data. This sometimes feels strange to people, *especially* when it’s used to define the return type of a getter. But while there are additional considerations when dealing with asynchronously loaded data as we do here, one of the key value propositions of `Promises` (and similar features in other languages, whether they go named `Task` or `Future` or something else entirely) is that they allow you to represent an asynchronous computation as *data*. This allows you to interact with that the same way you would with any *other* kind of data. The `AsyncData` type we work with here is just an extension of that same idea!

### 2. Handling all data states is important

When we’re dealing with *any* data, it’s important to understand the possible states it can be in, to guarantee that a given representation of that data cannot end up in *invalid* states, and—optimally—to make it so that we always handle all those states. The `AsyncData` type we’ll build in the rest of this post is designed to do all three of those.

Fundamentally, an asynchronous data loading operation can be in at least three states: *loading*, *loaded*, or *error*. I say “at least” because there are other states you might care about, as well: *not started* and *slow* in particular. For our purposes I’ll leave those other options aside: in most cases *loading* and *not started* end up in the same place for users and *slow* is a variant of *loading*. In your particular scenarios, though, you might have different tradeoffs!

It’s important particularly—though not only!—in dealing with user interfaces that we account for *all* of these scenarios. If we don’t, we might end up assuming that our data is always in a loaded state, and fail to show anything meaningful while it’s loading, or if there’s an error. At *best* this makes for a poor user experience. At worst, the result can be outright buggy! So this implementation exposes (and encourages you to think in terms of!) those states: `loading`, `loaded`, and `error`.[^1]

:::note

This idea is far from original to me or my colleagues. I learned it from a series of talks and blog posts around the idea of “making illegal states impossible,” an idea which has a lot of traction in the typed functional programming community.

:::

## Implementation

Let’s start building, keeping these core ideas in mind.

1. *Async data is just data*
2. *Handling all data states is important*

As we're implementing, then—

- We need to *model* the three states of the data: *loading*, *loaded*, and *error*.
- We need to *update* the state of the data when the promise resolves or rejects.
- If users call `load` with the same `Promise`, we should always return the same `AsyncData`.

That's a lot! If we take it step by step, though, it won't seem so bad, so we'll tackle the implementation in phases:

1. Make a helper
2. Model the states
3. Update the state

### Make a helper

We’re going to build this as an Ember helper, so that it can be used in templates. The easiest way for us to get started is to use Ember <abbr title="command line interface">CLI</abbr>’s generator:

```bash
ember generate helper load
```

The result is this file:

```js
import { helper } from '@ember/component/helper';

export default helper(function load(params/*, hash*/) {
  return params;
});
```

We’re going to start by pulling the `load` function out of the `helper` invocation so that we can use it in JavaScript:

```diff
  import { helper } from '@ember/component/helper';


- export default helper(function load(params/*, hash*/) {
-   return params;
- });
+ export function load(params/*, hash*/) {
+   return params;
+ }
+
+ export default helper(load);
```

Using it in another JS module would look like this:

```js
import { load } from 'my-app/helpers/load';
```

Now, we know that we want to represent the state of any given promise, so we’ll change the definition of the `load` function accordingly. For now, we’ll just have the helper return the promise passed in, and we’ll figure out what we actually want to return in a minute.

```diff
  import { helper } from '@ember/component/helper';

- export function load(params/*, hash*/) {
+ export function load(somePromise) {
+   // this isn't helpful, but we'll come back to it!
-   return params;
+   return somePromise;
  }

  export default helper(load);
```

This would blow up if we tried to actually use the helper in a template, though! Helpers expect their first argument to be an array of the positional arguments to the helper. Since we’ll only ever want to load one promise at a time with this helper, we can fix that pretty easily, by reworking how we connect the function definition to the helper:

```diff
  import { helper } from '@ember/component/helper';

  export function load(somePromise) {
    // this isn't helpful, but we'll come back to it!
    return somePromise;
  }

- export default helper(load);
+ export default helper(([promise]) => load(promise));
```

Now we need to think about what we want to return.

### Modeling the states

We can start by defining a type which will represent the state of the data. We’ll use a `class` here because it’s a really convenient tool for defining data structures in JS—but we don’t intend for this to be subclassed, and so we’re not exporting it from our module.

```diff
  import { helper } from '@ember/component/helper';

+ class AsyncData {
+   /** @type {'LOADING' | 'LOADED' | 'ERROR'} */
+   state;
+ }

  export function load(somePromise) {
    // this isn't helpful, but we'll come back to it!
    return somePromise;
  }

  export default helper(([promise]) => load(promise));
```

Notice that I’ve added a `@type` annotation in the JSDoc comment here. This will give us nice autocompletion and feedback for many editors, and if any users are using TypeScript directly *or* type-checking their JavaScript with TypeScript’s `checkJS` mode or `// @ts-check` notation, they’ll get errors if they try to compare the `AsyncData` type’s state to any value *besides* `'LOADING'`, `'LOADED'`, or `'ERROR'`.

Initially, the state should always default to `'LOADING'`, since we don’t *know* the state of the promise we’ll consume:

```diff
  import { helper } from '@ember/component/helper';

  class AsyncData {
    /** @type {'LOADING' | 'LOADED' | 'ERROR'} */
-   state;
+   state = 'LOADING';
  }

  export function load(somePromise) {
    // this isn't helpful, but we'll come back to it!
    return somePromise;
  }

  export default helper(([promise]) => load(promise));
```

Now we can create an `AsyncData` and return it from our `load` helper. This is completely useless for the moment, as it will just always remain in `'LOADING'`, but we’ll fix that shortly.

```diff
  import { helper } from '@ember/component/helper';

  class AsyncData {
    /** @type {'LOADING' | 'LOADED' | 'ERROR'} */
-   state;
+   state = 'LOADING';
  }

  export function load(somePromise) {
    // this isn't helpful, but we'll come back to it!
-   return somePromise;
+   return new AsyncData();
  }

  export default helper(([promise]) => load(promise));
```

We’ll also need to track the value of the resolved data for when promises resolve (the *value*) and the rejection reason when the reject (the *error*). We could do this by just adding two new fields to the class, one for each of those two outcomes:

```diff
  import { helper } from '@ember/component/helper';

  class AsyncData {
    /** @type {'LOADING' | 'LOADED' | 'ERROR'} */
    state = 'LOADING';
+
+   value;
+
+   error;
  }

  export function load(somePromise) {
    // this isn't helpful, but we'll come back to it!
    return new AsyncData();
  }

  export default helper(([promise]) => load(promise));
```

However, if we do this, and especially if we make it part of our public API, we are committing ourselves to *never* changing the way we manage our state. But there are ways we could make this more robust for ourselves in the future, especially if we were using TypeScript. For example, we might want our internal state to be `{ state: 'LOADING' }` *or* `{ state: 'LOADED', value }` *or* `{ state: 'ERROR', reason }`, so that we could never accidentally end up with a `state` of `'ERROR'` but have assigned `value` instead of `error`.

Our best bet for now is to make the fields “private” and expose a getter for each of these instead, so that end users can’t write to it. (We’d *like* to use [private class fields](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Classes/Private_class_fields), but they’re incompatible with decorators, and that will be important when we *update* the state.[^2]) This will also make it so that if we want to refactor to something like that set of alternate types in the future, we *can*. We’ll make `state` “private” as well.

```diff
  import { helper } from '@ember/component/helper';

  class AsyncData {
-   /** @type {'LOADING' | 'LOADED' | 'ERROR'} */
+   /**
+     @type {'LOADING' | 'LOADED' | 'ERROR'}
+     @private
+    */
-   state = 'LOADING';
+   _state = 'LOADING';
+
+   /** @private */
+   _value;
+
+   /** @private */
+   _error;
+
+   get state() {
+     return this._state;
+   }
+
+   get value() {
+     return this._value;
+   }
+
+   get error() {
+     return this._error;
+   }
  }

  export function load(somePromise) {
    // this isn't helpful, but we'll come back to it!
    return new AsyncData();
  }

  export default helper(([promise]) => load(promise));
```

This also lets us do a bit of runtime validation if we like: we can enforce that users *check the state* and make sure it’s valid before they try to get the `value` or `error` types, using assertions that only run in development or test:[^3]

```diff
  import { helper } from '@ember/component/helper';
+ import { assert } from '@ember/debug';

  class AsyncData {
    /**
      @type {'LOADING' | 'LOADED' | 'ERROR'}
      @private
     */
    _state = 'LOADING';
 
    /** @private */
    _value;
 
    /** @private */
    _error;
 
    get state() {
      return this._state;
    }
 
    get value() {
+     assert(
+       `You can only access 'value' when 'state' is 'LOADED', but it is ${this.state}`,
+       this.state === 'LOADED'
+     );
+
      return this._value;
    }
 
    get error() {
+     assert(
+       `You can only access 'error' when 'state' is 'ERROR', but it is ${this.state}`,
+       this.state === 'ERROR'
+     );
+
      return this._error;
    }
  }

  export function load(somePromise) {
    // this isn't helpful, but we'll come back to it!
    return new AsyncData();
  }

  export default helper(([promise]) => load(promise));
```

This way, users will *only* be able to get the `value` when `state` is `'LOADED'`.

At this point, we have a robust representation of the state and a way to expose the value of the resolved promise or the reason it rejected… but we don’t have a way to actually *change* the state or *set* the `value` or `error` properties. Let’s see how to do that!

### Updating the state

First things first, we need to make the state reactive so that it will work in the template, using `@tracked`:

```diff
  import { helper } from '@ember/component/helper';
  import { assert } from '@ember/debug';
+ import { tracked } from '@glimmer/tracking';

  class AsyncData {
    /**
      @type {'LOADING' | 'LOADED' | 'ERROR'}
      @private
     */
-   _state = 'LOADING';
+   @tracked _state = 'LOADING';
 
    /** @private */
-   _value;
+   @tracked _value;
 
    /** @private */
-   _error;
+   @tracked _error;
 
    get state() {
      return this._state;
    }
 
    get value() {
      assert(
        `You can only access 'value' when 'state' is 'LOADED', but it is ${this.state}`,
        this.state === 'LOADED'
      );
 
      return this._value;
    }
 
    get error() {
      assert(
        `You can only access 'error' when 'state' is 'ERROR', but it is ${this.state}`,
        this.state === 'ERROR'
      );
 
      return this._error;
    }
  }

  export function load(somePromise) {
    // this isn't helpful, but we'll come back to it!
    return new AsyncData();
  }

  export default helper(([promise]) => load(promise));
```

Now, any end user of the code—whether a JavaScript getter or a reference in the template—will correctly update when any of these values change *or* any getter which references them changes.

On the JavaScript side, we can define data in terms of the result of calling `load`—including in a getter. Since `args` are autotracked and `AsyncData` autotracks its internals, the `data` getter here will rerun any time `args.userId` changes and the `displayData` getter will rerun when the result of `data` changes (as long as `displayData` is used in the template).

```js
import Component from '@glimmer/component';
import { load } from 'my-app/helpers/load';
import { fetchSomeData } from 'my-app/data/fetchers';

export default class Neato extends Component {
  get data() {
    return load(fetchSomeData(this.args.userId));
  }

  get displayData() {
    switch (this.data.state) {
      case 'LOADING':
        return 'loading...';
      case 'LOADED':
        return this.data.value;
      case 'ERROR':
        return `Whoops! Something went wrong! ${this.data.error.message}`;
    }
  }
}
```

Similarly, if we had a component which had a promise passed into it and used `load` as a helper:

```handlebars
{{#let (load @somePromise) as |data|}}
  {{#if (eq data.state 'LOADING')}}
    <p>{{data.value}}</p>
  {{else if (eq data.state 'LOADED')}}
    <p>loading...</p>
  {{else if (eq data.state 'ERROR')}}
    <p>Whoops! Something went wrong!</p>
    <p>{{data.error.message}}</p>
  {{/if}}
{{/let}}
```

Using `eq` here to match the state with strings is a little cumbersome. It would be better if we could just check `{{#if data.isLoaded}}` and so on. We can do that easily enough by exposing a convenience getter for each state on `AsyncData`:

```diff
  import { helper } from '@ember/component/helper';
  import { assert } from '@ember/debug';
  import { tracked } from '@glimmer/tracking';

  class AsyncData {
    // SNIP: internal state and other getters are unchanged...
+
+   get isLoading() {
+     return this.state === 'LOADING';
+   }
+
+   get isLoaded() {
+     return this.state === 'LOADED';
+   }
+
+   get isError() {
+     return this.state === 'ERROR';
+   }
  }

  export function load(somePromise) {
    // this isn't helpful, but we'll come back to it!
    return new AsyncData();
  }

  export default helper(([promise]) => load(promise));
```

Now our template invocation could just look like this:

```handlebars
{{#let (load @somePromise) as |data|}}
  {{#if data.isLoading}}
    <p>loading...</p>
  {{else if data.isLoaded}}
    <p>{{data.value}}</p>
  {{else if data.isError}}
    <p>Whoops! Something went wrong!</p>
    <p>{{data.error.message}}</p>
  {{/if}}
{{/let}}
```

So far, so good! Unfortunately, though, both the template and the backing class uses of `load` will *always* return the `'LOADING'` versions: we haven’t done anything to connect the `Promise` state to the `AsyncData`’s state. We need to do two things to connect them:

1. Add methods to `AsyncData` for changing the state correctly.
2. Use those methods when the `Promise` changes state!

#### Add state change methods to `AsyncData`

We’ll start by adding two methods to `AsyncData`: one for when the promise *resolves* and one for when it *rejects*. We’ll name them `resolveWith` and `rejectWith` respectively, so that when we call them it will look like `resolveWith(value)` or `rejectWith(error)`.

```diff
  import { helper } from '@ember/component/helper';
  import { assert } from '@ember/debug';
  import { tracked } from '@glimmer/tracking';

  class AsyncData {
    // SNIP: internal state and getters are unchanged...
+
+   resolveWith(value) {
+     this._state = 'LOADED';
+     this._value = value;
+   }
+
+   rejectWith(error) {
+     this._state = 'ERROR';
+     this._error = error;
+   }
  }

  export function load(somePromise) {
    // this isn't helpful, but we'll come back to it!
    return new AsyncData();
  }

  export default helper(([promise]) => load(promise));
```

These two methods have one of the core responsibilities of `AsyncData`: managing internal state correctly. Providing this public interface and making the details of our state management private to callers lets us uphold the invariants that `AsyncData` needs to be used safely. If we happened to set `_value` instead of `_error` when calling `rejectWith`, things would be badly broken.

This is a good argument for rigorous tests! It’s also another reason we might choose to use the variant types I described above, with `{ state: 'LOADED', value }` instead of making them discrete properties. I’ve chosen to leave them as they are here both for simplicity and to match the version we actually use internally today, but if I were implementing this from scratch myself, I would certainly do it differently for exactly that reason!

At this point `AsyncData` is pretty much done! It’s time to connect it to the promise data flow.

#### Connect the `Promise` and `AsyncData` in `load`

We now return to the `load` function implementation. When the promise *resolves*, we want to call `AsyncData.resolveWith`; when it *rejects*, we’ll call `AsyncData.rejectWith`:

```diff
  import { helper } from '@ember/component/helper';
  import { assert } from '@ember/debug';
  import { tracked } from '@glimmer/tracking';

  class AsyncData {
    // SNIP: class body is unchanged...
  }

  export function load(somePromise) {
-   // this isn't helpful, but we'll come back to it!
-   return new AsyncData();
+   let asyncData = new AsyncData();
+
+   somePromise.then(
+     (value) => asyncData.resolveWith(value),
+     (error) => asyncData.rejectWith(error)
+   );
+
+   return result;
  }

  export default helper(([promise]) => load(promise));
```

That’s actually all that’s required to connect them. At this point, we can use the helper or the function and things will work as we expect! We’re not *quite* done, though.

### Return the same `AsyncData`

What if two different parts of our code both call `load` with the same `Promise`? It would be quite wasteful to create another `AsyncData` to represent the same promise every time it was passed in! Instead, we want to be able to know if we’ve seen any given `Promise` before, and associate it with the same `AsyncData` if so. We *don’t* want to prevent `Promise` or `AsyncData` instances from getting cleaned up when the app is done with them, though: that would result in a memory leak! If we made heavy use of `load` across our app, we could end up with undead `Promise` and `AsyncData` instances floating around forever.

Modern JavaScript has a tool for scenarios just like this: a `WeakMap`. A `WeakMap` is a *map* from keys to values, where the value can be anything but the key has to be an object. The neat thing about `WeakMap` is that it doesn’t interfere with garbage collection: if a `WeakMap` key is the last place that an object is used, it will get garbage collected and removed from the `WeakMap` automatically, along with the reference to whatever the key was pointing to in the map. That is, it has *weak* references to the objects it uses as keys. This is *not* like a normal `Map`, which can *also* use an object as its key. A normal `Map` will hold on to its keys *strongly*: you have to *remove the key explicitly* (using `Map.delete`) for the object to be allowed to be garbage-collected.  This makes `WeakMap`s a great choice for any time we want to create a link between two objects *without* creating a memory leak by preventing them from being freed.

To use a `WeakMap` to link each `Promise` to an `AsyncData`, we will create a `WeakMap` instance in module scope. Then, in `load`, we will first check if the `somePromise` argument is already a key in the `WeakMap`, and if so return the `AsyncData` it points to. If it is a `Promise` we haven’t seen before, we’ll connect the `Promise` and the `AsyncData` by using `WeakMap.set`.

```diff
  import { helper } from '@ember/component/helper';
  import { assert } from '@ember/debug';
  import { tracked } from '@glimmer/tracking';

  class AsyncData {
    // SNIP: class body is unchanged...
  }

+ const MAP = new WeakMap();

  export function load(somePromise) {
+   let existingAsyncData = MAP.get(somePromise);
+   if (existingAsyncData) {
+     return existingAsyncData;
+   }
+
    let asyncData = new AsyncData();
+   MAP.set(somePromise, asyncData);

    somePromise.then(
      (value) => asyncData.resolveWith(value),
      (error) => asyncData.rejectWith(error)
    );

    return asyncData;
  }

  export default helper(([promise]) => load(promise));
```

Now we can call `load` on the same `Promise` as many times as we want; it will always return the same `AsyncData`.

It’s important to understand that this only holds when it’s actually the same `Promise`, though—and when you chain off a `Promise` using its `.then` or `.catch` methods, you create a *new* `Promise`. That means that if you do this, you’ll end up creating two `AsyncData` instances:

```js
import { load } from 'my-app/helpers/load';

let promise = Promise.resolve('hello');
let firstAsyncData = load(promise);
let chained = promise.then((s) => s.length);
let secondAsyncData = load(chained);

console.log(firstAsyncData === secondAsyncData);  // false
```

This is *correct*. You could be triggering all sorts of *new* asynchronous behavior in those `.then` or `.catch` callbacks! Each `AsyncData` represents exactly *one* asynchronous data operation.

## Conclusion

With that, we’ve implemented a solution that captures both philosophical commitments we laid out at the beginning:

1. *Async data is just data*
2. *Handling all data states is important*

To implement those ideas:

- We created a `load` function we can use both in JS and as a helper in Glimmer templates.
- We modeled the states reactively with `@tracked` properties on an `AsyncData` class.
- We updated the state by wiring up the `Promise` transitions to methods on `AsyncData`.
- We also made sure that we always have exactly and only one `AsyncData` per promise, using a `WeakMap` to connect each `Promise` to an `AsyncData`.

Here’s what it looks like with all of the pieces put together:[^actual-impl]

```js
import { helper } from "@ember/component/helper";
import { assert } from "@ember/debug";
import { tracked } from "@glimmer/tracking";

class AsyncData {
  /**
    @type {'LOADING' | 'LOADED' | 'ERROR'}
    @private
   */
  @tracked _state = "LOADING";

  /** @private */
  @tracked _value;

  /** @private */
  @tracked _error;

  get state() {
    return this._state;
  }

  get value() {
    assert(
      `You can only access 'value' when 'state' is 'LOADED', but it is ${this.state}`,
      this.state === "LOADED"
    );

    return this._value;
  }

  get error() {
    assert(
      `You can only access 'error' when 'state' is 'ERROR', but it is ${this.state}`,
      this.state === "ERROR"
    );

    return this._error;
  }

  get isLoading() {
    return this.state === "LOADING";
  }

  get isLoaded() {
    return this.state === "LOADED";
  }

  get isError() {
    return this.state === "ERROR";
  }

  resolveWith(value) {
    this._state = "LOADED";
    this._value = value;
  }

  rejectWith(error) {
    this._state = "ERROR";
    this._error = error;
  }
}

const MAP = new WeakMap();

export function load(somePromise) {
  let existingAsyncData = MAP.get(somePromise);
  if (existingAsyncData) {
    return existingAsyncData;
  }

  let asyncData = new AsyncData();
  MAP.set(somePromise, asyncData);

  somePromise.then(
    (value) => asyncData.resolveWith(value),
    (error) => asyncData.rejectWith(error)
  );

  return asyncData;
}

export default helper(([promise]) => load(promise));
```

Hopefully you now have a better idea of how we can combine custom data structures built with JS classes, autotracking-powered reactivity, and modern JS features like `WeakMap` to build robust solutions for even tricky problems like asynchronous data flow!

:::callout

Feel free to respond with questions or comments on [Ember Discuss](https://discuss.emberjs.com/t/async-data-and-autotracking-in-ember-octane/18177)!

For further reading on autotracking, check out these posts by my friend and colleague Chris Garrett ([@pzuraq](https://www.pzuraq.com)), who knows autotracking better than almost anyone else:

- [How Autotracking Works](https://www.pzuraq.com/how-autotracking-works/)
- [Autotracking Case Study: TrackedMap](https://www.pzuraq.com/autotracking-case-study-trackedmap/)

:::

## Appendix: TypeScript

As long-time readers of this blog (and many folks in the Ember community) know, I’m a huge advocate of TypeScript. I’m particularly a fan of using types to guarantee that our data is *always* in a valid state. In the implementation of `AsyncData` as we have it, we *do* always have data in a valid state—but that’s just because we’ve been careful, and our end users can pretty easily interact with `AsyncData` in unsafe ways. What might it look like to make it so that we can never *construct* invalid data, and so that our end users need to interact more safely with the data?

:::note

For this example, I am *assuming* rather than *explaining* the TypeScript features in use.

:::

One key challenge and constraint is that the `AsyncData` type has to work in *two* programming languages: JavaScript (or TypeScript) and Glimmer templates. The TypeScript features I would normally reach for here simply don’t work all that well in Glimmer templates. So take what follows with that in mind: trying to make this more robust will mean that it will be *slightly* harder to use in templates—and any concessions we make to the template ergonomics will *necessarily* make our implementation less type-safe. That means that the *most useful* ways to use this implementation will be a little different on the TS side than on the Glimmer template side.

The key things to note here are:

- I’ve done as I suggested in the main part of the post, and made the internal state be a union of object types. This means that it would be a type error if we wrote a type in the `resolveWith` or `rejectWith` methods. It also means that callers can actually use `AsyncValue.data` directly and `switch` on its `state` property to get safe access to `value` or `error`. (This is how I would *recommend* people access it!)

- Unfortunately, exposing the `state` and `value` and `error` getters means that users can *also* engage in *unsafe* runtime behavior. The unfortunate reality, though, is that until we have type-checked templates, even making users go through the `data` getter wouldn’t help here: you could always write `{{asyncData.data.value}}` and it would simply throw via the debug assertion when it wasn’t in the correct state.

- The use of the `assert` functions guarantees that our getters for `value` and `error` are in fact safe. The type for `assert` takes advantage of TypeScript 3.7’s [assertion functions](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-3-7.html#assertion-functions) to inform the type system of which variant the `Data` is in. This *helps* in that users will get those runtime failures in their tests if they don’t check the state correctly, but it unfortunately means that you can also just write `someAsyncData.value` and it will type-check.

- I have added `// SAFETY: ...` comments for the places where we have to make type casts. TypeScript cannot track the guarantees we’re upholding via our invariants. I *always* write this kind of comment on any type cast I write, so that it’s clear later what has to remain true for the cast to remain safe.[^4]

Here’s what the TS implementation would look like:

```ts
import { helper } from '@ember/component/helper';
import { assert } from '@ember/debug';
import { tracked } from '@glimmer/tracking';

type Data<Value> =
  | { state: 'LOADING' }
  | { state: 'LOADED', value: Value }
  | { state: 'ERROR', error: unknown }
  ;

class AsyncData<Value, Err = unknown> {
  @tracked private _data: Data<Value> = { state: 'LOADING' };

  get data(): Data<Value> {
    return this._data;
  }

  get state(): Data<Value>['state'] {
    return this._data.state;
  }

  get value(): Value {
    assert(
      `cannot get 'value' with state ${this._data.state}`,
      this._data.state === 'LOADED'
    );

    return this._data.value;
  }

  get error(): unknown {
    assert(
      `cannot get 'reason' with state ${this._data.state}`,
      this._data.state === 'ERROR'
    );

    return this._data.error;
  }

  get isLoading(): boolean {
    return this._data.state === 'LOADING';
  }

  get isLoaded(): boolean {
    return this._data.state === 'LOADED';
  }

  get isError(): boolean {
    return this._data.state === 'ERROR';
  }

  resolveWith(value: Value): void {
    this._data = { state: 'LOADED', value };
  }

  rejectWith(error: Err): void {
    this._data = { state: 'ERROR', error };
  }
}

const MAP: WeakMap<Promise<unknown>, AsyncData<unknown>>
  = new WeakMap();

function load<Value>(
  somePromise: Promise<Value>
): AsyncData<Value> {
  let existingAsyncData = MAP.get(somePromise);
  if (existingAsyncData) {
    // SAFETY: this cast only holds because we *know* that we've
    // kept the `Promise` and the `AsyncData` instances in sync
    // via the `WeakMap`. If that were not the case, this cast
    // would be `unsafe`.
    return existingAsyncData as unknown as AsyncData<Value>;
  }

  // SAFETY: this only holds because we are working with
  // `Promise<Value>`.
  let asyncData = new AsyncData<Value>();
  MAP.set(somePromise, asyncData);

  somePromise.then(
    (value) => asyncData.resolveWith(value),
    (error) => asyncData.rejectWith(error)
  );

  return asyncData;
}

export default helper(
  ([somePromise]: [Promise<unknown>]) => load(somePromise)
);
```


*[JS]: JavaScript
*[API]: application programming interface


[^1]:

    I would actually seriously consider reworking this in terms of a tracked `WeakMap` implementation for use with TypeScript to make these guarantees that much more reliable!

[^2]:

    We could actually implement true privacy ourselves, using the same technique that Babel and TypeScript use—a `WeakMap` associating each instance and a POJO containing its private fields—but it doesn’t *really* matter for our purposes, and might not be compatible with a future version of the decorators spec anyway.

    *[POJO]: plain old JavaScript object

[^3]:

    If you reference [the gist I published](https://gist.github.com/chriskrycho/306a82990dd82203073272e055df5cd1) for `load` and `AsyncData`, you’ll notice that these `assert`s are *not* present. This is a matter of backwards compatibility with pre-Octane code. We’ll be working with autotracking in the next section, and therefore *could* use plain getters to access the state and update correctly.

    However, *if* you refer to these getters with `@computed` or any of the computed property macros, using the `@dependentKeyCompat` decorator, this will cause problems, because classic computed properties actually invoke the getters for their dependent keys, and so will invoke these even when users don’t intend to.

[^actual-impl]:

    If you look at the source gist for the implementation we’re using currently, you'll see a few differences and additions to what I described in this post:

    - We use `@dependentKeyCompat` to interoperate with Ember Classic computed properties, and avoid the debug assertions in the `value` and `error` getters for the same reason.

    - We have support for treating `AsyncData` as a “then-able”—that is, for making it possible to use it basically like you would a `Promise`. That is useful, but it’s not actually key to understanding the type and how to use it, so I left it aside in this discussion.

    - We also support passing in non-`Promise` data, and turning it into a `Promise` and `AsyncData` which are immediately resolved. In retrospect, I’d really prefer to remove this and have people think about their data more carefully—even just requiring them to explicitly do `load(Promise.resolve(123))` in those cases instead of `load(123)`.


[^4]: This is an idea I stole from Rust, where most of the community idiomatically uses the same kind of comments anywhere that Rust’s `unsafe` keyword appears in code.