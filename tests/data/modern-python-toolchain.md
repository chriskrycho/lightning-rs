---
Title: A Modern Python Development Toolchain
Date: 2015-05-16 22:40
Tags: Software Development
Summary: >
    Using homebrew, pyenv, and pip to manage Python development environments
    and workspaces.
---

Most of my development time these days---and especially the majority of my
happiest time!---is spent working in Python. As such, I've experimented off and
on over the last few years with the best workflow, and have settled down with a
set of tools that is *very* effective and efficient for me. I'm sure I'm not the
only one who's had to wrestle with some of the issues particular to this
toolchain, and I know that information like this can be valuable especially for
people just starting off, so I thought I would document it all in one
place.[^caveats]

Note: when talking about a given program, I will italicize it, like *brew* or
*git* or *python*. When talking about things to type, I will make them a code
block like `git clone <a repository>`. For any extended samples, I will make
them full-on code blocks:

```python
import re

def a_neat_function():
    my_string = "Isn't it cool?"
    if re.match(r"i\w+", my_string, flags=re.I):
        print(my_string)
```

---

The main tools I use are: a good text editor (I like all of [Sublime Text],
[Atom], [TextMate], and [Chocolat]; each has its own strengths and weaknesses)
or sometimes [a full IDE], version control software (I appreciate and use both
[Git] and [Mercurial]), and three dedicated tools to which the rest of this post
is devoted: *pyenv*, *pip*, and virtual environments.

Everyone is going to have their own preferences for version control tools and an
editor; but the recommendations I make regarding Python installations, package
management, and workspaces/virtual environments should be fairly standard for
anyone doing Python development on a Unix-like system in 2015.

[Sublime Text]: //www.sublimetext.com
[Atom]: //atom.io
[TextMate]: //github.com/textmate/textmate
[Chocolat]: //chocolatapp.com
[a full IDE]: https://www.jetbrains.com/pycharm/
[Git]: http://www.git-scm.com
[Mercurial]: http://mercurial.selenic.com

Python Proper
-------------
First up: Python itself. OS X ships with a built-in copy of Python 2; in the
latest version of Yosemite, it's running Python 2.7.6. The latest version of
Python 2 is 2.7.9, so that isn't *terribly* far behind---but it is still behind.
Moreover, OS X does *not* ship with Python 3, and since I do all of my
development in Python 3[^py3] I need to install it.

### Homebrew
For a long time, I managed all my Python installations with
[*homebrew*][homebrew]. If you're not familiar with it, *homebrew* is a package
manager that lets you installed tools on the command line, similar to what you
get from *aptitude* or *yum* on Ubuntu or Fedora respectively.[^pkg] If you're
not using *homebrew* yet, I highly recommend it for installing command-line
tools. (If you're not using command-line tools yet, then the rest of this post
will either bore you to death, or prove extremely enlightening!) If you haven't
started yet, now's a good time: [go install it!][homebrew].

While *homebrew* is great for installing and managing packages in general, I
can't say this loud enough: *don't manage Python with homebrew*. It's finicky,
and really isn't meant for all the things you have to do to manage more than one
version of Python at a time.[^finicky] (There's a reason there's a whole
[troubleshooting section] devoted to it.) If you think it's crazy that I might
want more than one copy of Python installed a time, well... let's just say I
suspect you'll change your mind after doing a bit more development. (At the most
basic, most people will end up wanting both Python 2 and 3 installed, and will
want to upgrade them as bug fixes and the like come out.)

[homebrew]: http://brew.sh
[troubleshooting section]: https://github.com/Homebrew/homebrew/blob/master/share/doc/homebrew/Homebrew-and-Python.md

### pyenv
Instead of installing via *homebrew*, use it to install [*pyenv*], and use that
to manage your installations. *pyenv* is a dedicated tool for managing your
"Python environment," and it excels at that. If you were on a Mac with
*homebrew* installed, your setup process to add the latest version of Python
might look something like this:

[*pyenv*]: https://github.com/yyuu/pyenv

