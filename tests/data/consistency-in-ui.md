---
Title: Consistency in User Interfaces
Subtitle: Or, one part of why Git has such a learning curve.
Date: 2016-07-15 10:37
Tags: git, software development
Category: tech
---

People sometimes ask what I mean when I say Git's UI is maddeningly inconsistent. Here's a concrete example: what are the commands to list tags, branches, and stashes?

- `git tag --list`
- `git branch --list`
- `git stash list`

Follow that up by noticing the difference in meaning for the `-v` flag between the commands:

- `git branch -v`: *verbose* mode: list the hash with an abbreviated commit summary
- `git tag -v`: *verify* a tag against its GPG signature
- `git stash list -v`: no-op, completely ignored

This is *disastrously* bad user interface design, and there is literally no reason for it except that the developers of Git, led by Linus Torvalds, don't care about designing for end users. They hack in whatever commands seem to make the most sense right here and right now, and call it good---and then imply or directly state that anyone who has a problem with it is stupid or lazy.

But users are neither stupid nor lazy, and it is not stupid or lazy to want a system to behave in a a consistent way. Imagine if the buttons on you car's media dashboard (a plastic one where the labels stay the same) did different things depending on whether you were in *Drive* or *Reverse*. Or if the light switches in your house behaved differently if you were using your toaster than if you were vacuuming, "on" and "off" labels notwithstanding.

Good user interface design is no less applicable to a command-line utility than to a pretty iOS app. Don't let Linus Torvalds or anyone else tell you otherwise.
