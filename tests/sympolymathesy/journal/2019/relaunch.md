---
title: Relaunch!
subtitle: >
    A new website design and implementation for 2020 and beyond—with a new title to boot!
date: 2019-11-17T15:30:00-0700
updated: 2020-08-15T20:38:00-0700
tags:
    - writing
    - blogging
    - web design
    - open-source software
    - TypeScript
    - site meta
qualifiers:
    audience: people who care about things like new website designs.
summary: >
    Sympolymathesy is the fifth version of chriskrycho.com—and I’m so happy to have it in the world at last!

---

Welcome to <b>Sympolymathesy</b>—the fifth version of this website! I’m happy to have it in the world at last.

Long-time readers will no doubt notice that it newly has a *title*, as well as a fresh look—albeit one closely connected in many ways to the previous design. Curious nerds will also notice that the tech making this thing go has changed. What follows are some comments on each of the above!

1. [New Site Title](#1-new-site-title)
2. [New Site Design](#2-new-site-design)
3. [New Site Tech](#3-new-site-tech)
4. [Onward!](#4-onward)

## 1. New Site Title

“Sympolymathesy”—<span class="greek">συμπολυμαθησι</span>—is a word I made up. It means “a thing which learns (_mathesi_) many things (_poly_) together (_sym_).” As a Greek-derived neologism extending [someone *else’s* Greek-derived neologism][symmathesy], I think it is both the nerdiest title imaginable for my site and therefore also the *best* title imaginable for my site. It is as close I can get as a statement of both the intent of my blogging and what it has been to me for the past fourteen years.[^14] I am learning many things here—in public, with you.

There are a few layers to this choice of name.

1. As I first noted [last year][z1], my blogging has long served as something of a public [Zettelkasten][z2]. _Sympolymathesy_ is a very good descriptor for a Zettelkasten: a system of notes that grows organically and helps you derive new connections between disparate ideas over time. _Poly_ gets at the variety of ideas and subjects in play, and _sym_ speaks to the connections between those ideas and subject. For that reason, the tag pages on this site will very soon now include a link to that same tag or category on previous versions of the site. This is a relaunch, but it is also a continuation of earlier work.

2. It matters enormously to me that I am learning in public. When you read and then respond by [sending me an email][email] or triggering a [Webmention][Webmention][^webmention], you help this space be what it is meant to be. Adding your voice to mine—whether in agreement or in disagreement—turns my offerings on this site into a conversation, and therefore something bigger than itself. It becomes a place of learning together, so that *we* become a sympolymathesy.

3. Finally, a clarification: don't get any weird ideas about the fact that the word "polymath" is embedded in the middle of this bigger word. I am not in the least claiming *that* word's connotations of brilliance—quite the contrary, in fact! This site is for me a way of learning about many things. In no way does it represent the state of being one *already* learned about many things.

### On adding a title at all

For a long time, I did not actually plan to *have* a site title other than “Chris Krycho.” That’s how the previous version of the site was titled, and it worked just fine. However, a few things pushed me to go ahead and add a title to this version.

First, I just kind of *like* sites which have titles. There is—or at least can be—a delightful bit of whimsy to it, and it communicates something interesting about the author. When you come to a website whose title is nothing other than a person’s name, you learn that… you are at the website of a particular person. When you come to a website titled [Irrational Exuberance], or [one][reda] with sections titled “Unredacted” and “Wide Gamut”—well, then you *do* learn something about the author.

Second, a long time ago, on [a previous version of this site][v3], I split the various interests which make up the site into sub-sites with their own titles. The main page was <i>Chris Krycho</i>, but below that lived <i>Designgineering</i>, <i>Ardent Fidelity</i>, <i>Ars Artis</i>, and <i>From the Hearth</i>. I liked the character that added to the site (and I quite enjoyed the art direction I added to give each of those sections its own character), but I ultimately found that there was too much overlap. I could not separate my thoughts on technology from those on art, nor those on art from those on my faith, nor those on my faith from its applications to my work in technology. Accordingly, I pulled those back together when I relaunched the site in 2014. But I miss the interesting titles! So here we are, with an interesting title for the *whole* site!

[v3]: https://2012-2013.chriskrycho.com
[Irrational Exuberance]: https://lethain.com/about/
[reda]: https://redalemeden.com
[symmathesy]: https://norabateson.wordpress.com/2015/11/03/symmathesy-a-word-in-progress
[email]: mailto:hello@chriskrycho.com
[Webmention]: https://webmention.net
[z1]: https://v4.chriskrycho.com/2018/blog-as-note-taking-tool.html
[z2]: https://v4.chriskrycho.com/2019/what-is-a-zettelkasten.html

[^webmention]: You know, once I have those set up: soon! It's a fairly high priority for me—but *lower* priority than just getting this out the door!

[^14]: Fourteen! Years! I find it rather astounding that I have been at this so long—longer than any other endeavor in my life.

## 2. New Site Design

The previous design of my site lasted me for a solid five years, and I quite liked it. However, I increasingly ran into issues with it—issues that I have run into with *every* design of my website so far. I have many different interests (as suggested by my site title!) and on every previous iteration of the site I attempted to segment my content by subject. But while *some* pieces clearly belonged in one category and not another—a post about some TypeScript code was definitely in *Tech* and not *Theology*—a great many of my posts were *not* so easily bracketed. Under which category should I file an essay on a Christian ethic of tech?

The primary mandate for this redesign, then, was to accommodate that variety. I am now sectioning the site by *medium*, instead of by *subject*:

- General blog posts (of whatever length) go in [Journal](/journal)
- Essays—actual essays!—go in [Essays](/essays)
- Book reviews, quotes, and the like go in [Library](/library)
- Information about my podcasting lives at [Podcasts](/podcasts)
- Photos go under [Photography](/photography)
- Ongoing projects, series, etc. will be displayed under [Projects](/projects)
- I have a dedicated page for speaking, being on other podcasts, etc.: [Elsewhere](/elsewhere)

As an orthogonal layer of taxonomy over this, I have *tags*. This means I can still provide ways of sorting through my various subject matter—but overlap is built in. This is an idea I have come to organically by way of my experience with the previous versions of this site, had strongly reinforced by learning about Zettelkasten and reflecting on the design of [rewrite](https://rewrite.software). Even when you *do* have folder-like structures in your projects, you need a way to connect ideas across those. Tags, keywords, topics—call them what you will, you need this tool for structuring the content.

I also have decided that those topical indices will sort the opposite direction from most blog archives (unlike the top-level sections). I have noticed a few times over the last few years tha it is quite helpful to be able to follow the development of a person's thoughts. The normal latest-first archive order of a blog makes sense in many ways—the most recent of a person's thoughts are likely those you are most curious about when first finding your way to their site. Not so much, though, when you want to follow the course their thoughts have taken over the years!

There is more to say here, on the way many of our technologies tend to reinscribe (quite forcefully!) our bent toward obsessing with the present, and on the unique affordances of blogs as compared to other media both to help and to hurt with this, but I will leave those thoughts for another day. The point here is simply that I am intentionally structuring this site as a counter to those trends.[^v4-tweak]

Once I had started down the road of designing this new information hierarchy, I took the opportunity to rethink the basic navigation and layout of the site. While I liked a lot about the typography of the previous design, it was showing its age. Although I had made some tweaks along the way, I had been working with the same underlying structure and layout since [v3], back in 2012! I still quite like that design—but I like the new look *much* better. It takes many of the same basic elements of typography (including the two main typefaces from [v4]), and tightens them up into something fresh and modern.

[v4]: https://v4.chriskrycho.com

[^v4-tweak]: I also went back and tweaked to do the same on v4 as one final act of curation and maintenance for the future! 

## 3. New Site Tech

In the midst of the refresh, I also switched up the ways I'm building and deploying the site.

The previous version of the site was built on [Pelican]. This version is built on [Eleventy]. Long-time readers will recall that once upon a time, ages ago (that is: in 2016) I set out to build [my own static site generator][lx]. My aspirations for that project, much deferred, ended up substantially delaying my ultimate work to relaunch this site: because my goal all along was to be able to relaunch using my own tool. That dream is not dead. But it is deferred. I have a lot of thoughts about what a tool in this space can and should look like. Eleventy gets a lot right! It also has a few (significant-to-me) frustrations.

I'll cover those more in the future. At this point I'm *satisfied* with Eleventy, and I can make it do the things I need it to. That's more than I've been able to say for Pelican for the last few years! Note that this isn't a criticism of Pelican: it's a perfectly solid tool. The problems that made me switch mostly have to do with *me*! First, it doesn't *easily* fit certain things I want to do with it and it *requires* certain templates (even if they’re empty) that I just don’t use. Second, I'm no longer writing Python on a regular basis, so hacking on it to make improvements or to support some of the custom functionality I have come to lean on has very little appeal to me.

Third, those customizations themselves are things that blocked me from using a nice modern build strategy via something like [Netlify]. I definitely could have wrangled a *normal* Pelican setup into building on Netlify, not just deploying there. Unfortunately, [my setup] was not exactly normal: to support my use of citations and other [pandoc]-powered Markdown extensions, the build process required having both pandoc and [pandoc-citeproc] installed on the builder and—worst of all—some custom forks of various Pelican plugins to make everything play nicely with pandoc.

Eleventy is fast, its [default Markdown engine][markdown-it] is *much* easier to extend, and… I can use TypeScript with it! As part of this work, I have written TypeScript type definitions for nearly the entire public API surface of the project. I expect to open source those late this year, after I polish them like crazy. In the meantime, if you’re an Eleventy user who wants some types, feel free to just steal them from [the site repository][gh]. I have also built some other nice little bits of library code to integrate with Eleventy, which I'll be blogging about more over the coming weeks as I am able to open source them.

*[API]: application programming interface

One other thing I’ve long wanted to do: make it easy for people to submit corrections for typos and other mistakes. However, the aforementioned build issues always meant that even if someone *was* a well-educated nerd and found their way to the repository where the previous version of the site was hosted, and did the work of making a pull request, I’d still have to pull that down and rebuild it manually. Getting myself onto a builder that works with Netlify (or anything like it) eliminates that issue entirely: if someone submits a correction, all I have to do is merge it and the site will get rebuilt and redeployed automatically.

Even better, though: Netlify has a new feature called [Open Authoring]—currently in beta—that lets you combine their [Identity service] with [Netlify CMS][netlify-cms] and its [editorial workflow] to let *anyone* suggest changes.[^HT] This plus Netlify's awesome [branch preview] feature means those corrections can even come with previews of the effect on the site. [^netlify]

*[CMS]: content management system

As a result, each post will soon actually include a direct link to its source on GitHub, along with a "Suggest an edit" link that will allow people to send in corrections!

[Pelican]: https://github.com/getpelican/pelican
[Eleventy]: https://www.11ty.io
[lx]: https://www.github.com/chriskrycho/lightning-rs
[gh]: https://github.com/chriskrycho/v5.chriskrycho.com
[ghp]: https://pages.github.com
[Netlify]: https://www.netlify.com
[my setup]: https://github.com/chriskrycho/v4.chriskrycho.com/blob/master/pelicanconf.py
[pandoc]: https://pandoc.org
[pandoc-citeproc]: https://www.github.com/jgm/pandoc-citeproc
[markdown-it]: https://markdown-it.github.io
[Identity service]: https://docs.netlify.com/visitor-access/identity/
[netlify-cms]: https://www.netlifycms.org
[editorial workflow]: https://www.netlifycms.org/docs/configuration-options/#publish-mode
[Open Authoring]: https://www.netlifycms.org/docs/open-authoring/
[branch preview]: https://docs.netlify.com/site-deploys/overview/#branches-and-deploys

[^HT]: Hat tip to Chris Coyier on this—I learned it from [this CSS Tricks article][css-tricks]!

[css-tricks]: https://css-tricks.com/netlify-cms-open-authoring/

[^netlify]: Not a paid promotion, I promise. 😂 Netlify is just doing *really* great work and I can't help but enthuse about it. 

## 4. Onward

I’ve been working on this redesign and reimplementation for over a year. It’s bounced through multiple iterations on both implementation and design. And that’s after thinking about it and sketching ideas for years before that! If you're technically-minded, you can actually see the whole history in [the project repository on GitHub][gh]—and for *most* of the history of the project I have been quite conscientious about my commit history, so you may actually find it informative!

I’m glad to have it out here, and I’m hopeful that it’ll be a satisfying online home for the *next* half decade. As my friend and longtime collaborator [Stephen] put it as we talked about this design recently: this fresh start opens spaces wide enough to fit all of my work.

So now: time to get down to the business of actually filling up this space with words and photography and more!

[Stephen]: https://stephencarradini.com/
