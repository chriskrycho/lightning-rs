---
title: >
  Things I Was Wrong About: Types
subtitle: >
  Because it would do us all good to be a little more honest about where we’ve changed our minds or simply been mistaken.
summary: >
  I have changed my mind about some topics big and small so far in my life. Today: the value of types for programming.
qualifiers:
  audience: >
    People who care about *actually* learning (in this case, software developers in particular). Also, my wife, who really enjoys hearing me say, “I was wrong.” 😂
date: 2020-09-26T14:00:00-0600
updated: 2020-09-28T08:35:00-0600
series:
  part: 1
tags:
  - software development
  - humility
thanks: >
  Nick Morgan let me know about a typo. A number of readers and commenters on [Hacker News](https://news.ycombinator.com/item?id=24604943) and [lobste.rs](https://lobste.rs/s/v1nxkm/things_i_was_wrong_about_types) provided a variety of insightful responses.

---

It’s pretty rare, at least in my experience, for people to discuss things they were *wrong* about in the past. Even when we post “I think this new thing!” we rarely call out exactly what it was we changed our minds about. So in this series—as part of my ongoing exercise of modeling *learning in public* on this site—I’m going to dig through things I currently think I was simply *wrong* about. Topics will range from software (as today) to theology and anything else that comes to mind!

---- 

If you talked to me about types in programming languages six or seven years ago, you would quickly have learned that I was *not* a fan. I had spent the preceding years working in Fortran and C, with a touch of Java thrown in, and my experience of types was that they added a great deal of overhead and didn’t remotely pay for themselves. I was working with Python and JavaScript and simply didn’t miss types *at. all*, even in fairly complex systems. I told a colleague at one point that I thought types were basically worthless.

In 2015, all of that changed. I read [<cite>Maybe Haskell</cite>](https://gumroad.com/l/maybe-haskell/) on a plane that spring, and I encountered [Rust](https://www.rust-lang.org) about two months later, and all of a sudden I had a very different outlook on types. It’s not an exaggeration to say that the entire trajectory of my professional career shifted in that two-month span. Since then, I’ve run [a podcast about Rust](https://newrustacean.com) which included no few discussions about type theory, helped bootstrap an [entire sub-community](https://ember-cli-typescript.com) for another [typed programming language](https://www.typescriptlang.org) (which is now part of my day job), and written a hilarious amount about types and how they can help.

I understand why I thought that types were worthless from 2012–2014. In fact, for the specific languages I had used up to that point, I continue to think that the types *don’t* really pay for themselves. But I have substantially changed my tune about type systems more generally. *Some* type systems can make a substantial difference in the kinds of programs you can write, and the degree of value you get out of them is much higher relative to the effort you put in to get that value.

The key differentiators between the type systems I didn’t value and those I now do—between thinking all type systems were worthless and thinking a *good* type system is worth its weight in gold—were:

- **Type inference:** because having to write out every type, however obvious, is an incredible waste of time. `Person me = new Person();` is ridiculous. `let me = new Person();` may seem like a small improvement, but spread over the body of an entire program and generalized to all sorts of contexts means that type annotations become a tool you employ *because they’re useful*—for communicating to others, or for constraining the program in particular ways—rather than merely because the compiler yells at you about something it should know perfectly well.

- **Sum/tagged union types:** because a great many of the problems we solve in software come down to well-defined options: *A* or *B* or *C*, and with discrete options for what data goes with each of *A* and *B* and *C*. Lots of problems also *don’t* fit into that space, but enough do that missing a tool to express it feels painful. In fact, I had been reaching for my own ways of building “sum types” in C and Python for years before finally discovering that the idea already existed and the tools around it were great… in other languages. The fact that languages with first-class support for sum types also come with exhaustiveness checking for those was the icing on the cake. It meant that I could finally tell the computer what I meant, and have it check me—and it also meant that if I changed my mind later, I could tell the computer *that* and have it tell me every place I needed to update my assumptions.

- **Soundness:** I was sick to death of `null` and `NullPointerException` and `undefined is not a function` and `TypeError: object of type 'NoneType' has no len()`. But at least in untyped languages, I had only myself to blame for these errors. Getting them in a language with types was utterly infuriating: why couldn’t the compiler tell me that I had missed a case where something could be `null`? And this problem was general: the compiler simply couldn’t tell me whether my program actually followed the rules I wanted it to follow, no matter how much work I put into the types. Soundness changed all of that. It didn’t mean I was free from logic bugs. (Nothing can do that in the general case!) It did mean that a program which type-checked wouldn’t blow up *in ways the type-checker said it shouldn’t*, though.

These three differences allowed me to turn types into tools for thought, means of communicating both to other programmers and to the computer. It let me specify the invariants I wanted to remain true, assured that the compiler would tell me if they *actually* held true or not, and assured that the compiler wouldn’t just be lying to me about it.

Types are not perfect. They still have tradeoffs. Some type systems *aren’t* worth it. But five years ago, I changed my mind about the value of type systems *in general*, because I learned about type systems that I hadn’t known about previously. And, critically, this taught me to be far less dogmatic about the value of ideas in programming languages and software development in general. If smart people see the value in something and I don’t, it’s quite likely that I have missed something, and there’s something to learn from them!