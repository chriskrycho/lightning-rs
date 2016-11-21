---
Title: The Next Generation of Version Control
Summary: >
    The current state of affairs with version control is a mess. Things we can
    get right next time around.
Slug: next-gen-vcs
Author: Chris Krycho
Date: 2014-10-16 21:45
Modified: 2014-10-20 07:25
Tags: software development
...

The current state of affairs in version control systems is a mess. To be sure,
software development is *far* better with *any* of the distributed version
control systems in play---the three big ones being [Git][git], [Mercurial][hg]
(`hg`), and [Bazaar][bzr] (`bzr`), with a few other names like [Fossil][fossil]
floating around the periphery---than it ever was in a centralized version
control system. There are definitely a few downsides for people converting over
from some standard centralized version control systems, notably the increased
number of steps in play to accomplish the same tasks.[^dad] But on the whole,
the advantages of being able to commit locally, have multiple complete copies of
the repository, and share work without touching a centralized server far
outweigh any downsides compared to the old centralized system.

[^dad]: A point that was highlighted for me in a conversation a few months ago
    with my father, a programmer who has been using SVN for a *long* time and
    found the transition to Git distinctly less than wonderful.

That being so, my opening statement remains true, I think: *The current state of
affairs in version control is a mess.* Here is what I mean: of those three major
players (Git, Hg, and Bazaar), each has significant downsides relative to the
others. Git is famously complex (even arcane), with a user interface design
philosphy closely matching the UI sensibilities of Linus Torvalds---which is to
say, all the wires are exposed, and it is about as user-hostile as it could
be.[^hostile] It often outperforms Hg or Bazaar, but it has quirks, to say the
very least. Hg and Bazaar both have *much* better designed user interfaces. They
also have saner defaults (especially before the arrival of Git 2.0), and they
have better branching models and approaches to history.[^history] They have
substantially better documentation---perhaps especially so with Bazaar, but with
either one a user can understand how to use the tool *without having to
understand the mechanics of the tool*. This is simply not the case with Git, and
while I *enjoy* knowing the mechanics of Git because I find them interesting,
*having* to understand the mechanics of a tool to be able to use it is a
problem.

[^hostile]: Anyone who feels like arguing with me on this point should go spend
    five minutes laughing at the [fake man pages][man] instead.

[^history]: Few things are as hotly debated as the relative merits of the
    different systems' branching models and approaches to history. At the least,
    I can say that Hg and Bazaar's branching models are *more to my taste*.

But the other systems have their downsides relative, to Git, too. (I will focus
on Hg because I have never used Bazaar beyond playing with it, though I have
read a good bit of the documentation.) Mutable history in Git is valuable and
useful at times; I have rewritten whole sequences of commits when I realized I
committed the wrong things but hadn't yet pushed.[^mutable] Being able to commit
chunks instead of having to commit whole files at a go is good; I feel the lack
of this every time I use Hg.[^patch] (Needing to understand the *file system*
that Git invented to make sure you do not inadvertently destroy your repository
is... not so good.) A staging area is nice,[^queues] (even if *having* to stage
everything manually can be in the pain in the neck[^dash-a]).

[^mutable]: Yes, there are extensions that let you do this with Hg, but they are
    fragile at best in my experience, and substantially less capable than Git's.

[^patch]: Yes, I know about Hg's record extension. No, it is *not* quite the
    same, especially because given the way it is implemented major GUI tools
    cannot support it without major chicanery.

[^queues]: Yes, I know about Hg's queue extension, too. There is a reason it is
    not turned on by default, and using it is substantially more arcane than
    Git's staging are. Think about that for a minute.

[^dash-a]: Yes, there is the `-a` flag. No, I do not want to have to remember it
    for every commit.

In short, then, there was no clear winner for this generation. Each of the tools
has significant upsides and downsides relative to the others. Git has become the
_de facto_ standard, but *not* because of its own superiority over the
alternatives. Rather, it won because of other forces in the community. Mostly I
mean [GitHub][github], which is a *fantastic* piece of software and easily the
most significant driving factor in the wider adoption of Git as a tool. The
competition ([Bitbucket][bitbucket] and [Launchpad][launchpad]) are nowhere near
the same level of sophistication or elegance, and they certainly have not
managed to foster the sorts of community that GitHub has. The result has been
wide adoption of Git, and a degree of Stockholm Syndrome among developers who
have adopted it and concluded that the way Git works is the way a distributed
version control system *should* work.

It is not. Git is complicated to use and in need of tools for managing its
complexity; the same is true of Hg and Bazaar, though perhaps to a slightly
lesser extent because of their saner branching models. This is what has given
rise to the [plethora][git flow] of [different][github flow] formal
[workflows][gitlab flow] representing various attempts to manage that complexity
(which have been [applied][hg flow] to other systems [as well][hg flow intro]).
Managing branching, linking that workflow to issues, and supplying associated
documentation for projects have also cropped up as closely associated tasks---
thus the popularity of GitHub issues and Bitbucket wikis, not to mention
[Fossil's][fossil] integration of both into the DVCS tool itself. None of the
tools handle differences between file systems very elegantly (and indeed, it
took *years* for Git even to be useable on Windows). All of them especially
struggle to manage symlinks and executable flags.

So there is an enormous opportunity for the *next* generation of tools. Git, Hg,
and so on are huge steps forward for developers from CVS, Visual SourceSafe, or
SVN. But they still have major weaknesses, and there are many things that not
only can but should be better. In brief, I would love for the next-generation
version control system to be:

  - distributed (this is now a non-negotiable);
  - fast;
  - well-documented---*at least* as well as Hg is, and preferably as well as
    Bazaar is;
  - well-designed, which is to say having a user interface that is actually a
    user-interface (like Hg's) and not an extremely leaky abstraction around the
    mechanics;[^leaky]
  - fast;
  - file-system oriented, *not* diff-oriented: this is one of Git's great
    strengths and the reason for a lot of its performance advantages;
  - extensible, with a good public API so that it is straightforward to add
    functionality like wikis, documentation, social interaction,  and issue
    tracking in a way that actually integrates the tool;[^integrates]
  - and last but not least, truly cross-platform.

[^leaky]: Let's be honest: if Git's abstraction were a boat, it would sink. It's
    just that leaky.

[^integrates]: GitHub does all of this quite well... but they have had to write
    heaps and gobs of software *around* Git to make it work.

That is a non-trivial task, but the first DVCS that manages to hit even a
sizeable majority of these desires will gain a lot of traction in a hurry. The
second generation of distributed version control has been good for us. The third
could be magical.

[git]: http://git-scm.com
[hg]: http://mercurial.selenic.com
[bzr]: http://bazaar.canonical.com/en/
[fossil]: http://www.fossil-scm.org
[man]: http://git-man-page-generator.lokaltog.net
[github]: https://github.com
[bitbucket]: https://bitbucket.org
[launchpad]: https://launchpad.net
[git flow]: http://nvie.com/posts/a-successful-git-branching-model/
[github flow]: http://scottchacon.com/2011/08/31/github-flow.html
[gitlab flow]: https://about.gitlab.com/2014/09/29/gitlab-flow/
[hg flow]: https://bitbucket.org/yujiewu/hgflow/wiki/Home
[hg flow intro]: https://andy.mehalick.com/2011/12/24/an-introduction-to-hgflow
