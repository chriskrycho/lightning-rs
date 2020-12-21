---
title: >
    Migrating Off of `PromiseProxyMixin` in Ember Octane
subtitle: >
    An important refactor for getting rid of mixins *and* proxies.
date: 2020-08-17T17:15:00-0600
updated: 2020-09-26T10:50:00-0600
qualifiers:
    audience: Software developers working with Ember Octane.
summary: >
    Switch away from the legacy "PromiseProxyMixin" type to a lightweight, auto-tracked, Octane-ready, future-friendly solution--and learn to think about async data in a new way!
tags:
    - JavaScript
    - software development
    - web development
    - Ember
    - auto-tracking
templateEngineOverride: md
thanks: >
    Thanks to Jeremy Sherman for catching an obnoxious mistake I missed in editing!

---

Idiomatic Ember Octane avoids using Ember’s classic `Mixin` and `ObjectProxy` types. However, a very common pattern in many Ember Classic apps and addons was to use Ember’s `PromiseProxyObject` mixin in conjunction with `ObjectProxy` to expose the state of a promise to end users, and to make accessing the resolved data more convenient. Migrating an app from Ember Classic to be *idiomatic* Ember Octane means replacing all of that with something more Octane-friendly.

In this post, we will cover how to rewrite code that uses promise proxy mixing into a lightweight, auto-tracked, Octane-ready, future-friendly solution—and hint at a new to think about asynchronous data in a new way, as well!￼

