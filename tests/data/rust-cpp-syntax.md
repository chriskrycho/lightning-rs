---
Title: Rust and C++ function definitions
Subtitle: A small study in syntax and legibility.
Date: 2016-06-03 18:01
Modified: 2016-06-07 23:16
Tags: programming languages, rust, cplusplus
Slug: 03-1801
---

I just put my finger on one of the (many) reasons Rust reads better than C++: the visual consistency of its function definitions. Compare---

Rust has:

```rust
fn foo() -> i32 { /* implementation */ }
fn bar() -> f32 { /* implementation */ }
```

C++ has:

```cpp
int foo() { /* implementation */ }
double bar() { /* implementation */ }
```

That consistency adds up over many lines of code. There are many other such choices; the net effect is that Rust is *much* more pleasant to read than C++.

---

Note: I'm aware that C++11 added the `auto foo() -> <type>` syntax. But this actually *worsens* the problem. A totally new codebase which uses that form exclusively (which may not always be possible, because the semantics aren't the same) would have roughly the same visual consistency as Rust *in that particular category*. (Plenty of others would still be a mess.) But the vast majority of C++ codebases are *not* totally new. Adding the form means your codebase is more likely to look this this:

```cpp
int foo() { /* implementation */ }
auto quux() -> uint32_t { /* implementation */ }
double bar() { /* implementation */ }
```

That is, for the record, *more* visual inconsistency---not less!
