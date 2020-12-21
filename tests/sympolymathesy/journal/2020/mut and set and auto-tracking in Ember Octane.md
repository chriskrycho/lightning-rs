---
title: >
    `mut` (and `set`) and auto-tracking in Ember Octane
subtitle: >
    Understanding a surprising behavior—and fixing a refactoring hazard.
qualifiers:
    audience: >
        Software developers working with Ember Octane.
summary: >
    Ember’s `mut` helper, and the `set` helper from ember-simple-set helper, can mask an auto-tracking bug and refactoring hazard. Understand the bug and see how to fix it!
thanks: >
    Thanks to [Chris Garrett (@pzuraq)](https://www.pzuraq.com) for reviewing a draft of this before publication!
tags:
    - JavaScript
    - Ember
    - auto-tracking
    - things I learned
    - web development
    - software development
date: 2020-05-13T12:50:00-0600
updated: 2020-05-15T21:03:00-0600
templateEngineOverride: md

---

Yesterday, while I was double-checking some new Ember Octane code in the LinkedIn app, I ran into a scenario that surprised me—and I suspect it might surprise you, too!

Here’s a minimal version of the code that surprised me—a single component which uses `mut` (or the `set` helper from [ember-simple-set-helper]) to change a value on the backing class when an item is clicked.[^not]

Backing class (`confusing.js`):

```js
import Component from '@glimmer/component';

export default class Confusing extends Component {
  surprising = true;
}
```

Template (`confusing.hbs`):

```htmlbars
<button {{on "click"
  (fn (mut this.surprising) (not this.surprising))
}}>
  {{this.surprising}}
</button>
```

As you click the button, it will change the value from `true` to `false` and back again. (You can see this working in [this Ember Twiddle][mut-behavior].) This surprised me because `surprising` on the backing class *is not explicitly tracked*. It seems like it shouldn’t change the value! What’s more, if we changed the implementation *not* to use `mut`, but to use a regular action instead, it *wouldn’t* work!

```js
import Component from '@glimmer/component';
import { action } from '@ember/object';

export default class Confusing extends Component {
  surprising = true;
  
  @action toggle() {
    this.surprising = !this.surprising;
  }
}
```

```htmlbars
<button {{on "click" this.toggle}}>
  {{this.surprising}}
</button>
```

You can see in [this twiddle][action-behavior]: the value does not change!

I initially suspected this was a quirk with `mut`, which has a *lot* of strange behaviors, so I went back and tried it with [ember-simple-set-helper] instead.[^set] Unfortunately, I can’t share a Twiddle for this, but the implementation looks just like the `mut` version, but a bit nicer in the template:

```htmlbars
<button {{on "click" (set this.surprising (not this.surprising))}}>
  {{this.surprising}}
</button>
```

Once again, it works! So the problem is not specific to `mut`; there’s something about both `mut` and `set` which makes this work, while regular actions using normal Octane idioms *don’t* work. What’s up?

Under the hood, both `mut` and `set` use [Ember’s `set` function][set], and when templates reference values, they use [Ember’s `get` function][get]. Both of these *implicitly* auto-track the values they consume. They have to for Ember’s backwards compatibility story to hold: this is how you can freely mix Classic code and Octane code and everything “just works.”

However, as we saw above, this is a serious refactoring hazard: the second you switch from using `mut` or `set` to a normal action, everything stops working. To make this safe, we simply need to stop depending on implicit behavior of `mut` and `set`, and explicitly track the value:

```js
import Component from '@glimmer/component';
import { tracked } from '@glimmer/tracking';
import { action } from '@ember/object';

export default class Confusing extends Component {
  @tracked surprising = true;
  
  @action toggle() {
    this.surprising = !this.surprising;
  }
}
```

This has no impact on the behavior of the version using `mut` or `set`, but it is robust in the face of refactoring, and if `mut` is ever deprecated or a version of `set` is released that does *not* use the `set` function under the hood, it will keep working correctly.

:::callout

Thoughts, comments, or questions? [Discuss on the forum!][discuss]

:::

[ember-simple-set-helper]: https://github.com/pzuraq/ember-simple-set-helper
[ember-truth-helpers]: https://github.com/jmurphyau/ember-truth-helpers
[mut-behavior]: https://ember-twiddle.com/e7a1b51310ab33590e6102b25967cc46?openFiles=templates.components.confusing%5C.hbs%2Ctemplates.components.confusing%5C.hbs
[action-behavior]: https://ember-twiddle.com/562d6b5d41f58ff49ce2a014667f0e78?openFiles=templates.components.confusing%5C.hbs%2Ctemplates.components.confusing%5C.hbs
[set]: https://api.emberjs.com/ember/3.18/functions/@ember%2Fobject/set
[get]: https://api.emberjs.com/ember/3.18/functions/@ember%2Fobject/get
[discuss]: https://discuss.emberjs.com/t/mut-and-set-and-auto-tracking-in-ember-octane

[^not]: I’m assuming the existence of a `not` helper like the one from [ember-truth-helpers] here. If you don’t have that, here’s the simplest possible implementation:

    ```js
    import { helper } from '@ember/helper';
    
    export default helper(([value]) => !value);
    ```

[^set]: `set` has *better* developer ergonomics than `mut`, as you can see from this example, *and* [it avoids a lot of edges cases that `mut` has][pzuraq-post].

[pzuraq-post]: https://www.pzuraq.com/on-mut-and-2-way-binding/