- [A direct migration](#a-direct-migration)
- [Alternative: less JS, more template](#alternative-less-js-more-template)
- [Alternative: less template, more JS](#alternative-less-template-more-js)
- [Summary](#summary)

## A direct migration

We’ll start with a utility that allows us to create a proxy at will, `createPromiseProxy`:

```js
// my-app/utils/object-promise-proxy.js
import ObjectProxy from '@ember/object/proxy';
import PromiseProxyMixin from '@ember/object/promise-proxy-mixin';

const ObjectPromiseProxy = ObjectProxy.extend(PromiseProxyMixin);

export function createPromiseProxy(promise) {
  return ObjectPromiseProxy.create({ promise });
}
```

Then we might use it in a component that looks like this:[^jquery]

```js
import Component from '@glimmer/component';
import { createPromiseProxy } from 'my-app/utils/object-promise-proxy.js';

const USERS_API = 'example.com/users';

export default class SmartComponent extends Component {
  get userData() {
    let url = new URL(USERS_API);
    url.searchParams.append('id', this.args.id);

    return createPromiseProxy(fetch(url)).then((data) => data.json());
  }
}
```

:::note

Here we’re relying on the fact that `args` are auto-tracked: this getter consumes `this.args.id`, so it’ll rerun any time the component is invoked with a new `id`. In a classic Ember component, you might see `@computed('id')` to update whenever the `id` argument updated.

:::

We would invoke the component something like this (presumably with a more dynamic source of the ID):

```hbs
<SmartComponent @id={{1234}} />
```

The body of the component might look like this:

```hbs
{{#if this.userData.isFulfilled}}
  {{this.userData.userName}}
{{else if this.userData.isRejected}}
  Whoops, something went wrong!
{{/if}}
```

To migrate away from this, we can use a composition-based approach instead of a mixin/inheritance-based approach. I’m going to use a `load` helper and associated `AsyncData` structure ([defined here][load]). I plan to write a post explaining the underlying ideas for that helper in the future. For now, it’s enough to know the following things:

- The helper can be used with any value, `Promise` or not.

- It can be used in templates *or* imported and used in JavaScript.

- It returns an `AsyncData`, which has the following public properties:
    - `state`, which can be `'LOADING'`, `'LOADED'`, or `'ERROR'`
    - `isLoading`
    - `isError`
    - `value`, which is either the resolved value if the promise has resolved or `undefined` if it’s still pending or has rejected
    - `error`, which is either the promise rejection value if the promise rejected or `undefined` if the promise is still pending or has resolved successfully

[load]: https://gist.github.com/chriskrycho/306a82990dd82203073272e055df5cd1

Using it looks pretty similar to using the component with the promise proxy mixin—we’ve just replaced the `createPromiseProxy` call with the `load` call:

```js
import Component from '@glimmer/component';
import { load } from 'my-app/helpers/load';

const USERS_API = 'http://www.example.com/users';

export default class SmartComponent extends Component {
  get userData() {
    let url = new URL(USERS_API);
    url.searchParams.append('id', this.args.id);
    
    return load(fetch(url)).then((data) => data.json());
  }
}
```

Invoking it would be identical; the only change is in the corresponding template:

```hbs
{{#if this.userData.isLoaded}}
  {{this.userData.value.userName}}
{{else if this.userData.isError}}
  Whoops, something went wrong!
{{/if}}
```

The actual changes here are small:

- There’s one extra `.value` intermediate value lookup: `this.userData.value.userName` instead of `this.userData.userName`. (This is the result of *composing* the data instead of *inheriting* it.)
- The names of the _state_ values are different: `isLoaded` and `isError` instead of `isFulfilled` and `isRejected`.

And with that, we’ve successfully gotten away from `PromiseProxyMixin` in our app code!

## Alternative: less JS, more template

We could also do more of this template-side, since the `load` tool is both a utility function and a helper. In that case, here’s how the component would look:

```js
import Component from '@glimmer/component';

const USERS_API = 'http://www.example.com/users';

export default class SmartComponent extends Component {
  get userData() {
    let url = new URL(USERS_API);
    url.searchParams.append('id', this.args.id);
    
    return fetch(url);
  }
}
```

The template would use the `load` helper with the resulting promise in the template, by invoking it with [the `let` helper][let]:

[let]: https://api.emberjs.com/ember/3.20/classes/Ember.Templates.helpers

```hbs
{{#let (load this.userData) as |result|}}
  {{#if result.isLoaded}}
    {{result.value.userName}}
  {{else if result.isError}}
    Whoops, something went wrong!
  {{/if}}
{{/let}}
```

## Alternative: less template, more JS

Because the `load` utility and its `AsyncData` type use auto-tracking, we can freely do things with the resulting data type in our JavaScript, too. For example, if we wanted to pull *all* the logic into a new component which just accepts an `AsyncData` for the user profile, we could do that. Assume we had our original `load`-using component version, which has `this.userData` as an `AsyncData`. We could pass it to another component like so:

```hbs
<RenderUser @userData={{this.userData}} />
```

Then we could make the `RenderUser` component’s template be extremely simple:

```hbs
<div>{{this.content}}</div>
```

The `content` could be specified via a getter on the backing class:

```js
import Component from '@glimmer/component';

export default class RenderUser extends Component {
  get content() {
    switch (this.args.userData.state) {
      case 'LOADED': {
        let user = this.args.userData.value;
        return `${user.name} is ${user.age} years old!`;
      }
      case 'LOADING':
        return 'Loading...';
      case 'ERROR':
      default:
        return 'Something went wrong. 😱 Please try again!';
    }
  }
}
```

Again, we’re taking advantage of `args` being auto-tracked: if we ever got a *different* `AsyncData` passed in as `userData`, we would update to the correct version of that. Likewise, because the `state` and `data` properties of the `AsyncData` type are tracked, this getter will recompute any time either of those is updated as well.

## Summary

We do have to type `.value` in a couple of places now… but in exchange, we get all the benefits of the old `PromiseProxyMixin` in exchange, and we get to get rid of a `Mixin` *and* a use of Ember’s classic (and very expensive for performance) `ObjectProxy`, which is yet another `Mixin`. What’s more, there’s no magic here. You can implement `load` yourself in plain JavaScript using the Glimmer tracking library, just the same as I did!

:::callout

Feel free to respond with questions or comments [on Ember Discuss](https://discuss.emberjs.com/t/migrating-off-of-promiseproxymixin-in-ember-octane/18138/2). And if you’re curious about how `load` and `AsyncData` work, check out [the follow-up post](https://v5.chriskrycho.com/journal/async-data-and-autotracking-in-ember-octane/)!

:::



[^jquery]: Ember’s API guides for `PromiseProxyMixin` give an example very similar to this, but with less context and more jQuery. I’ve replaced the use of jQuery’s `$.getJSON` with `fetch` and `Body.json()`, and used arrow functions instead of `function` declarations; I’ve also embedded it in an example component to make the ideas a bit clearer.
