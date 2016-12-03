---
Title: Testing Ember.js Mixins With a Container
Subtitle: Fixing "Attempting to lookup an injected property on an object without a container" errors in mixin tests.
Date: 2016-06-09 20:35
Modified: 2016-06-10 07:30
Tags: emberjs, javascript, software development
Category: tech
---

Today I was working on an Ember.js [mixin] for the new mobile web application we're shipping at Olo, and I ran into an interesting problem when trying to test it.

[mixin]: http://emberjs.com/api/classes/Ember.Mixin.html#content
[\@rwjblue]: https://github.com/rwjblue
[Ember Community Slack]: https://ember-community-slackin.herokuapp.com

When you're testing mixins, you're generally not working with the normal Ember container.[^container] In fact, the default test setup for mixins doesn't have *any* container in play. It just looks like this (assuming you ran `ember generate mixin bar` in an app named `foo`):

```js
import Ember from 'ember';
import BarMixin from 'foo/mixins/bar';
import { module, test } from 'qunit';

module('Unit | Mixin | bar');

// Replace this with your real tests.
test('it works', function(assert) {
  let BarObject = Ember.Object.extend(BarMixin);
  let subject = BarObject.create();
  assert.ok(subject);
});
```

Note two things:

1. It uses the basic Qunit `module` setup, not the ember-qunit `moduleFor` setup.
2. It assumes you're generating a new object instance for every single test.

Both of those assumptions are fine, *if you don't need to interact with the container*. In many cases, that's perfectly reasonable---I'd go so far as to say that most mixins probably *shouldn't* have any dependency on the container.

In the specific case I was working on, however, the point of the mixin was to abstract some common behavior which included all the interactions with a [service]. This meant making sure the dependency injection worked in the unit test. This in turn meant dealing with the container. So let's see what was involved in that.

[service]: https://guides.emberjs.com/v2.6.0/applications/services/

We start by switching from the basic `qunit` helpers to using the `ember-qunit` helpers.

```js
// Replace this...
import { module, test } from 'qunit';
module('Unit | Mixin | bar');

// with this:
import { moduleFor, test } from 'ember-qunit';
moduleFor('mixin:bar', 'Unit | Mixin | Bar');
```

The `moduleFor()` helper has two things going for it---one of which we *need*, and one of which isn't strictly *necessary*, but has some nice functionality. In any case, this will help when registering a container. Those two features:

1. It does support the use of the container. In fact, it's declaring how this mixin relates to the container in the first argument to the helper function: `'mixin:foo'` is the definition of the mixin for injection into the container.
2. Any functions we define on the options argument we can pass to the `moduleFor()` helper are available on the `this` of the test.

Now, in the first version of this, I had set up a common `Ember.Object` which had mixed in the `BarMixin`, so:

```js
const BarObject = Ember.Object.extend(BarMixin);
```

Then, in each test, I created instances of this to use:

```js
test('test some feature or another', function(assert) {
  const subject = BarObject.create();
  // ...do stuff and test it with `assert.ok()`, etc.
});
```

The problem was that any of those tests which required a container injection always failed. Assume we have a service named `quux`, and that it's injected into the mixin like this in `foo/app/mixins/bar.js`:

```js
import Ember from 'ember';

export default Ember.Mixin.create({
  quux: Ember.inject.service()
});
```

Any test which actually tried to *use* `quux` would simply fail because of the missing container (even if you specified in the test setup that you needed the service):

```
test('it uses quux somehow', function(assert) {
  const subject = BarObject.create();
  const quux = subject.get('quux');  // throws Error
});
```

Specifically, you will see `Attempting to lookup an injected property on an object without a container` if you look in your console.

Taking advantage of the two `ember-qunit` features, though, we can handle all of this.

```js
import Ember from 'ember';
import { moduleFor, test } from 'ember-qunit';

const { getOwner } = Ember;

moduleFor('mixin:bar', 'Unit | Mixin | bar', {
  // The `needs` property in the options argument tells the test
  // framework that it needs to go find and instantiate the `quux`
  // service. (Note that if `quux` depends on other injected
  // services, you have to specify that here as well.)
  needs: ['service:quux'],

  // Again: any object we create in this options object will be
  // available on the `this` of every `test` function below. Here,
  // we want to get a "test subject" which is attached to the
  // Ember container, so that the container is available to the
  // test subject itself for retrieving the dependencies injected
  // into it (and defined above in `needs`).
  subject() {
    BarObject = Ember.Object.extend(BarMixin);

    // This whole thing works because, since we're in a
    // `moduleFor()`, `this` has the relevant method we need to
    // attach items to the container: `register()`.
    this.register('test-container:bar-object', BarObject);

    // `Ember.getOwner` is the public API for getting the
    // container to do this kind of lookup. You can use it in lots
    // of places, including but not limited to tests. Note that
    // that because of how the dependency injection works, what we
    // get back from the lookup is not `BarObject`, but an
    // instance of `BarObject`. That means that we don't need to
    // do `BarObject.create()` when we use this below; Ember
    // already did that for us.
    return getOwner(this).lookup('test-container:bar-object');
  }
});

test('the mixin+service does what it should', function(assert) {
  // We start by running the subject function defined above. We
  // now have an instance of an `Ember.Object` which has
  // `BarMixin` applied.
  const subject = this.subject();

  // Now, because we used a test helper that made the container
  // available, declared the dependencies of the mixin in `needs`,
  // and registered the object we're dealing with here, we don't
  // get an error anymore.
  const quux = subject.get('quux');
});
```

So, in summary:

1. Use the `ember-qunit` helpers if you need the container.
2. Define whatever dependencies you have in `needs`, just as you would in any other test.
3. Register the mixin-derived object (whether `Ember.Object`, `Ember.Route`, `Ember.Component`, or whatever else) in a method on the options argument for `moduleFor()`. Use that to get an instance of the object and you're off to the races!

One final consideration: while in this case it made good sense to use this approach and make the service injection available for the test, there's a reason that the tests generated by Ember CLI don't use `moduleFor()` by default. It's a quiet but clear signal that you should reevaluate whether this *is* in fact the correct approach.

In general, mixins are best used for self-contained units of functionality. If you *need* dependency injection for them, it may mean that you should think about structuring things in a different way. Can all the functionality live on the service itself? Can all of it live in the mixin instead of requiring a service? Can the service calls be delegated to whatever type is using the mixin?

But if not, and you *do* need a mixin which injects a service, now you know how to do it!

[^container]: If you're not familiar with the "container", this is where all the various dependencies are registered, and where Ember looks them up to inject them when you use methods like `Ember.inject.service()`.

---

**Side note:** The documentation around testing mixins is relatively weak, and in general the testing docs are the weak bits in the Ember guides right now.[^docs] After a conversation with [\@rwjblue] on the [Ember Community Slack], though, I was able to get a handle on the issue, and here we are. Since it stumped me, I'm guessing I'm not the only one.

When this happens, *write it up*. I've been guilty of this too often in the past few months: learning something new that I couldn't find anywhere online, and then leaving it stored in my own head. It doesn't take a particularly long time to write a blog post like this, and if you're stuck, chances are *very* good someone else is too.

[^docs]: Something I intend to help address in the next week or two via a pull request, so if you're my Ember.js documentation team friend and you're reading this... it's coming. 😉
