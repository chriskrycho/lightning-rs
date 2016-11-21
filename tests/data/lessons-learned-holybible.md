---
Title: Lessons Learned
Subtitle: 9 Months With HolyBible.com
Date: 2015-04-12 13:49
Tags: software development, javascript, angularjs
...

Since mid July 2015, I have been working on a complete redesign and re-build of
[HolyBible.com]. The good folks at [Puritan Reformed Theological Seminary] who
own the site wanted to replace its previous content with a Bible reading tool.
While there's still a lot to wrap up, the project is *nearing* its conclusion,
and I thought I'd note a few things I've learned (in some cases, learned
*again*) along the way. I want to say up front, lest these be taken the wrong
way: I'm extremely proud of the work I've done, and the application I've
delivered *does* work to the specifications I was hired to meet. More than that,
it does it well. But, of course, it could do it *better*. The following thoughts
are therefore not, "How I failed" but rather "How I will do this *even better*
next time around."

[HolyBible.com]: //holybible.com
[Puritan Reformed Theological Seminary]: //prts.edu

 1. *Single page apps are great, but not always the right choice.* I made the
    decision, based on my expectations and understandings of what I would need,
    to develop the site as a single-page web application. This was a mistake.
    Not the worst mistake ever: it has its upsides, including performance *once
    the app spins up*, but for the kind of content I have here, I would take a
    different tack today. Better in this case to deliver static content and
    *update* it dynamically as appropriate than to try to load all the content
    dynamically every time.

    At a technical level, that would probably mean supplementing standard HTML
    with [Backbone] instead of developing it as a single-page app in [Angular].
    For the backend, while I did it in Node.js and that would work fine, I'd
    probably do a straight Django app (especially with a few of the goals I
    learned about *after* the project was well along in development).

 2. *Progressive enhancement or graceful degradation are hard in web
    applications, but they still matter.* In the past, I've always taken a hard
    line on making sure things either degrade gracefully or are simply enhanced
    by JavaScript content. In the architecture decisions I made for this app, I
    failed to take that into account (largely because I thought it would just
    *need* to work as a web app, but see above). I regret that enormously at
    this point; it would be much better in this particular case to have content
    available even if the additional functionality doesn't work. Even if you
    *are* doing something where you are building an *app*, finding ways to make
    it work on poor connections, older browsers, etc. matters. I'm still
    thinking a *lot* about the best way to do this in the future.

 3. *More popular doesn't mean better.* Angular has a ton of traction and
    uptake, and that was deceptive early on. I won't so easily be fooled in the
    future. Angular is so very popular in part because Google can put serious
    money behind its development---and its marketing. But it's *not* the best
    for many applications; if you're not in the business of developing your own
    custom framework, it's not even *close* to the best. Use Ember or Knockout
    or any number of other full-stack frameworks rather than a meta-framework.

    How to avoid making that mistake? Well, for my part since then, I've learned
    to look not just as the *quantity* of material in a given community, but its
    *quality*. For example, [Ember] has *incredible* documentation (far better
    than Angular's), and they also have a much clearer vision and a more
    dependable approach to development (strict semantic versioning, etc.). Had I
    taken the time to read *both* sets of docs more carefully and think through
    the consequences of their designs more thoroughly, I could have recognized
    this before starting. Next time, I will do just that.

    I will also look at the way the community behaves. The Ember community is
    *far* friendlier for newcomers from what I've seen than the Angular
    community---no slam meant on the Angular crowd, but the Ember folks are just
    doing that really well. That matters, too. (I can't speak for other
    communities, of course; these are just the groups I've watched the most.)

    All in all, Ember would have been the better fit between these two (even
    though, as noted above, it also wouldn't have been the *best* fit).

 4. *Unit tests really are the best.* I did a vast majority of this project with
    unit tests---the first time I've ever been able to do that for a whole
    project. In other projects, I've been able to do it for parts, but never
    this much. It saved my bacon a *lot*. Where I got in a hurry and felt like I
    didn't have time to write the tests, I (inevitably and predictably!) ended
    up spending a lot of time chasing down hard-to-isolate bugs---time I could
    have avoided by writing well-tested (and therefore better-factored) code in
    the first place. Lesson learned *very* thoroughly. Server- and client-side
    unit tests are *really* good. They're also sometimes *hard*; getting mocks
    set up correctly for dealing with databases, etc. can take a while. That
    difficulty pays for itself, though.

 5. *Unit tests **really** don't replace API documentation.* I have seen people
    advocate test-driven-development as a way of obviating the need to do major
    documentation of an API. This is, in a word, ridiculous. Having to read unit
    tests if you want to remember how you structured an API call is a pain in
    the neck. Don't believe it. Design your API and document it, *then* do
    test-driven development against that contract.

 6. *Sometimes 'good enough' is enough.* There is always more to be done, and
    inevitably you can see a thousand things that could be improved. But 'good'
    shipping code is far more valuable than 'perfect' code that never ships. You
    should never ship *bad* code, but sometimes you do have to recognize 'good
    enough' and push it out the door.

 7. *Full-stack development is fun, but it's also really hard.* I wrote every
    scrap of code in HolyBible.com proper (though of course it relies on a lot
    of third-party code). It was very, very difficult to manage that all by
    myself; it's a lot to hold in one's head. (One of the reasons I chose Node
    was because keeping my implementation and testing all in one language helped
    reduce that load somewhat.) Would I do it again? Sure. But very much
    chastened about the difficulties involved. It has been enormously rewarding,
    and I *like* being a full-stack developer. But it's a lot of work, and now I
    know more clearly just how much.

[Backbone]: //backbonejs.org
[Angular]: //angularjs.org
[Ember]: //emberjs.com

I could say a great deal more about the technical side of things especially, but
my biggest takeaway here is that a lot of the hardest and most important work in
developing software has nothing to do with the code itself. Architecture and
approach shape *far* more than the implementation details (even if those details
still matter an awful lot). And popularity is not at all the same as either
*quality* or (especially) *suitability for a given task*. In the future, I will
be better equipped for the necessary kinds of evaluation, and will hopefully
make still better decisions accordingly.
