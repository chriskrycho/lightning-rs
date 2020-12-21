---
title: >
  Initializing Class Fields in Ember Octane
subtitle: >
  One of the many small-but-lovely benefits of getting to use native classes in Ember Octane.
qualifiers:
  audience: >
    Software developers working with Ember Octane.
date: 2020-10-06T10:55:00-0600
updated: 2020-10-06T17:31:00-0600
tags:
  - JavaScript
  - Ember
  - software development
  - web development

---

Long-time [Ember.js][ember] developers have often internalized the idea that any initialization should happen explicitly in Ember classic classes' `init` hook, and *not* directly in the plain old JavaScript object (POJO) which defines the classic class.

Bad classic code:

```js
import Component from '@ember/component';

export default Component.extend({
  someData: [],
})
```
    
Good classic code:

```js
import Component from '@ember/component';
import { set } from '@ember/object';

export default Component.extend({
  init() {
    this._super(...arguments);
    set(this, 'someData', []);
  }
})
```

This is because of how classic classes worked: the values passed into the POJO were set on the [prototype] for the class, and so were shared between all instances of the class. That meant that when you passed an array literal as part of the POJO, like `someData: []`, that same array would be shared between *all* instances of the component. This is *almost never* what you want, so developers learned (and [Ember's official lint rule set][lint] enforced!) that any reference types like objects or arrays be set in `init`.

When working with native classes, though, class fields are *not* set on the prototype, but on the instance. That means that you can just define values directly as class fields, because this—

```js
import Component from '@glimmer/component';

export default class MyComponent extends Component {
  someData = [];
}
```

—is the same as writing this:

```js
import Component from '@glimmer/component';

export default class MyComponent extends Component {
  someData;
  
  constructor() {
    super(...arguments);

    Object.defineProperty(this, 'someData', {
      enumerable: true,
      configurable: true,
      writable: true,
      value: [],
    });
  }
}
```

:::note

Here I’ve switched to the `@glimmer/component` base class to use `constructor` instead of `init`, but the idea is the same!

:::

For day-to-day purposes, [that `Object.defineProperty` call][defineProperty] is basically the same as just doing the assignment in the constructor.[^differences] The net of this is that you can leave behind your long-standing habits of doing assignment in the `constructor` where you’re just setting up a default value for a class field. The *only* time you should prefer to do things in the `constructor` is when it depends on the other state of the class—in other words, when it references `this` in some way. If it *doesn't* refer to `this`, though, even things like instantiating utility classes can just happen in class field assignment:

```js
import Component from '@glimmer/component';
import Formatter from '../utils/formatter';

export default class MyComponent extends Component {
  formatter = new Formatter();
}
```

Just one of the many niceties that come from upgrading to Ember Octane!


[ember]: https://emberjs.com
[prototype]: https://developer.mozilla.org/en-US/docs/Learn/JavaScript/Objects/Object_prototypes
[lint]: https://github.com/ember-cli/eslint-plugin-ember
[defineProperty]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/defineProperty

*[POJO]: plain old JavaScript object

[^differences]: The meaningful differences between assignment and `defineProperty` only come up when you’re stomping a property via inheritance. That’s nearly always a bad idea *anyway*!
