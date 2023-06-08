---
title: >
  Autotracking: Elegant DX Via Cutting-Edge CS
subtitle: >
  A modern JavaScript reactivity system powered by Lamport clocks and incremental computation and depth-first searches: *oh my!*
summary: >
  One of the key features of Ember Octane is autotracking: a lightweight reactivity system powered by Lamport clocks, incremental computation, and depth-first-searches—which lets you write “normal” JavaScript or TypeScript and have everything Just Work™.
qualifiers:
  audience:
    Software engineers interested in reactivity models in general and in web <abbr title="user interface">UI</abbr> and JavaScript in particular.

date: 2020-09-22T15:15:00-0600
updated: 2020-11-22T21:10:00-0600

thanks: >
  [Chris Garrett](https://pzuraq.com) ([@pzuraq](https://github.com/pzuraq)) gave helpful feedback on a draft of this post, as well as helping me understand some of these mechanics better in the first place. [James C. Davis](https://github.com/jamescdavis) and [Nick Morgan](https://github.com/morganick) helped me fix some typos. (All mistakes are mine!)

tags:
  - Ember
  - Glimmer
  - JavaScript
  - autotracking
  - software development
  - web development
  - reactivity

featured: true

templateEngineOverride: md

---

One of the key features of [Ember Octane][octane] is *autotracking*, a lightweight reactivity system powered by Lamport clocks, incremental computation, and depth-first-searches—which allows you to write code like this, and have it *Just Work™*:

[octane]: https://emberjs.com

```js
import Component from '@glimmer/component';
import { tracked } from '@glimmer/tracking';

const MAX_LENGTH = 10;

export default class PersonInfo extends Component {
  @tracked name = '';

  get nameLength() {
    return this.name.length;
  }

  get remaining() {
    return MAX_LENGTH - this.nameLength;
  }

  get showError() {
    return this.remaining < 0;
  }

  updateName = ({ target: { value } }) => this.name = value;
}
```

```handlebars
<div>
  <input {{on "input" this.updateName}} value={{this.name}} />
  <p class={{if this.showError "error"}}>
    ({{this.remaining}} remaining)
  </p>
</div>
```

There are a handful of interesting features to note about this code’s approach to reactivity. We decorate *one* piece of state, `name`, with `@tracked`, and the rest of the state updates automatically—including the `showError` and `remaining` properties, which don’t even refer to `name` directly. All of this with a particularly light touch:

- There is no need to mark dependent keys on the getters (as in classic Ember components) and no need for a `computed` hash (as in Vue 2) for derived state: these are plain JavaScript getters.

- There is no need for a dedicated utility like `setState` like in React’s class-based components or `set` from Ember Classic; this code just uses standard JavaScript assignment to update the value of `name`.

- This does not use two-way binding like *really old* Ember did or current day Angular or Vue do[^vue-2wb]—updates are explicit, but brief.

This can look like magic when you first encounter it—especially the way undecorated getters update on demand. In fact, though, it’s *Just JavaScript™*, built on standard JavaScript patterns and a mix of computer science ideas ranging from tried-and-true ideas from decades ago to cutting-edge research. In the rest of this post, we’ll see how it works.

[^vue-2wb]: Vue does not *require* two-way binding, but does make it *easy*.

## How getters work

First, let’s make sure we have a clear handle on how getters work in JavaScript in general. Once you understand this, seeing how autotracking works will be much easier. (If you already have a good understanding of the semantics and behavior of getters vs. assignment, feel free to [skip to the next section](#autotracking).) We’ll start by looking at the exact same class we started with, but with all of the Glimmer and DOM details removed, a constructor added, and continuing to use the same function style for `updateName`:[^updateName-style]

```js
const MAX_LENGTH = 10;

export default class PersonInfo {
  name;

  constructor(name) {
    this.name = name;
  }

  get nameLength() {
    return this.name.length;
  }

  get remaining() {
    return MAX_LENGTH - this.nameLength;
  }

  get showError() {
    return this.remaining < 0;
  }

  updateName = (value) => this.name = value;
}
```

Whenever we look up `nameLength` from somewhere else—

```js
let personInfo = new PersonInfo("Chris");
console.log(personInfo.nameLength); // 5
```

—the `nameLength` property (technically an *accessor*) executes as if it were a function. Before JS had native getters, that’s how we would have written it, and we still *could* write it that way:

```js
const MAX_LENGTH = 10;

export default class PersonInfo {
  name;

  constructor(name) {
    this.name = name;
  }

  nameLength() {
    return this.name.length;
  }

  remaining() {
    return MAX_LENGTH - this.nameLength();
  }

  showError() {
    return this.remaining < 0;
  }

  updateName = (value) => this.name = value;
}

let personInfo = new PersonInfo();
console.log(personInfo.nameLength());
```

Notice the two differences here: `personInfo.nameLength()` instead of `personInfo.nameLength `, and `nameLength() { ... }` instead of `get nameLength() { ... }`. These are effectively the same: both are functions which compute a value.

The other thing to notice here is that method invocations and getter lookups are both “lazy:” they run on demand. Until you actually invoke the method or the getter, there is a reference to a function as part of the class, but there isn’t any value computed by it. This is different from assigning a property directly. For example, if we assigned the values of `nameLength`, `remaining`, and `showError` in the constructor, they would *initially* have the same values as in the lazy version, but it would immediately get out of sync if you *changed* the value of `name` later:

```js
const MAX_LENGTH = 10;

export default class PersonInfo {
  name;
  nameLength;
  remaining;
  showError;

  constructor(name) {
    this.name = name;
    this.nameLength = name.length;
    this.remaining = MAX_LENGTH - this.nameLength;
    this.showError = this.remaining < 0;
  }

  updateName = (value) => this.name = value;
}

let personInfo = new PersonInfo("Chris");
console.log(personInfo.nameLength); // 5

personInfo.updateName("Chris Krycho");
console.log(personInfo.nameLength); // still 5 😭
```

Doing this “eagerly” means that we computed the values of `name`, `nameLength`, and `remaining` when we assigned each of the derived properties, `nameLength`, `remaining`, and `showError`. We did *not* create a function which references those properties, which we could use to evaluate their values at a later time. To do that in the constructor, we could define `nameLength`, `remaining`, and `showError` as arrow functions, taking advantage of the fact that closures get a reference to the values they use from their enclosing scope:[^closures-classes]

```js
const MAX_LENGTH = 10;

export default class PersonInfo {
  name;
  nameLength;
  remaining;
  showError;

  constructor(name) {
    this.name = name;
    this.nameLength = () => this.name.length;
    this.remaining = () => MAX_LENGTH - this.nameLength;
    this.showError = () => this.remaining < 0;
  }

  updateName = (value) => this.name = value;
}

let personInfo = new PersonInfo("Chris");
console.log(personInfo.nameLength()); // 5

personInfo.updateName("Chris Krycho");
console.log(personInfo.nameLength()); // 12
```

But calling `personInfo.nameLength()` like this looks awfully familiar: it’s the same as the class method version we might have used before we had native getters. We’re back to where we started, in other words.

The values a function uses are only evaluated when the function is invoked, whether the function in question is a standalone function, a class method, or a getter. If we have a *chain* of getters (or methods or functions), none of them will be reinvoked until the one at the end of the chain is. We won’t evaluate any of the values they reference until we access a getter which uses them. As a result, any time we evaluate a getter, we’ll always get an up-to-date version of all the values involved. We can add some logging to the getters in `PersonInfo` to see how this behaves:

```js
const MAX_LENGTH = 10;

export default class PersonInfo {
  name;

  constructor(name) {
    this.name = name;
  }

  get nameLength() {
    console.log("evaluating `nameLength`");
    return this.name.length;
  }

  get remaining() {
    console.log("evaluating `remaining`");
    return MAX_LENGTH - this.nameLength;
  }

  get showError() {
    console.log("evaluating `showError`");
    return this.remaining < 0;
  }

  updateName = (value) => this.name = value;
}
```

If we create and use a `PersonInfo` like this—

```js
let personInfo = new PersonInfo("Chris");
console.log(" --- 1 --- ");
console.log(personInfo.showError);

console.log("\n --- 2 --- ");
console.log(personInfo.nameLength);

console.log("\n --- 3 --- ");
personInfo.updateName("Chris Krycho");
console.log(personInfo.remaining);
console.log(personInfo.showError);
```

—the console output would read:

```
 --- 1 --- 
evaluating `showError`
evaluating `remaining`
evaluating `nameLength`
false

 --- 2 --- 
evaluating `nameLength`
5

 --- 3 --- 
evaluating `remaining`
evaluating `nameLength`
-2
evaluating `showError`
evaluating `remaining`
evaluating `nameLength`
true
```

In this example, the JavaScript I’ve written evaluates the values directly when logging them. When we use a value in a template in Ember or Glimmer apps, the template engine (the Glimmer VM) evaluates those values. The VM uses a lightweight reactivity system called *autotracking* to track which items in the UI need to be updated in any render. The next step, then, is understanding autotracking.

[^updateName-style]: We could switch to a class method here, but we’d just have to switch back later when we come back to the component code again. For Ember users reading this: yes, you *can* use this approach, although it’s currently idiomatic to use `@action`. 

[^closures-classes]: It’s also worth seeing how closures are the [dual](https://en.wikipedia.org/wiki/Duality_(mathematics)) of classes. These two have *the same semantics* as far as an end user is concerned:

    ```js
    class PersonA {
      #age;
      #name;

      constructor(name, age) {
        this.#age = age;
        this.#name = name;
      }

      get description() {
        return `${this.#name} is ${this.#age} years old!`;
      }

      haveABirthday() {
        this.#age += 1;
      }

      changeNameTo(newName) {
        this.#name = newName;
      }
    }

    function PersonB(name, age) {
      let _name = name;
      let _age = age;

      return {
        get description() {
          return `${_name} is ${_age} years old!`;
        },

        haveABirthday() {
          _age += 1;
        },

        changeNameTo(newName) {
          _name = newName;
        },
      };
    }
    ```
    
    Bonus: this is actually a critical part of how [React Hooks][hooks] work under the hood.

[hooks]: https://reactjs.org/docs/hooks-intro.html

## Autotracking

Autotracking is a lightweight reactivity system, composed of three ideas:[^mobx-redux-too]

1. Create a single global “clock:” a single integer, only ever increasing,[^monotonic] counting how many times any “tracked” state in your system has changed.

2. “Track” each piece of data in your system that you care about reacting to. Whenever any tracked data changes, increment the global clock (1) and associate the updated global clock value with the data that just changed.

3. Whenever you compute a value for a template,[^reactive-contexts] note any tracked values used in the computation, storing their global clock values. Combined with (2), these can be used to know when to *re*-compute template values.

The autotracking runtime implements exactly these three ideas: (1) a global clock (2) which is connected to tracked state (3) to know when to recompute the values in templates. The global clock is extremely simple: it really is [just an integer][revision-impl]. The more interesting bits are the *other* ideas: (2) connecting tracked state to the global clock, and (3) using that connection to know when to recompute values in templates.

[revision-impl]: https://github.com/glimmerjs/glimmer-vm/blob/520fb6f75897e89bea5231f83f5b01bf0bd94fc7/packages/%40glimmer/validator/lib/validators.ts#L14:L18

[^mobx-redux-too]: These same ideas—which are used for Ember’s template layers today—can also be used to implement pay-as-you-go reactivity in totally different reactivity models. For example, you can use it to reimplement [MobX](https://github.com/pzuraq/trackedx) or [Redux](https://github.com/pzuraq/tracked-redux).

[^monotonic]: That is: [*monotonically* increasing][monotonicity].

[monotonicity]: https://en.wikipedia.org/wiki/Monotonic_function

[^reactive-contexts]: Today, the only reactive context Ember has is its template layer, where values you render or pass as arguments to components, modifiers, or helpers are all *reactive*. [Soon][invoke-helper], though, we will also have reactive functions available in JavaScript contexts, which will make the reactivity system fully general!

### (2) Tracked state

Decorating a property with `@tracked` sets up a getter and a setter for a tracked property, and both connect to the global clock. When you write this—

```js
import { tracked } from '@glimmer/tracking';

class PersonInfo {
  @tracked name = '';
}
```

—it turns into something which acts more like this, where `markAsUsed` says that a property was *read* and `markAsChanged` says it was *set*:

```js
// THESE IMPORTS ARE NOT REAL
import { markAsUsed, markAsChanged } from '@glimmer/...';

class Person {
  // THIS IMPLEMENTATION IS NOT THE REAL ONE EITHER
  #name;

  get name() {
    markAsUsed(this, 'name');
    return this.#name;
  }

  set name(newValue) {
    markAsChanged(this, 'name');
    this.#name = newValue;
  }
}
```

This is *not* the actual implementation—for one thing, you can’t use a decorator to change imports like this!—but it *is* the right mental model.[^actual-impl] Reading a tracked property always invokes `markAsUsed`, and setting it always invokes `markAsChanged`. (This is no different from the logging we added manually in the `PersonInfo` example earlier!)

[^actual-impl]:
    In the actual implementation, `@tracked` is actually implemented using a closure in another module, which uses functions named `consumeTag` and `dirtyTagFor`. The “tags” referenced in the functions’ names are lightweight objects which store the global clock value for a given piece of tracked data. For a walkthrough of the implementation, see the [Tracking in the Glimmer VM][walkthrough-video] video that [Chris Garrett][cg] and I recorded as he helped me fill in some of my gaps in understanding around all of this.

```js
let person = new PersonInfo();
console.log(person.name);  // -> `markAsUsed(person, 'name')`
```

Critically, the exact same thing is true if we use getters which *refer* to the tracked property. When we add the `nameLength` getter, which computes its value by referring to `this.name`, using that getter *also* causes `markAsUsed` to get run:

```js
import { tracked } from '@glimmer/tracking';

class Person {
  @tracked name = '';

  get nameLength() {
    return this.name.length;
  }
}

let person = new Person();
console.log(person.nameLength);
```

First, `@tracked` turns `name` into a getter/setter pair, just as we saw above. Second, `nameLength` gets the value of `name`. The getter for `name` first runs `markAsUsed(this, 'name')`, then returns the actual value stored in `#name`. This would remain true no matter how many getters we chained together: by the end, they would all end up using `name`, which would call `markAsUsed(this, 'name')`.

```js
import { tracked } from '@glimmer/tracking';

class Person {
  @tracked name = '';

  get nameLength() {
    return this.name.length;
  }

  get remaining() {
    return MAX_LENGTH - this.nameLength;
  }

  get showError() {
    return this.remaining < 0;
  }

  updateName = (value) => this.name = value;
}

let person = new Person();

// Person.showError ->
//   Person.remaining ->
//     Person.nameLength ->
//       Person.name *getter* ->
//         markAsUsed(this, 'name')
//         this.#name
console.log(person.showError);
```

Similarly, changing the value of `name` would invoke `markAsChanged` via the setter installed by `@tracked`:

```js
// Person.name *setter* ->
//   markAsChanged(this, 'name')
//   this.#name
person.name = "Chris";

// Person.updateName ->
//   Person.name *setter* ->
//     markAsChanged(this, 'name')
//     this.#name
person.updateName("Chris Krycho");
```

Exactly the same things happen if we render values or trigger changes from a Glimmer component’s template—as in the code example from the introduction:

```js
import Component from '@glimmer/component';
import { tracked } from '@glimmer/tracking';

const MAX_LENGTH = 10;

export default class PersonInfo extends Component {
  @tracked name = '';

  get nameLength() {
    return this.name.length;
  }

  get remaining() {
    return MAX_LENGTH - this.nameLength;
  }

  get showError() {
    return this.remaining < 0;
  }

  updateName = ({ target: { value } }) => this.name = value;
}
```

```handlebars
<div>
  <input {{on "input" this.updateName}} value={{this.name}} />
  <p class={{if this.showError "error"}}>
    ({{this.remaining}} remaining)
  </p>
</div>
```

Using `this.name` in the template directly evaluates `name`, which is the getter set up by `@tracked` and therefore calls `markAsUsed(this, 'name')`. Likewise, using `this.showError` and `this.nameLength` in the template evaluates those getters, which ultimately evaluate `name`, which again calls `markAsUsed(this, 'name')`. Calling `markAsUsed` tells the autotracking runtime that `this.name` is used to compute `name`, `remaining` and `showError` in the `PersonInfo` component’s template.

Triggering `updateName` by typing into the input invokes the setter for `name` installed by `@tracked`, and the setter calls `markAsChanged(this, 'name')`. Calling `markAsChanged` increments the global clock value, stores the updated clock value as the new clock value for `this.name`, and schedules a re-render.

With these pieces in place, we can start to see how the system works as a whole. Reading a `@tracked` property while evaluating a value in the template informs the Glimmer VM that it was used in computing that template value. Changing a `@tracked` property bumps the global and property clock values and schedules a new render. This leads us to idea (3): using the global clock values to know when to recompute values in templates.

### (3) Recomputing

When rendering templates,[^reactive-again] the runtime sets up what is called a *tracking frame* for each new “computation” in the UI—values, components, helpers, modifiers, etc. A tracking frame is basically just a list of all the tracked properties that called `markAsUsed` while computing any particular value in the template. Since each tracking frame corresponds to a dynamic element of the UI, evaluating the entire UI the first time it is rendered produces a tree of tracking frames which corresponds exactly to the tree of UI components. Critically, though, a tracking frame doesn’t store the *values* of the tracked properties referenced during its computation. Instead, the frame stores only a reference to each property along with the property’s current and previous global clock values.

In a normal JavaScript invocation, there is no active tracking frame, so calling `markAsUsed` is a no-op. When rendering, a tracking frame *does* exist, and it ends up populated with the clock values for all the tracked properties used while calculating that value. When a given tracking frame “closes”, as at the close of a component invocation, it computes its *own* clock value. A tracking frame’s clock value is the maximum clock value of any of the properties marked as used in that frame. Since clock values are integers, this maximum clock value can be computed very simply: by using `Math.max`.[^math-max]

As we saw above, changes enter the system by setting tracked properties. Recall that invoking `markAsChanged` bumps both the overall global clock value and the clock value for that property, and schedules a new render.[^coalescing] When the Glimmer VM re-renders, it can traverse the tree in a [depth-first search](https://medium.com/basecs/demystifying-depth-first-search-a7c14cccf056), comparing each frame’s current and cached clock values. If the clock value for a given frame hasn’t changed, nothing below it in the UI tree has changed, either—so we know we don’t need to re-render it. Checking whether that clock value has changed is literally just an integer equality check. At the nodes which *have* changed, the VM computes the new value and updates the DOM with the result.

[^math-max]: There are some details about how it checks the tree and makes sure that it manages its internal state correctly, but it really is [using `Math.max`](https://github.com/glimmerjs/glimmer-vm/blob/e8e2fc6f39a60baac2b72c1a19aea9585b162c47/packages/%40glimmer/validator/lib/validators.ts#L130:L172).

[^reactive-again]: or when using a “reactive function” via [the upcoming `invokeHelper` functionality][invoke-helper]

[^coalescing]: The VM coalesces these bumps so if you set a bunch of values in response to user action or API responses or other inputs, it only triggers *one* re-render, not *many*.

## Summary

There are a handful of really delightful consequences of this system:

- Re-renders are about as cheap as they possibly can be: all the state computations are simple integer math.

- Intermediate, “derived” state gets computed on demand when the state it depends on changes—but with normal JavaScript semantics, without extra developer-facing boilerplate or end-user impact on performance.

- It’s trivial to layer your own caching or memoization on top of these semantics if you need them, but you only pay for what you need.

- All the “smarts” lives at the very edge of the system, in root state marked with `@tracked` and leaf values computed in reactive contexts like templates.

Hopefully this has give you a good idea how autotracking works in general, and specifically how it simultaneously enables most of our code to be “just JavaScript” *and* gives us a very low-cost reactivity.

:::callout

You can discuss this [Hacker News](https://news.ycombinator.com/item?id=24560106), [lobste.rs](https://lobste.rs/s/amklz3/autotracking_elegant_dx_via_cutting_edge), or [Ember Discuss](https://discuss.emberjs.com/t/autotracking-elegant-dx-via-cutting-edge-cs/18231).

If you’d like to see some of the details of how these pieces are implemented, check out [the video][walkthrough-video] of my conversation with Ember core team member and Glimmer VM contributor [Chris Garrett][cg] ([@pzuraq](https://github.com/pzuraq/)). Chris also gave a [great talk on autotracking](https://www.youtube.com/watch?v=HDBSU2HCLbU) at EmberConf 2020, and wrote up a series of blog posts on the subject:

1. [What is Reactivity?](https://www.pzuraq.com/what-is-reactivity/)
2. [What Makes a Good Reactive System?](https://www.pzuraq.com/what-makes-a-good-reactive-system/)
3. [How Autotracking Works](https://www.pzuraq.com/how-autotracking-works/)—the most direct complement to *this* post
4. [Autotracking Case Study - TrackedMap](https://www.pzuraq.com/autotracking-case-study-trackedmap/)

Readers interested in the underpinnings of autotracking may want to take a look at [Adapton](http://adapton.org), the original research implementation of the specific theory of “incremental computation” underpinning autotracking. For another “real-world” implementation of the same ideas, check out [salsa](https://salsa-rs.github.io/salsa/): a [Rust](https://www.rust-lang.org) implementation of incremental computation which powers the [rust-analyzer](https://rust-analyzer.github.io) language server.

:::

[walkthrough-video]: https://www.youtube.com/watch?v=BjKERSRpPeI&amp;feature=youtu.be
[cg]: https://www.pzuraq.com
[invoke-helper]: https://emberjs.github.io/rfcs/0626-invoke-helper.html

*[VM]: virtual machine

*[API]: application programming interface

*[UI]: user interface

*[DOM]: document object model

