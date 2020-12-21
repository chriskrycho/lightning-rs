---
title: JavaScript Functional Immutable Update Performance
subtitle: What are the performance implications of “immutable functional updates”?
date: 2020-04-09T12:00:00-0600
tags:
    - JavaScript
    - functional programming
    - Redux
    - Q & A
summary: >
    Is using array spread or concat too expensive to use? It depends on how much data you have!
qualifiers:
    audience: >
        Programmers, particularly front-end JavaScript programmers who’ve encountered [Redux](https://redux.js.org), who are curious about functional programming and performance. Note that I’m not trying to persuade anyone about functional programming in this post—just answering a question about it!

---

In a chat group I’m a part of online, someone asked basically this question (I’ve tightened it up a bit for the purposes of this post):

<dl>

<dt>

I'm a complete newbie to Javascript and React, and something got my attention while I'm reading a React book: they advocate a functional programming style, which means things should be immutable. In a [Redux](https://redux.js.org) reducer, an "add" action updates the state by generating a new state like this:

```js
return [ ...state, newElement ]
```

That’s okay in Haskell, Lisp, Erlang, because those languages use linked lists, but in Javascript I would guess this will be <i>O(n)</i>,[^big-o-notation] right? That seems like overkill; why not just do this instead?

```js
state.push(newElement); return state
```

</dt>

<dd>

You’re correct; however, for many cases it doesn’t matter. If you have 30 elements in your array (or honestly even 300) that’s fine performance-wise. Creating a new array by iterating over the old one isn’t as fast as just inserting a new element, but it also is rarely the bottleneck, especially since using [the spread operator](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Spread_syntax) or using [the `.concat()` method](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/concat) do *shallow* copies of the data. When your arrays get large, it *does* matter, of course.

Also worth note: it’s not specifically the use of linked lists that makes it safe in other contexts; it’s the use of linked lists as one means of implementing persistent data structures. Elm and others also have arrays! They just have slightly different implementations and performance characteristics.

As for why you wouldn’t want to just use `push`: because doing so turns into “spooky action at a distance” pretty easily unless your whole team is exceedingly disciplined. I’ve spent non-trivial parts of the last two weeks looking at bugs (in the Ember app I work on) caused by people using push instead of functional-style updates, so this particular pain is very fresh for me.

You can also do `return state.concat(newElement)`, which has the same semantics as using the spread operator does.

It’s basically just a workaround for the fact that this stuff isn’t how JS natively behaves – JS kind of assumes mutation is the default at a language level.

</dd>
</dl>

[^big-o-notation]: If that <i>O(n)</i> notation is unfamiliar to you, don’t worry: it’s not as complicated as it might seem. This is an example of [Big-O Notation](https://medium.com/basecs/whats-a-linked-list-anyway-part-2-131d96f71996#95e1), which is just a convenient shorthand for the basic performance characteristics of an operation: “O” for the “order,” or growth rate, of the function. The thing inside the parentheses describes the relationship between the number of items <i>n</i> and that growth rate. <i>O(n)</i> means the growth rate is “linear”: it grows with the number of items involved. If it were <i>O(n<sup>2</sup>)</i> it would grow like the number of items involved *squared*; if it were <i>O(1)</i> it would be constant no matter how many items were involved.