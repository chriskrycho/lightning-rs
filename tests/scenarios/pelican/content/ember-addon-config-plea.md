---
Title: "TODO"
Subtitle: A plea for
Date: 2016-08-27 21:00
Tags: emberjs, javascript
Status: draft
---

Over the past six months working at Olo, I've repeatedly run into scenarios where an Ember CLI add-on was *perfect* for our needs... except that it assumed that a given add-on could have configuration set at build time.

That's a pretty big "except" for us, because we use a white-labeling strategy for the applications we ship. A number of pieces of the required configuration data are supplied to the app at runtime.