```shell
$ brew install pyenv
$ echo 'eval "$(pyenv init -)"' >> ~.profile
$ source ~/.profile
$ pyenv install 3.4.3
```

Line by line, that (a) installs *pyenv*, (b) adds a hook to your shell
profile,[^profile] \(c) updates your current session using the updated profile,
and (d) installs the latest version of Python (as of the time I'm writing this).
Now you have a full version of Python 3.4.3 alongside the system install of
Python 2.7.6. If you wanted to install 2.7.9, or 2.2.3, or the development
version of PyPy3, you could easily do that as well.

In addition, *pyenv* lets you specify which version to use globally
(`pyenv global <name>`) and which version to use in a given directory structure
(`pyenv local <name>`). So if you prefer to use Python 3 in general, but need to
use Python 2 on one project, you can just navigate to the root of that project
and set it:

```shell
$ pyenv global 3.4.3
$ cd path/to/my/project
$ pyenv local 2.7.9
```

This will create a simple plain text file, `.python-version`, whose contents
will be just `2.7.9`---but for everything under `path/to/my/project`, typing
`python` will launch Python 2.7.9, while typing it *outside* that folder will
launch Python 3.4.3. (If you want, you can just create the `.python-version`
file yourself manually and give it the name of a version. There's nothing
special about it all; it's just the place `pyenv` looks to know which Python
version to use.)

Managing Python Packages
------------------------
There are four basic approaches to managing Python packages:

  - installing them manually
  - using a system-level package manager like *homebrew*, *yum*, or *aptitude*
  - using *easy_install*
  - using *pip*

The vast majority of the time, the right choice is using *pip*. Over the last
few years, *pip* has become the default install tool for Python packages and it
now ships natively with it on every platform. Suffice it to say: if you need to
install a package, do not install it not with *homebrew* (or *aptitude* or
*yum*). Install it with *pip*. It integrates with Python better, it always has
access both to the latest versions of Python packages (including those only
available in e.g. development repositories on GitHub or Bitbucket or wherever
else) and to all previously released versions, and it's the community's main
tool for the job.

That said, occasionally it makes sense to install packages manually by
downloading them and running `python setup.py install` or to use a system-level
package manager. On the other hand, given *pip*'s ability to do everything
*easy_install* does, and its ability to do quite a few more things as well,
there really isn't a time to use *easy_install*. Using the language-supplied
tools keeps everything playing nicely together. Perhaps just as importantly, it
is the only way to make sure everything behaves the way it should when you start
using...

Virtual Environments
--------------------
When working with a variety of different clients, or simply on different
projects, it is common not only to end up with different versions of Python but
also with different sets of packages or---tricker still!---different versions of
the same package required for different projects. Virtual environments
provide a solution: they reuse the main Python executable (by creating links on
the file system to it), but create isolated "workspaces" for the various
packages you might install.

That way, in one workspace, you might have version 1.2 of a package installed,
and in another you might have version 3.3 installed---because those are the
required dependencies for something *else* you're doing. This isn't a
hypothetical situation. For quite a while with one of my clients, we had pinned
a particular version of the Python documentation package we use because it broke
our use case after an update---but I still wanted to have the latest version of
that tool in my *other* projects. Setting up virtual environments neatly solves
that problem.

### venv and virtualenv
If you have Python 3.3 or later, you have a built-in tool for this called
[*pyvenv*]; if you have Python 3.4 or later, it supports *pip* right out of the
gate so you don't have to install it yourself. If you're on older versions, you
can install [*virtualenv*] \(`pip install virtualenv`) and get the same basic
tooling: *pyvenv* was inspired by *virtualenv*. Then you can create virtual
environments with the `pyvenv` or `virtualenv` commands, and use those to
isolate different setups from each other. If you haven't started using virtual
environments yet, start now!

[*pyvenv*]: https://docs.python.org/3/library/venv.html
[*virtualenv*]: https://virtualenv.pypa.io/en/latest/

### pyenv with virtualenv
I know, the similarity of names for *pyenv* and *pyvenv* is unfortunate. If it
helps, you can call the latter as `venv` rather than `pyvenv`. But, more
importantly, one of the areas *pyenv* is much better than *homebrew* is its
support for managing virtual environments. Install [*pyenv-virtualenv*]:

[*pyenv-virtualenv*]: https://github.com/yyuu/pyenv-virtualenv

```shell
$ brew install pyenv-virtualenv
$ echo 'eval "$(pyenv virtualenv-init -)"' >> ~/.profile
```

Now you're off to the races: you'll never have to type
`pyvenv <path to a virtual environment>`, because instead you can just type
`pyenv virtualenv <version> <name>` and *pyenv* will take care of setting it up
for you. Even better: all the nice tricks I listed above about setting
directory-specific and global preferences for which Python version to use work
equally well with virtual environments managed via *pyenv*. In other words, you
can do something like this:

```shell
$ pyenv install 2.7.9
$ pyenv install 3.4.3
$ pyenv global 3.4.3
$ pyenv virtualenv 2.7.9 my-virtual-environment
$ cd path/to/my/project
$ pyenv local my-virtual-environment
```

The `.python-version` file will contain `my-virtual-environment`. The Python
version will be 2.7.9. The environment will be isolated, just as if you had run
`pyvenv` to set up a virtual environment. Everything works together beautifully!
Moreover, you can easily reuse virtual environments this way, because you can
set the `local` value in more than one place. For example, I use the same
virtual environment for this site and [Winning Slowly], because they have
slightly different site configurations but all the same Python dependencies.
Creating it was simple:

```shell
$ pyenv install 3.4.3
$ pyenv virtualenv 3.4.3 pelican
$ cd ~/Sites/chriskrycho.com
$ pyenv local pelican
$ cd ~/Sites/winningslowly.org
$ pyenv local pelican
```

[Winning Slowly]: //www.winningslowly.org/
    "A podcast: taking the long view on technology, religion, ethics, and art."

I named the virtual environment after [the tool I use to generate the
sites][pelican], and reused it in both sites. Both now have a `.python-version`
file that reads `pelican`. Now, anytime I'm working anywhere under
`~/Sites/chriskrycho.com` *or* `~/Sites/winningslowly.org`, I have the
same tooling in place.

[pelican]: //docs.getpelican.com/

Summary
-------
The combination of *pip*, *pyenv* and virtual environments makes for a very
simple, straightforward process to manage Python environments these days:

  - Install Python versions with *pyenv*.
  - Install Python packages with *pip*.
  - Set up virtual environments with *pyenv-virtualenv*.

If you stick to those basic rules, Python itself shouldn't give you any trouble
at all.



[^caveats]: All the usual caveats apply, of course: this may or may not work
    well for you; it's just what works for me, and I make no claim or warranty
    on the tools below---they're working well for *me*, but I don't maintain
    them, so if they break, please tell the people who maintain them! Also,
    because I do nearly all my development on a Mac (I test on Windows, but
    that's it), the following is necessarily *fairly* specific to OS X. You can
    readily adapt most of it to Linux, though, or even to a [Cygwin] install on
    Windows---I do just that when I have cause. But my main tool is a Mac, so
    that's what I've specialized for.

[Cygwin]: https://www.cygwin.com

[^py3]: Lucky me, I know!

[^pkg]: Yes, I know that those are wrappers around Debian and Arch, and I know
    about *apt-get* and *rpm*. No, that information isn't especially relevant
    for the rest of this post.

[^finicky]: For example, if you upgrade your Python installation using homebrew
    and then cleanup the old version (e.g., by running the typical
    `brew update && brew upgrade && brew cleanup` sequence)---say, from 3.4.2 to
    3.4.3---and you have virtual environments which depended on 3.4.2... well,
    you're in a bad spot now. A *very* bad spot. Have fun getting back to a
    working state!

[^profile]: You can of course drop it directly in `.zshrc` or `.bash_profile` or
    wherever else. [My setup] puts all common handling in `.profile` and runs
    `source .profile` as the first action in any other shell configurations.

[My setup]: //github.com/chriskrycho/profile
