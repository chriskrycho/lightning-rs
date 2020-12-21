---
title: Follow-Up on Command-Line Finding and Filtering
subtitle: >
    A simpler solution that doesn’t require <code>tr</code>… if you have GNU utils or other alternatives.
date: 2020-05-04T09:15:00-0600
updated: 2020-07-26T12:05:00-0600
summary: >
    You can use a variant flag with GNU grep and ripgrep to filter with null characters.
tags:
    - things I learned
    - command line
    - software development
qualifiers:
    audience: >
        90% myself in the future, when I (inevitably) ask this question again—but also anyone else who hits this particular question about command-line invocations.
    epistemic: >
        Slightly higher than [the *previous* post on the subject](https://v5.chriskrycho.com/journal/find-grep-xargs-newlines-null/), courtesy of the requested reader feedback!
---

In my [previous post](https://v5.chriskrycho.com/journal/find-grep-xargs-newlines-null/), I used the `tr` utility to deal with needing to transform newlines into null characters. However, as I hoped when I put a request for a better way to do it in my <b>Epistemic Status</b> qualifier, a reader emailed me with a better solution!

If you’re using the GNU version of `grep`, it has a `--null-data` (shortened as `-z`) flag which makes grep treat its input as null-character-separated. You can combine that with the `-print0` flag to `find` to get the same results as I got with `tr` (presumably with better performance because it doesn’t require doing the replacement in another tool):

```sh
$ find notes -name ".md" -print0 |\
  grep --null-data "notes/2020" |\
  xargs -0 wc -w
```

This reminded me that [ripgrep] has the same feature, with the same `--null-data` flag. Similarly, [fd] has a `--print0` (`-0`) option. You can combine *these* and (if you like) [cw][cw][^cw] to get the same effect:

```sh
$ fd notes --print0 ".md" notes |\
  rg --null-data 'notes/2020' |\
  xargs -0 cw -w
```

Huzzah for versions of tools that understand these things and make this simpler than the solution I posted yesterday (and thanks to my reader for sending in that note)!



[^cw]: `cw` is nice because with especially large sets of data, the fact that you can invoke across threads becomes very handy. If I word-count *all* of my notes with it (currently 667 files and just shy of 150,000 words), using 4 threads instead of 1 (the default, and all you get with `wc`) takes about 6–8 milliseconds off the run time. Not important at *this* scale… but if you’re dealing with *very* large amounts of data, it might be.

[ripgrep]: https://github.com/BurntSushi/ripgrep
[fd]: https://github.com/sharkdp/fd
[cw]: https://github.com/Freaky/cw
