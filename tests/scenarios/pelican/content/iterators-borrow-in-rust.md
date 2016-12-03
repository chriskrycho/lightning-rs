---
Title: Vectors and Iterator Access in Rust
Subtitle: Be careful not to move things instead of borrowing them.
Category: Tech
Date: 2016-06-16 20:59
Tags: rust
---

<i class="editorial">In the midst of doing my reading and research for New Rustacean episode 15 (which will be out fairly soon after I post this), I bumped into this little tidbit. It doesn't fit in the episode, so I thought I'd share it here.</i>

When you're dealing with vectors in Rust, a common misstep when working with them via iterators is to *move* them when you only to *borrow* them. If you write `for i in x` where `x` is an iterator, you'll *move* the iterator into the looping construct. Instead, you should nearly always write `for i in &x` to borrow a reference to the iterator, or `for i in &mut x` if you need to get a mutable reference to it.
