---
title: How I Publish This Site
subtitle: Explaining how I run this site—everything.
summary: Explaining how I run this site—everything.
date: 2020-02-09T21:00:00-0600
updated: 2020-11-28T11:11:00-0600
qualifiers:
    audience: >
        People interested in the nerdy details of how to get a website like this up and running. Here I get into everything from getting a domain and setting up <abbr>DNS</abbr> to how I use Markdown and Git!
tags:
    - blogging
    - writing
    - Markdown
    - websites
    - domain
    - DNS
    - Netlify
    - Cloudflare
    - Hover
    - own your turf
    - IndieWeb

---

On seeing this site relaunch back in November, my friend [John Shelton](https://sites.google.com/site/iamjohnshelton/home) asked if I had anywhere I’d listed out the whole of my setup for hosting this site. The answer is: I hadn’t, but as of *now* I have!

If you want the *super* short version, this is it (with the topics covered in this post marked with a <b>\*</b>):

- <b>\*</b> The domain name is registered at [Hover][Hover].
- <b>\*</b> The DNS runs through [Cloudflare.com][Cloudflare].
- The site is generated with  [11ty][11ty], with—
    * a mix of [Nunjucks], JSON, and TypeScript for the templating
    * a *very* light use of [SCSS] to generate the CSS
    * a bunch of custom filters and plugins, also written in TypeScript
- The fonts are licensed from  [Fonts.com](http://fonts.com/) (purchased and self-hosted) and  [fonts.adobe.com](http://fonts.adobe.com/) (hosted).
- <b>\*</b> The content—written entirely in [Markdown]—lives in Git repositories which I maintain on copies of on all my machines as well as on [GitHub.com][gh].
- <b>\*</b> The site is deployed via [Netlify.com][netlify].
- <b>\*</b> I actually *write* using a(n ever-changing) mix of text editors, currently primarily [1Writer] on iOS and [Byword] and [Caret] on macOS.

If you want the longer version, read on. In this post, I will trace out the details of how I get this site to you. In a follow-on post I will hopefully write as a follow-up sometime this year, I’ll dig into the technical details of how the site is put together. (*Hopefully*, I say, because I started *this* post three months ago!)

I should clarify, before I go any further: this is *not* a stack I would recommend to anyone else who’s not a total nerd, though this same basic *kind* of stack is workable with a much lower degree of effort than I put in. You need to be willing to do a *small* amount of semi-technical work; you *don’t* have to build an entire site from scratch like I did. The support for normal CMS interfaces to this kind of setup has grown enormously in the past few years, and it can actually be a really good, very lightweight experience.[^cms]

[Hover]: https://hover.com/
[Cloudflare]: https://cloudflare.com/
[11ty]: https://11ty.io/
[Markdown]: https://daringfireball.net/projects/markdown/
[Nunjucks]: https://mozilla.github.io/nunjucks/
[SCSS]: https://sass-lang.com
[gh]: https://github.com/chriskrycho/v5.chriskrycho.com
[netlify]: https://netlify.com/
[1Writer]: http://1writerapp.com/
[Byword]: https://www.bywordapp.com 
[Caret]:  https://caret.io/ 

*[DNS]: domain name system
*[JSON]: JavaScript Object Notation
*[SCSS]: Sassy CSS
*[CSS]: Cascading Style Sheets
*[CMS]: content management system

[^cms]: I’ve experimented a bit with both [Forestry] and [Netlify CMS]. I mentioned in my relaunch announcement post that I was leaning toward Netlify CMS because it would in principle allow me to allow *anyone* to suggest edits to my site. That didn’t end up panning out; I explain why below.

[Forestry]: https://forestry.io
[Netlify CMS]: https://www.netlifycms.org

<!-- omit in toc -->
## Outline

- [Costs](#costs)
- [Writing](#writing)
    - [Why Markdown?](#why-markdown)
- [Workflow](#workflow)
    - [Where the content lives](#where-the-content-lives)
    - [How the content gets to you](#how-the-content-gets-to-you)
    - [CMS](#cms)
- [Domain registration](#domain-registration)
- [DNS: Cloudflare](#dns-cloudflare)
- [Summary](#summary)

## Costs

My costs are pretty low for this setup. Cloudflare is free for setups like mine. GitHub is free for setups like mine. Netlify is free for setups like mine. The code font, [Hack][hack], is *also* free. (Sensing a theme here?)

In terms of things I *do* actually pay for (or have in the past), though:

- I pay $15/year for the domain at Hover.

- I paid a few hundred dollars to perpetually license [Sabon][sabon] (the body text) a few years ago—both for the web and for desktop work. I get [Cronos][cronos] via my $10/month for Adobe’s Lightroom package, which includes Adobe Fonts. (This is the piece here that stings the most in terms of ongoing costs, but Lightroom is *fabulous*, so I’m just rolling with it at this point.)

- As will become clear in the *next* section, I have spent a… *non-trivial*… amount of money on writing applications over the last decade.

In general, I’m not opposed to paying for good services—actually, very much to the contrary!—but if there is a good service with a freemium model and I fit comfortable in the free tier, I’m happy to use it.

[hack]: https://sourcefoundry.org/hack/
[sabon]: https://www.fonts.com/font/linotype/sabon
[cronos]: https://fonts.adobe.com/fonts/cronos

## Writing

These days I do my writing in a wild hodgepodge of tools. None of them thrill me, because all of them do *some* things really well… and leave others in a “ugh, not quite there” state. For example, this particular paragraph I’m drafting in [Byword]—my old standby, an app I’ve been using for over half a decade now. It remains a rock-solid, very lightweight and very *fast* editor with just the right level of minimal Markdown support, and I love it for that. If I’m just writing a blog post like this, and I’m on macOS, Byword is still the app I’m most likely to reach for.

However, when I am working on code samples, it leaves a few things to be desired. For that, I turn to [Caret]—a more recent discovery, and one that lacks Byword’s light weight and phenomenal performance, but which is tuned to the writing *programmer*. At this point I’m using the [latest beta][caret-beta] they released… about a year ago. They’ve since [declared][caret-tweet] their intention to build something new and better using some of the same tech that underpins Caret. The *big* downside for Caret is that it’s an [Electron] app, and that means that it just *is* slower and heavier than Byword—inevitably.

Also in the “unfortunately slower than Byword” are two other tools I reach for on both macOS and iOS: [iA Writer] and [Ulysses]. Ulysses in particular I tend to reach for when working on *large projects*. So, for example, when I built out the teaching materials for [the Sunday School class I taught last summer][christology-class], I wrote the majority of it in Ulysses. It’s a much better *project*-focused writing tool than any of the others, though iA Writer has gotten much closer in the last few years. The big differentiator between iA Writer and Ulysses is that, both for good and for ill, Ulysses does more magic. You can build out a project in iA Writer, and the ways it does it works with other tools too… but you do it *manually*. Ulysses’ way of working with Markdown is much more bespoke, but it Just Works.[^just-works]

As mentioned, though, both iA Writer and Ulysses are *slower* (and just feel heavier) than I’d like. As a result, I *also* have dedicated apps I reach for on iOS for one-off posts. While I love the experience of writing in Byword there no less than I do on macOS, I pretty much never use it, for one critical reason: it doesn’t integrate nearly as nicely as some of the other options with the [document provider locations][macstories] introduced and increasingly polished in the last few versions of iOS. Instead, I end up using [1Writer] almost exclusively for one-off posts like this one. It lets me much more quickly open and interact with not only iCloud folders but—and this is the real key—Git locations exposed by [Working Copy]. (For more on this, see the [<b>Workflow</b>](#workflow) section below!)

Finally, I will admit that I *do* in fact do *some* of my writing in [Visual Studio Code][vs-code]. It’s *really* not a great writing environment, but it has some really nice tools for Markdown editing. In particular, I use [an extension][all-in-one] that automates everything from generating and maintaining a table of contents (like the one above) to table formatting. It also makes for a nice environment for working with code samples.

### Why Markdown?

I have been writing *everything* in Markdown since 2012. Every blog post, every paper I wrote for seminary, every essay I have published elsewhere—even if it ended up in a Word document or published on a WordPress site, I *drafted* it in Markdown, in one of these tools. I chose Markdown a long time ago because it fits so well with the way I write. It gets out of my way by dint of its simplicity and its complete *lack* of formatting at the time of authoring. That makes the writing process much better for me, and always has. The fact that at the end I can generate *any* kind of document I need from the same source—HTML, Word, PDF, LaTeX, you name it—is all the better.

Equally important to me at this point though: writing in Markdown means I am writing in a plain text document. I can write it in any text editor anywhere which handles UTF-8—and while UTF-8 isn’t *perfectly* portable as far as document formats go, it is literally the closest thing to it which exists—right down to and including the fact that it can and does handle any language I can throw at it. (καιρε, שלמ, buenos días!)

[Byword]: https://www.bywordapp.com
[Caret]: https://caret.io
[caret-beta]: https://github.com/careteditor/releases-beta/releases
[caret-tweet]: https://twitter.com/careteditor/status/1136198029357264896?s=20
[Electron]: https://www.electronjs.org
[iA Writer]: https://ia.net/writer
[Ulysses]: https://ulysses.app
[christology-class]: https://v4.chriskrycho.com/2019/christology-god-with-us-and-for-us.html
[macstories]: https://www.macstories.net/stories/ios-11-the-macstories-review/14/
[1Writer]: http://1writerapp.com
[Working Copy]: https://workingcopyapp.com
[vs-code]: https://code.visualstudio.com
[all-in-one]: https://github.com/yzhang-gh/vscode-markdown

[^just-works]: One of the goals I have for [rewrite] is for it to *feel* as good to use as Ulysses while being as interoperable as iA Writer for project management, and as fast and lightweight as Byword. This may prove impossible… but it’s a goal.

[rewrite]: https://rewrite.software

*[UTF]: unicode transformation format

## Workflow

My publishing work flow feels relatively straightforward to me at this point, but that’s entirely a function of the fact that I’ve been using a variant on this same approach for over half a decade now, and that it’s a *programmer’s* work flow.

### Where the content lives

When I write all that Markdown material, it goes in one of two places, depending on how far along in the process I am. If it’s a big post or essay that I don’t intend to publish for a *long* time yet, I just keep it in an iCloud Drive folder which has a bunch of work-in-progress material like that. That makes it easy to work on from any of those writing tools I mentioned above. Once I’m getting closer to publishing, I move it into the [Git] repository where the entire site lives. I have copies of that on every machine I use, as well as [on GitHub][gh].

<aside>

I use GitHub as a convenient tool for coordination, but (and this is important to me!) it is *not* a single point of failure. If it goes away tomorrow, I’ll be fine. All of the content lives somewhere else, too. I have multiple backups of those copies—in iCloud, in Time Machine backups, and in [Backblaze]. This is another advantage to just using simple text files: backups are *super* easy. If I were hosting this in Ghost or WordPress or another CMS like that, I would need to make regular backups of those databases and set up automated exports for them. Doing it the way I do, I get backup in quintuplicate with *zero* extra effort on my part compared to what I do to back up my data in general.

</aside>

I usually create a Git <i>branch</i> for new posts before I publish them, so that I can take of some of the Netlify features described in the next section. On a Mac, I use mostly the command line and occasionally the [Fork] Git UI for interacting with the repository. On iOS, I use [Working Copy] to interact with the repository. It exposes the repository as a location which I can open from other apps which support document provider locations:

<figure>
<img src="https://cdn.chriskrycho.com/file/chriskrycho-com/images/how-i-publish/document-picker.jpeg" style="max-width: 416px" />
<figcaption>Opening a Working Copy location from 1Writer</figcaption>
</figure>

Then I can work with it directly in any app which has mounted the folder. For example, viewing this very post in iA Writer:

<figure>
<img src="https://cdn.chriskrycho.com/file/chriskrycho-com/images/how-i-publish/ia-writer-view.jpeg" style="max-width: 416px" />
<figcaption>Viewing a Working Copy-supplied item in iA Writer</figcaption>
</figure>

When I’m done, I can just commit that file to the repository on whatever branch I’m working in and push it up to GitHub, and it will trigger the publication workflow that I built with Netlify (described in the next section).

<figure>
<img src="https://cdn.chriskrycho.com/file/chriskrycho-com/images/how-i-publish/commit.jpeg" style="max-width: 416px" />
<figcaption>Committing a change in Working Copy</figcaption>
</figure>

I had used a similar approach in the past for managing the content and design of the site, but it was never a *full* workflow because I couldn’t use it to *publish* the site. For that, I needed to switch up how I published the site. So: Netlify!

<aside>

If you want a *truly* deep dive on this approach with iOS, see Federico Viticci’s [writeup at MacStories][macstories]. This is where I originally learned this workflow was even possible!

</aside>

[Git]: https://git-scm.com
[Backblaze]: https://www.backblaze.com
[Fork]: https://fork.dev

### How the content gets to you

I use [Netlify] to actually host and deploy the site. Netlify is a developer-focused tool, which makes it *super* easy to take a Git repository which defines how to build a website with some basic tools, and turn it into a deployed website.

In the past, I have used [GitHub Pages][ghp] to publish various websites. I have also done plain-old static file hosting on a server I own, deployed via SFTP. These options are fine so far as they go; in some cases they’re actually great choices. However, for my purposes, any kind of purely static file hosting approach meant that I *had* to have access to a laptop or desktop or server machine of my own to actually *build* the site before I published it. Netlify solves that problem by supporting a *build* step before deploying the content. For me, that means running my 11ty build step (on which see [below] for more details).

Netlify is also just exceedingly pleasant and easy to use—the setup for my site was a matter of pointing and clicking a few buttons to tell it what GitHub repository to use, and filling in one text field to tell it how to build my site. In fact, if I weren’t *particular* about separating my DNS from my hosting/deployment setup (as discussed below), I could do *that* on Netlify as well, and that is *also* an incredibly simple setup process.

One of the Netlify features I particularly love—and which I make heavy use of for *most* posts for this site, but especially for longer or more involved ones—is its [deploy previews]. Each deploy preview is a specific URL that is not indexed by search engines, but which is available pretty much permanently (as long as Netlify itself is around). I can hand that URL to someone else to read the post before it’s live in the world and they can give feedback on the thing *exactly* as it will appear. For example, the preview for *this* post was [here][preview-this].

This is handy for content, of course, but it was even handier during the design process for the site, when I could set up two options with different approaches to a particular design question, each with its own URL, and share them for others to give feedback.

[below]: #site-generator
[ghp]: https://pages.github.com
[deploy previews]: https://docs.netlify.com/site-deploys/overview/#branches-and-deploys
[preview-this]: https://deploy-preview-41--v5-chriskrycho-com.netlify.com/journal/how-i-publish-this-site/

*[SFTP]: secure file transfer protocol
*[URL]: universal resource locator

### CMS

I don’t normally *need* a CMS, but I do like to have the option. Historically, there were not great options in terms of an interface for writing and managing content… unless you wanted a setup more like WordPress or Ghost: a server application with a database, and a server to run it on. I have a preference (admittedly a bit strange) for simple text files to be the “source of truth” for the content on my website.[^pdfs-etc] For the last few years, I got by managing everything just via command line tools and building everything on my home machine.

Two things have changed. First: as I noted above, I now deploy everything via Netlify, and I don’t *need* to build it on my local machine. Second, though, the last few years have seen the advent of some decent <abbr>CMS</abbr>es for statically-generated sites like this one! The two best options I found at this point are [Forestry] and [Netlify CMS]. Each has its upsides and its downsides; in the end, for these purposes, I reach for Forestry if I *really* need it… but mostly just don’t reach for either.

Forestry has far and away the better UI of the two. In fact, it has such a reasonable UI that [my friend Stephen][sc] said of it:

> Wow. I am impressed with this CMS.
> 
> It ... it makes sense. It's laid out like ... normal people would lay it out. I shouldn't be so shocked, but lo, I'm shocked.

He’s not wrong. Most CMS user interfaces are *not good*. (The best I can say for [WordPress] is that I’ve gotten used to it. [Ghost] is pretty good, but unfortunately doesn’t work for the exact workflow I described above.) That goes double for viewing them on mobile devices, and Forestry’s mobile view is actually quite good! The experience of writing in Forestry is also good, even on iOS, which is *very* unusual for web text editors—even more unusual than just working there at all. Unfortunately, though, it doesn’t support working with Git *branches*, only working with the single “master” branch of the repository. This makes it a non-starter for drafting totally new work at this point, as (at least for now) committing to `master` *publishes the post*!

Netlify CMS handles that particular problem well via its [Editorial Workflow]! However, where Forestry’s CMS UI is one of the *best* of its sort, Netlify CMS is… not. For one thing, it simply does not even try when it comes to mobile devices—not for displaying and certainly not for editing. Given that this is the context where I’m *most* apt to want a CMS, this makes it a non-starter on *that* end.

The other reason I was particularly interested in Netlify CMS is that its Editorial Workflow supports [Open Authoring], and when I was initially doing my research, I *thought* this would allow me to accept corrections from *any* user. Unfortunately, as I dug into the docs, I found my initial reading to have been wrong. In fact, it just means that—

> you can use Netlify CMS to accept contributions from GitHub users without giving them access to your repository.

This is fine so far as it goes, but if the users *already* have to be GitHub users, well… then the GitHub UI gets the job done well *enough* and doesn’t involve keeping another dependency up to date. There is a workaround using their Git Gateway workflow, but while you can restrict what kind of mischief users can get up to, it still looked like an invitation for would-be makers-of-mayhem to make for a very annoying day.[^obnoxious] As such I just ended up making the links for editing a post take users straight to GitHub. It’s not perfect, but it’s good enough.

The net of all of this is that I had Forestry enabled for a while but eventually removed it. My Git-based workflow works well *enough* from any device, and works better than any CMS option I tried, that it wasn’t worth the hassle (however small). If you know of other good headless CMS systems which can just slot into this kind of Git-based workflow, I’d actually love to hear about them! I may or may not end up reaching for them with *this* site, but I run other sites in relatively similar ways, and it would be nice for *those*.

[WordPress]: http://wordpress.org
[Ghost]: https://ghost.org
[sc]: https://stephencarradini.com
[Editorial Workflow]: https://www.netlifycms.org/docs/configuration-options/#publish-mode
[Open Authoring]: https://www.netlifycms.org/docs/open-authoring/

[^pdfs-etc]: I like being able to generate things which *aren’t* web pages from my content sometimes!

[^obnoxious]: Internet users can be obnoxious. When one of my posts hit the top of Hacker News a few months ago, I had people “signing up” for [rewrite] updates with email addresses—I kid you not—like `dont-be-arrogant@rewrite.software` and `dont-advertise-your-own-software@chriskrycho.com`. Dumb? Very. The internet is *full* of dumb.

*[UI]: user interface

## Domain registration

I buy *all* my domains at [Hover].[^not-hover] I first tried Hover after a podcast ad half a decade ago, and it worked out so well that in that span I have steadily moved *everything* there, and I have never regretted it. I don’t actually have a lot to say beyond that: Hover is easy to use to register a new domain, they have great customer service (when you need it, which has only happened to me once and because of a problem on a *different* registrar), and they even have a *nice* website!

[^not-hover]: This is *strictly* not true: I have a single domain registered at another registrar. That was, in retrospect, a mistake… for a variety of reasons. I won’t be repeating it. :insert grimacing emoji here:

## DNS: Cloudflare

I switched all of my DNS name servers to [Cloudflare] earlier this year. I had a longstanding goal of having my registration, my name servers, and my actual hosting and deployment in separate places for a few years now. I don’t remember where I first ran into the idea of keeping those separate, but it stuck—forcefully, by dint of experience.

At one point I was managing all three—registration, name servers, and hosting—through an old-school shared hosting provider ([Stablehost], still a pretty solid option in that space!)… and migrating *out* of that provider was incredibly painful. (It’s actually not 100% done! The hard parts are all done now, though, which is a relief.)

After doing a bunch of research back in late June, I migrated all of my DNS to Cloudflare. *All* of it. This took [a fair bit of work][rewrites] but it has made everything else since then *much* easier. Their domain name management control panel is really good—as good as any I’ve used—and in my experience it’s also incredibly *fast* to propagate the information around the web. That latter bit is particularly pleasant and important, as anyone who has ever had to mess with DNS knows!

<aside>

If you’re curious: yes, I *do* have thoughts on Cloudflare’s approach to deciding who to leave on the internet and who to kick off the internet, but I’ll save those for another day.

</aside>

[Stablehost]: https://www.stablehost.com
[rewrites]: https://v4.chriskrycho.com/2019/my-final-round-of-url-rewrites-ever.html

## Summary

When you put all those pieces together, what you have is:

- Domain registered at Hover, configured with its nameservers to point to Cloudflare
- Cloudflare managing the DNS, pointing the URL `v5.chriskrycho.com` to the corresponding Netlify setup
- Netlify publishing the site as defined—both its content and its design—in a Git repository, which is hosted on GitHub
- Content all written in Markdown, with a variety of tools across macOS and iOS and iPadOS

In a planned future post (which I have not even started writing yet, so no promises as to when it will appear), I will try to dig into the details of *how* I build the site—the design, the HTML, the custom implementation details, the fonts, you name it. If you’re curious in the meantime, the implementation is [all publicly available][gh]—and you’re [welcome to crib from it][license]!

[license]: https://v5.chriskrycho.com/colophon/#copyright-and-license
