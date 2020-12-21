---
title: find, grep, xargs, and newlines and null
subtitle: >
    Turns out `tr` is your friend for this kind of thing.
date: 2020-05-03T13:00:00-0600
updated: 2020-05-03T13:14:30-0600
summary: >
    If you want to find files, filter them on file name, and pipe the result into some other Unix command, use tr to substitute the null character for newlines.
tags:
    - things I learned
    - command line
    - software development
qualifiers:
    audience: >
        90% myself in the future, when I (inevitably) ask this question again—but also anyone else who hits this particular question about command-line invocations.
    epistemic: >
        Well, I know this works, but I wouldn’t be surprised if someone told me an even better way to implement it. If that’s you, email me?

---

:::note

See [the follow-up](/journal/follow-up-on-command-line-finding-and-filtering/) post, in which I show an easier and faster way of doing this… as long as you have the GNU versions of the utilities, or alternatives like [ripgrep].

:::

[ripgrep]: https://github.com/BurntSushi/ripgrep

<b>Summary:</b> If you want to find files, filter them on file name, and pipe the result into some other Unix command (e.g. `wc` to count words), use `tr` to substitute the null character for newlines:

```sh
$ find notes -name ".md" |\
  grep "notes/2020" |\
  tr '\n' '\0' |\
  xargs -0 wc -w
```

---

I sometimes want to use find subset of files matching a pattern, further filter it with grep, and then do something with the results using `xargs`—most often, something like counting the words in the subset I found. The basic flow I want is:

```sh
$ <find files> | <filter files> | <count the words in each>
```

My first instinct (and possibly yours if you’re reading this via a web search!) is to use `find`[^fd] to get the first set of files, do the further filtering with `grep`,[^rg] and finally use `xargs` to pipe the results into `wc -w`.[^cw]

The first problem is that `wc`, like most Unix commands, assumes that its arguments are space-delimited. If you hand any standard Unix utility a bunch of files where any of them have spaces in their names, you’ll see reports that various files don’t exist—where the “file” named is just one part of an actual file name.

Normally, I solve this kind of thing using `xargs -0`, which uses the null `\0` character as the separator for arguments to the function you invoke. If I were *just* using `find`, I would use its `-print0` flag. If I were *just* using `grep`, I would use its `--null` flag.[^rg-1] Unfortunately, here I’m combining them. Since I’m not working with the output from `find` directly, its `-print0` expression isn’t useful: those results will be piped into `grep`, which prints each result on a line. Meanwhile, `grep --null` separates *file results* with `\0`, but does not separate results from within a stream of text—which is what `grep` sees when we pipe the results of `find` into it. No matter what I did, I kept seeing the error:

> open: File name too long

The problem was that there were no `\0` characters in the stream going into `wc`, but I was invoking it as `xargs -0 wc -c`, so it was trying to treat the list of *all* the matching files as a *single argument*… which, at over 12,000 characters long, far exceeded the operating system’s limits for file paths (on *any* file system in common use today).

After thinking about this for a few, I realized I needed to treat `grep` output as a plain text stream, rather than a list of files. Then the question was how to substitute the null character `\0` for each of the newlines `\n` in that stream. My first thought was to use `sed`, but `sed` works on *lines*, using `\n` as its separator, so you have to do shenanigans to get it to work. Much easier is to use `tr`, a utility I had never heard of before today, which is used to <i>translate characters</i>. (Credit to [this Stack Overflow question](https://stackoverflow.com/questions/1251999/how-can-i-replace-a-newline-n-using-sed) for teaching me *both* of these things!) The `tr` man page’s description:

> The <b>tr</b> utility copies the standard input to the standard output with substitution or deletion of selected characters.

While it can do substantially more sophisticated transformations than this, too, it’s perfect for this simple text replacement: `tr '\n' '\0'` substitutes the null character `\0` for every newline in the input stream—and *then* `xargs -0` will do what we need. The final workflow looks like this (separated onto multiple lines so it’s easier to follow):[^my-way]

```sh
$ find -name ".md" notes |\  # find the files
  grep "notes/2020" |\       # filter them
  tr '\n' '\0' |\            # replace newline with null
  xargs -0 wc -w             # word count each file!
```

In sum: today I learned that—

- `grep` treats *streams* differently than *files*
- `sed` doesn’t (easily) work with newlines `\n`
- `tr` *exists* and is great for simple character substitution throughout a stream of text



[^fd]: Or the lovely and very fast [Rust-powered alternative, `fd`](https://github.com/sharkdp/fd)—but the exact same set of challenges in the rest of this post apply whichever you’re using.

[^rg]: Here I’m usually using [ripgrep], which is *also* powered by Rust and is *ridiculous* fast, but again: the same constraints apply.

[^cw]: And here I’m usually using [cw](https://github.com/Freaky/cw), yet another very fast Rust implementation of a utility.

[^rg-1]: The same goes for `rg --null` and its `rg -0` shorthand.

[^my-way]: In fact, this looked a *little* different, because I was using all the Rust-powered substitutes:

    ```sh
    $ fd ".md" notes |\   # find the files
      rg "notes/2020" |\  # filter them
      tr '\n' '\0' |\     # replace newline with null
      xargs -0 cw -w      # word count each file!
    ```
