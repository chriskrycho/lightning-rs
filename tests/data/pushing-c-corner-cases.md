---
Title: Pushing Into C's Corner Cases
Author: Chris Krycho
Date: 2014-08-12 09:00
Tags: software development
...

I'm working on a project that is all in C because of its long history and
legacy. We're slowly modernizing the codebase and writing all our new code in
Python (using NumPy, C extensions, and so on for performance where necessary).
Occasionally, I just want to bang my head against the wall because there are
things we can do so simply in any modern language that you just can't do in any
straightforward way in C. For example, I have file writers that all work
*exactly* the same way, with the single exception that the format string and the
data that you put into it vary for each file.

In Python, this would be straightforward to handle with the class machinery: you
could simply specify the format string in each inheriting class and define the
data points to be supplied at the top of an overriding function, call the parent
function with `super()` and be done.

To do something similar in pure C is nearly impossible. You can supply a format
string with each function (or module, or however you separate out the code), and
if you feel especially clever you could convert all your data types to strings
and pass them as a list to be printed by the standard function. The net result
would be *longer* and *less maintainable* than simply having a set of
essentially-duplicate functions, though.
