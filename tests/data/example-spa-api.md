---
Title: An Example Single-Page App API
Subtitle: >
    Implementing a "single-page" JavaScript app in ExpressJS and EmberJS.
Tags: software development
Date: 2015-07-27 08:00
Status: draft
---

A few weeks ago, I published [a short description][SPAs] of a better way to
write single-page-type applications with a two-part API, with matching page- and
data-targeting URL endpoints. Of course, as nice as that is in principle, it is
incredibly helpful to see ideas put into practice, so I've created a small
example application to demonstrate the principle.

[SPAs]: http://www.chriskrycho.com/2015/how-to-build-a-single-page-app-api-right.html

A few caveats:

  - The *structure* of the app I outline below is *not* necessarily how you should do things in production. I have intentionally kept this as bare-bones as possible so that you can see the *idea*. When structuring your application, follow the best practices of whatever tools you're using, not just random examples you found on the web (this one included). Trust me: you'll thank me later.
  - The goal of keeping this barebones, just to demonstrate the point, means I'm also leaving out a lot of other pieces that you'll have to account for in most apps---other kinds of URL endpoints, authentication and authorization,[^auth] and so on.

In short, I'm going to willfully disregard a *lot* of best practices to make
this point clear. Please don't take that as a license to do the same in your
production apps.

Those qualifications given, let's get going.

## Moving pieces

To make this work, we're going to need both a front-end and a back-end toolkit.
[ExpressJS] is basically *the* standard NodeJS server framework at this point,
so we'll use that.[^python] There is a *lot* of competition among front-end
frameworks, and while [AngularJS] is probably the most popular currently, it is
distinctly *not* my favorite---not to mention that it will very much be [in
flux] over the next few years. [ReactJS] is also super hot right now, and for
good reason. However, I'm going to use my *favorite* of the front-end frameworks
out there right now: [EmberJS]. Why? Basically, just because I like it the
best![^ember]

[ExpressJS]: http://expressjs.com
[AngularJS]: https://angularjs.org
[in flux]: https://angular.io "Angular 2.0"
[ReactJS]: http://facebook.github.io/react/
[EmberJS]: http://emberjs.com

For the sake of my own convenience and future-proofing this to some extent, I'm
writing the samples up in ES2015 (ES6), which is *much* nicer for a lot of
things.

To manage all of this, you're going to need the following:

  - [io.js] v2.0+ (anything higher than that will work; v2.3+ is preferred)
  - [Babel] (`npm install -g babel`)
  - [Ember CLI] (`npm install -g ember-cli`)

[io.js]: https://iojs.org
[Babel]: https://babeljs.io
[Ember CLI]: http://www.ember-cli.com

[^auth]: Note: these are *not* the same. Roughly speaking, *authentication* is
    determining that someone is who they allege themselves to be;
    *authorization* is determining what they have access to. These are of course
    related, but do not make the mistake of conflating them.

[^python]: I would be quite happy to do this in Python, but it's easier
    to keep things simple if we limit this to a single language.

[^ember]: Somewhat more seriously, I actually have a lot of reasons to prefer
    Ember, among them its governance model (i.e. *not* a corporate project) and
    its strong conventions. The latter is a good reason *not* to love it in some
    cases, in which case an up-and-coming toolkit you should check out is Rob
    Eisenberg's [Aurelia].

[Aurelia]: http://aurelia.io
