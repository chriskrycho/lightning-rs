---
Title: Rust vs. React Native—What?
Subtitle: "Tradeoffs, or: Speed and stability are for everyone"
Tags: JavaScript, Rust, Swift, F Sharp, software development
Date: 2016-10-07 08:20
Category: Tech
---

<i class=editorial>I was recently discussing some thoughts I’ve had on building a top-notch application experience in a Slack team I belong to, and noted that I believe that a Rust core with native UIs is a *massively* winning strategy. A friend in the group responded that he thinks "React + JS is eating the world right now" and that "Rust as awesome for if you want to write a JS vm, or something like that… or a compiler… anything involving lots of speed and stability." What follows is my response, lightly edited to remove details specific to that friend and to add a few further thoughts.</i>

---

> Here’s the thing: I don’t *care* what’s eating the world today, for three reasons:
>
> 1. I just want to build the best stuff I can build, and native UIs are still massively better than React and even React Native[^rn] in innumerable ways. There are clear advantages to React Native + JavaScript, and times when you absolutely should take that approach. But there are also a lot of times and reasons why you shouldn’t. Heck, even if you just want killer performance *in browsers*, our future includes things like Rust-to-WebAssembly, and that’s a good thing.
> 2. What was eating the world five years ago? Ten? Is it still eating the world today? I don’t feel obliged to follow those trends (not least because, not being a consultancy, following those trends doesn’t buy me anything for the things I want to do; your tradeoffs and mine look way different).
> 3. I’m actually getting really tired of just treating as acceptable or normative the performance characteristics of browsers. Browsers are awesome. But we can (and should) do a _lot_ better in terms of user experience, and I don’t see browsers catching up to what you can do with e.g. Cocoa (Touch). Sure, that doesn’t matter that much for building yet-another-storefront. (Again, there are different tradeoffs for every single app!) But why in the world are we in a spot now where one of the most popular text editors in the world is _slower_ than any text editor of five years ago? That’s not a *necessary* decision, and you can (and should) go after the same degree of ease-of-extensibility that Atom has had—perhaps even using things like HTML and CSS for skinning!—while not tying yourself to the browser and its upsides and downsides for _everything_. We have _incredibly_ powerful machines, and the user experience is often getting _slower_. I’m looking for ways to change that.
>
> Again, JS+React[^or-ember] may be *exactly* the right tradeoff for a lot of apps, and given what consultancies (like my friends’s!) are doing, I think doing that with ReactNative for apps is a *very* good move. It makes good sense business-wise, and it makes good sense in terms of the apps you’re likely to be delivering. Don’t hear me for a second saying Rust is the best for *everything*. I think it, or something like it, is a very good choice for *many* things, though, and it shouldn’t be dismissed simply because it’s a very different world from doing Ruby or Elixir or JavaScript.
>

[^rn]: I'm aware that React-Native ultimately binds down to native widgets. It's still not quite the same.

[^or-ember]: or, frankly, Ember or whatever else; React is great, but it is also overhyped.

---

<i class=editorial>So much for my initial response. On reflection, I wanted to expand it a bit. So here’s another few hundred words!</i>

Beyond this, I think there’s a bit of a false dichotomy here: the idea that "lots of speed and stability" *aren’t* values we should be seeking more aggressively for *all* our apps. Fully granted that not every app needs the same *degree* of each of those, and moreover that there are a lot of ways to get to those goals. Still: speed and stability are *core* user experience values. I don’t really care how you get at those goals, whether it’s with Rust, or Elixir or Clojure, or, yes, React with TypeScript or [Flow][flow]. I *do* think that Rust is, for the moment at least, uniquely positioned to add real value in this space because it gives screaming performance but with so many niceties we’re used to when writing languages like Python or Ruby and so much of the power you get in languages like OCaml or F♯.[^swift] But at the end of the day, I think *all* apps should focus much more on speed and stability than they do today. We have supercomputers in our pockets, and we’re often shipping apps that are slower and more finicky.

[flow]: https://flowtype.org

[^swift]: Swift too, and honestly for a lot of things Swift is an easier experience for not *that* much less performance than Rust. But as of today you *can’t* ship core functionality in Swift for Android or Windows.

But I have this dream of a world where apps aren’t  needlessly power-hungry or memory-intensive, where every swipe and or click or scroll results in buttery-smooth responses. We won’t get there by saying, “You know, Facebook is doing _x_ so that’s good enough for me.”

Of course every developer, and any given product shop or consultancy, is going to have to make decisions about which stacks it invests in. If you’re primarily shipping web applications, investing in Elixir and React with React Native for your apps is a very sensible move. Most of your clients’ native apps may not *need* the degree of polished performance you might get from writing their iOS app in Swift and their Android app in Kotlin and the core in Rust (or even C++). That tradeoff is a *tradeoff*.

But let’s remember that there is real value there, and that some apps *do* deserve that investment. We should evaluate the tradeoffs at every turn, and our core considerations should enduringly include *speed and stability*. Don’t dismiss Rust (or Swift, or F♯) out of hand.

Equally importantly, we need to stop assuming that just because something is eating the world today means it’s also the future. Betting big on Flash in the mid-2000s wasn’t a *bad* move by a long shot. But its massive popularity then wasn’t a good predictor for its future. That goes double, frankly, for projects coming out of Facebook or Google or similar: big companies like that have the resources to drop everything and use a new language, or a new tool, as it suits them. If you don’t believe me, look at the actual open-source records of both of those companies! What’s hot today is far more relevant to a consultancy than to a product shop. And in both cases, choosing tech suitable for the job at hand is more important yet.

My friend gets that, for what it’s worth. He’s making the right moves for his business as the owner of a consultancy. I just want him—and lots of other people—to see where languages like Rust and Swift and F♯ might be worth considering. And speed and stability matter in a lot of places besides just compilers and VMs.