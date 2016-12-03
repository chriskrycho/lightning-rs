---
Title: How to Build a Single-Page App API <em>Right</em>
Subtitle: >
    Or: How to get performance, progressive enhancement, and SEO in your
    Ember/Angular/Backbone/etc. app for free!
Summary: >
    How to write a single-page app API so that you get usable data on the first
    load *and* have a nice interface for your single-page application built in
    Ember/Angular/Knockout/Backbone/etc.
Tags: software development, javascript
Date: 2015-06-09 22:16
---

When I was first working on HolyBible.com, I struggled for quite a while to wrap
my head around the right way to structure its API---and in truth, I actually
didn't come up with what I would call the *right* solution. I came up with a
*working* solution, and the site performs all right, most of the time. However,
our goal as developers shouldn't be "all right, most of the time." It should be
"really well, all the time." A big part of what I did wrong came from the bad
advice I found in reading up on the issue along the way. This is my shot at
helping you, dear reader, avoid making the same mistake.

## The challenge
When building a client-side application, we need to get the data for each view
so that we can render it. In the case of HolyBible.com, that means everything
from actual Bible text to study Bible notes, about pages, etc. The question is
*how* to do this: we need to be able to load an actual page from our server,
and we need a way to request data (rather than whole pages) from the server.

(More experienced developers already know where this is going: that last
sentence there has the key to this whole thing. I know. But the internet
*doesn't.* I learned this the hard way.)

### The mistake
Here's the mistake I made: I built the Bible data API as (essentially) a
*single* endpoint. When I went looking for advice on how to build this in
Angular and Node/Express, every single tutorial or blog post I found outlined
the same basic solution: routes for your data endpoints, and catch-all route
that returns the basic frame page for everything else. So, for HolyBible.com,
that would come out with route matchers for e.g. `/data/gen.1.1`, and for any
other specific routes needed (for other views, static resources, etc.), with
a default behavior of just dropping a static, basically empty template at the
catchall `*` route. Then, once the application has loaded, it can inspect the
URL and load the relevant data.

This works. It's exactly what I did on HolyBible.com, in fact. But it's *slow*.

Don't get me wrong: the time until the initial page load is actually relatively
quick (though I plan to improve it substantially over the next couple months).
The real problem is that the initial page load *doesn't include any content*.

I *hate* this. That's why people are on the site: not to see my neat skills with
JavaScript, just to read the Bible. And they have to wait, because once the page
*does* load, Angular has to spin up the full application, see what content
*should* have been loaded, and request it.

### The solution
Don't write *one* API. Write *two*. They should be structured nearly
identically, but one of them will be a *page* API endpoint, and one will be a
*data* API endpoint. In the context of HolyBible.com, here's how that would play
out.[^hb-api] One endpoint would be based purely on the standard URL, something
like `holybible.com/jhn.3.16`. The other would be to retrieve a set of *data*
associated with a given address, like `holybible.com/data/jhn.3.16`. This is
only a little different from the approach suggested above, but that small
difference matters---in fact, it matters a *lot*.

Instead of having the `/jhn.3.16` route get handled by a catchall `*` route on
the back end, it gets its own API endpoint, which looks for URLS of this shape
and hands back a full page. That API endpoint is responsible to actually render
the content of the page appropriately---in this case, with something like the
whole chapter of John 3.[^semantic] *That* gets handed back to the browser, so
the very first thing the user sees is not a blank page while the JavaScript
framework spins up and requests data, but rather *the Bible text they asked for
in the first place*.

Meanwhile, the JavaScript framework *can* spin up, and load any required session
data, etc. and start managing the UI like normal. Once we get to this point, the
framework can go ahead and request a data payload from the `/data/<reference>`
endpoint. So, for example, if there is a navigation control on the page (as
on HolyBible.com and indeed most sites), clicking to navigate to Job 14 could,
instead of requesting `/job.14.4`, fetch the data from the other endpoint by
running an AJAX request to `/data/job.14.4`.

The backend thus supplies *both* a `/<resource>` and a `/data/<resource>` route.
This might seem redundant, but we've just seen why it isn't, Moreover, if you
have any logic that needs to be in place---in our example here, a Bible
reference parser, for example, to decide what content should be supplied---you
can easily reuse it between the two routes. The differences is simply in the
form of the data returned: is it a fully-rendered template, or just the data?

## So what?
This approach has two big advantages over the catch-all approach that was
frequently recommended in e.g. Angular SPA tutorials I read.

 1. It's *progressive enhancement*. If the JavaScript fails, or the user has it
    disabled, or it fails to load because it's loaded asynchronously, the user
    still gets the page they asked for. Moreover, as long as the page content is
    build carefully (links built appropriately for other content, and so on),
    the entire application could continue to work even if the JavaScript *never*
    becomes available.

 2. It's *performant*. Loading the content this way will be *much* faster than
    the standard approach recommended for single-page apps. As noted above, it
    gets the content to the user immediately, then lets the JavaScript UI bits
    come into play. Since future page loads can take advantage of both caching
    and smaller data payloads, the whole thing can actually be faster than
    either a pure client-side *or* a pure server-side approach. That is, once
    the client-side application is running, it can just update its views with
    data delivered via AJAX, rather than reloading the whole page. But *before*
    that, the user doesn't have to wait to see something useful until the
    JavaScript framework spins up.

It's not often an approach gives you progressive enhancement and actually
increases the performance of an application, but this one does. Better yet, you
can apply this in just about any framework: it's equally applicable to AngularJS
with ExpressJS, Backbone with Rails, Ember with Django, Aurelia with Phoenix, or
any other combination you come up with.

[^hb-api]: Note: this is *not* the actual API structure of HolyBible.com, or
    even particularly close to it. Remember, I learned everything I'm writing
    here by doing it *wrong*.

[^semantic]: Or possibly a section which constitutes a semantic block of data.
    I have some thoughts on chunking Bible data semantically rather than by
    chapter and verse for this kind of thing. That's another post for another
    day, though.
