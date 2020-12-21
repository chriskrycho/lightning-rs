---
title: >
    This Week I Learned #1
feedId: journal/this-week-i-learned-1/
date: 2020-04-24T21:15:00-0600
updated: 2020-04-26T08:25:00-0600
summary: Summaries and links to a number of articles I read this week.
tags:
    - fuzz testing
    - software development
    - health care
    - politics
    - Covid-19
    - remote work
    - things I learned
    - historical theology
    - academia
    - This Week I Learned

---

Starting this week, I’m trying something new—taking a page out of [Reda Lemeden’s book][rl], I’m going to make a point to post a short summary at the end of the week of the things I *learned* this week. This will mostly just be a handful of articles I read that actually made an impression. (I read a *lot*. Not all of it is important or meaningful.)

- *’80s music*. [What Makes It Sound ’80s? | Journal of Popular Music Studies](https://jpms.ucpress.edu/content/31/3/73), HT Dan Cohen, [Humane Ingenuity 20: Physical Distancing, Social Cohesion](https://buttondown.email/dancohen/archive/fa0093f2-3a4e-4e35-8ae6-9d49ab130412): fascinating information about the development of synthesizer culture in the 1980s; along with some incisive arguments about the nature of timbre as a musical phenomenon, especially *tone color*; and some smart comments on “80’s music as a *genre*” with interesting death-of-the-composer/songwriter POV.

- *Sonic fidelity*. [The Lossless Self](https://thenewinquiry.com/the-lossless-self/), HT precious article—an argument that “fidelity” as a value for music listeners says as much about us as listeners, and our culture, as it does about the “ideal” nature of a recording or its playback, with an interesting allusive nod here and there to the mechanics of attention.

- *Fuzzing*. [John Regehr][regehr] walks through “fuzzing” an implementation of an abstract data type (specifically, a red-black-tree). Fuzzing is *generating* tests—not writing them by hand—to get higher confidence in the behavior of some piece of code. It’s an extremely useful technique which is very much underused in the industry. I’ve never had a chance to do it myself, but this article gave me a much better idea how I *would*.

- *Health care*. Scott Alexander (Slate Star Codex) [digs into][ssc] why the Amish pay *much* less for health care than do average Americans, and why they’re generally healthier, too. (They do die younger than average Americans, too, but in part because they tend to take death with a bit more grace than the average American, aiming to live a good life and then to die with dignity rather than to stretch it out to the last possible moment.) Lots of interesting food for thought on how we might change the American health care system here.

- *Indentation*. Debates over the goodness/badness of significant indentation recur whenever programming languages come up. (Ruby, Python: fight.) One of my favorite programming language blogs, [Programming Linguistics][pl], dug into how the author (Jon Goodwin) is tackling it in *his* programming language, Cone: [cut the Gordian knot!][cone]—by defaulting to significant whitespace, but supporting a curly-brace delimited mode.

- *Zoom*. Michael Sacasas [explores][sacasas] what we might call the techno-physical causes for “Zoom Fatigue”: that phenomenon so well known to long-term remote workers of being exhausted after a day of video conferences even beyond what a day of meetings would normally entail. I’ve been doing this for 7 years, so it’s old hat for me… and I still feel the challenge here; I’ve just adapted to it.[^remote]

- *Theological development*. Fred Sanders looks at Richard Muller’s <cite>Post-Reformation Reformed Dogmatics</cite> and [asks][sanders] why Muller spent so much time working to connect dots between the patristic, medieval, and Reformation eras in building out his discussion of *post*-Reformation dogmatics. The answer is that the modern academy has a sharp split between categories like “doctrine, philosophy, biblical interpretation, and devotion”—a division that was utterly foreign to the scholars and pastors and teachers working in the eras discussed. Everything is specialized today such that work that crosses those boundaries is considered “interdisciplinary” (and, too often, scorned by the rest of the academy).

[rl]: https://redalemeden.com/microblog/post-1587316560066
[regehr]: https://blog.regehr.org/archives/896
[ssc]: https://slatestarcodex.com/2020/04/20/the-amish-health-care-system/
[pl]: http://pling.jondgoodwin.com/
[cone]: http://pling.jondgoodwin.com/post/significant-indentation/
[sacasas]: https://theconvivialsociety.substack.com/p/a-theory-of-zoom-fatigue
[sanders]: http://scriptoriumdaily.com/why-its-hard-to-trace-trajectories-and-continuities-muller/

[^remote]: Remote is *absolutely* worth that tradeoff to me. I should write more on that at some future date. But it would be a lie to call it anything *but* a tradeoff.

*[HT]: hat tip—the idea of a tip-of-the-hat to someone who helped you in some way.