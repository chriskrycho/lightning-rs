---
Title: If-expressions in Rust
Date: 2015-09-12 11:05
Tags: software development, programming languages, rust, python
Modified: 2015-09-12 11:10
...

I love the fact that all `if` statements in Rust are expressions. It gives you a
great deal of expressitivity in the language.

Let's contrast with Python (which I love, for the record). In Python, you can do
something like this:

```python
some_condition = True
if some_condition:
    a_value = "Yeah!"
else:
    a_value = "Oh, sads."
```

Those are *statements* in the body of the `if`/`else` block; you can't assign
the block itself to `a_value`. However, like C, C++, Java, etc., Python does
provide an *expression*-type conditional, a ternary expression.

So you can also do this:

```python
some_condition = True
a_value = "Yeah" if some_condition else "Oh, sads."
```

This expression form of the `if` block is what all Rust `if` blocks are. So in
Rust, the normal long form is:

```rust
let some_condition = true;
let a_value = if some_condition {
    "Yeah!"
}
else {
    "Oh, sads."
}
```

(You could also write this with a `let mut a_value` and then set its value
inside the conditional blocks, but that's not at all good form in Rust.)

And of course, you can shorten that rather nicely where the expressions are
brief enough:

```rust
let some_condition = true;
let a_value = if some_condition { "Yeah!" } else { "Oh, sads." }
```

But this gets really nice when you have more complicated work to do in a Rust
conditional. It doesn't matter how many things going on inside an `if`
expression; it's still an expression. As such, you can also write this:[^compiler]

```rust
let some_condition = true;
let a_value = if some_condition {
    let the_answer = 42;
    let theme = "Take my love, take my land...";
    "Yeah!"  // An expression!
}
else {
    let the_question = "What do you get when you multiply six by nine?";
    let song = "You can't take the sky from me!";
    "Oh, sads."  // An expression!
}
```

Obviously this is totally contrived and silly; the point is that no matter what
the internals are, `if` blocks are expressions, and their final expressions can
be assigned like any other.

---

As a note: I got here because I was originally thinking you couldn't do a
one-liner like you can in Python. As shown above, that's totally false, and in
fact the Rust version is much more capable than Python's, because you don't need
a dedicated ternary when all `if` blocks are expressions. Rust used to have a
C-style ternary (`<condition> ? <value if true> : <value if false>`) but it was
[removed] during the lead-up to the 1.0 release---a decision I wholeheartedly
affirm.

[removed]: https://github.com/rust-lang/rust/issues/1698

[^compiler]: Note that under normal conditions the compiler won't actually
    accept this because of the unused names.
