---
Title: HTML5 Location, <code>&lt;base&gt;</code>, and SVG
Date: 2015-06-20 10:30
Subtitle: >
    Making Angular and SVG <code>xlink:href</code> work correctly in Firefox.
Summary: >
    Angular requires <code><base></code> if you want to use HTML5's
    <code>location</code>… but if you get it wrong, SVG things can and will
    break under you.
Tags: software development, angularjs, javascript
Modified: 2015-07-02 22:00
...

For quite some time, I have been frustrated by a bug in HolyBible.com: Firefox
would not render SVGs using the `<use xlink:xhref="#some-SVG-ID"></use>`
pattern. Today, I set aside my ongoing work on new user-facing functionality
and dedicated what working time I had to hunting down the cause of this and
fixing it at last.

I was surprised to find the culprit: the `<base>` tag. If you don't know what
the `<base>` tag is, you're not alone. It is *not* used all that much in
general, and I had never actually seen it on a site before starting on this
project last year.

So what went wrong? How do these two things play together?

I am using (and reusing) SVG items throughout the HolyBible.com interface,
taking advantage of the ability to define symbols and reference them with the
`<use>` tag, like so:

```html
<svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" xmlns:ev="http://www.w3.org/2001/xml-events" style="display: none">
  <symbol id="logo-shape" viewBox="0 0 256 256">
    <title>Logo</title>
    <desc>The HolyBible.com logo: sunrise breaking over an open book (the Bible).</desc>
    <path id="logo-light" d="M172.1 116.3l5.1-4.1-12.5-.5 32-26.3-41.4 18.4 11-20.1L148 96l12.2-37.5L138.8 91l.1-36.2-10.3 34.4L114 36.1l4.3 54.9-22.2-34.9 13 39.9-18.3-12.4 11 20.1-42.5-19.2 32.8 26.9-10.4.8 4.4 3.9c13.1-1.6 27.4-2.7 42.4-2.7 15.4 0 30.1 1.2 43.6 2.9z"/>
    <path id="logo-book" d="M199.9 219.9c-47.4-9.8-96.4-9.8-143.8 0-6-28.9-12-57.7-17.9-86.6 59.3-12.3 120.4-12.3 179.7 0-6 28.9-12 57.8-18 86.6z"/>
  </symbol>
</svg>

<!-- somewhere else on the page -->
<svg>
  <use xlink:href="#logo-shape"></use>
</svg>
```

Throughout all my early prototyping, this worked perfectly across all modern
browsers. (For more, see [CSS Tricks].) Now, when I started moving from the
prototype phase into actually building the application in Angular last fall, I
learned that you have to set the base URL for the application using the `<base>`
tag to use the HTML5 Location API with Angular 1.x. If you want URL-based,
rather than `#`-based navigation in an Angular app, you need this. Following the
recommendation of whatever documentation and tutorials I found, I set it so:

[CSS Tricks]: https://css-tricks.com/svg-sprites-use-better-icon-fonts/

```html
<base href="/">
```

Again, this was the recommendation I saw in every bit of documentation and every
tutorial, so I assumed it would have no problems. As it turns it, that's not
the case. (This is a [recurring theme] in my experience with Angular.) In
Chrome, Safari, and IE9+, this works exactly as expected. In Firefox, however,
it does *not*. The use of the `<base>` tag changes the behavior of `#`-based
URLs on a page. Specifically, it makes it so that if you're at a URL that
*isn't* the base route, anchor links don't behave as expected. In order to make
the `<use>` tag as expected, we would have to use the same URL as the base tag.
Among other things, this would require making sure that any place we used the
`<use>` tag, we would have to set that---not exactly a good idea, given that it
would entail an awful lot of changes if the base URL were ever changed.

[recurring theme]: http://www.chriskrycho.com/2015/how-to-build-a-single-page-app-api-right.html

What if, instead, we did this?

```html
<script>document.write('<base href="' + document.location.origin + '" />');</script>
```

This way, when the page renders, it writes the document location based on the
*current* location. The URL history still behaves as expected with Angular, but
the relative URLs for IDs behave as expected in Firefox again, while not
breaking the behavior in any other browsers.

But... then you'll navigate to another page, and Firefox will be back to not
working.

The [solution], it turns out, only came into being after I'd done the initial
implementation, and I have no idea how much later it found its way into the
Angular docs. However, even though it now *exists* in the docs, it's by no means
obvious why you should do it this way, and certainly no mention of SVG! This
might not seem odd to you... but it should, given that the only reason that
Angular introduced this API change was to account for *exactly this issue*.[^1]

[solution]: https://github.com/angular/angular.js/issues/8934#issuecomment-56568466

As the Angular docs note, leaving out the `<base>` tag means all your URLs have
to be absolute if you want to use HTML5 location and the `$locationProvider`. If
you want to use SVGs with `<use>` and Firefox, though, that's what you have to
do (and therefore that's what I'm doing).

Fun times, right?



[^1]: The closest it gets is this reference:

    > Links that only contain a hash fragment (e.g. `<a href="#target">`) will
    > only change `$location.hash()` and not modify the url otherwise. This is
    > useful for scrolling to anchors on the same page without needing to know
    > on which page the user currently is.

    Even this, however, only *hints* at the root of the SVG issue.
