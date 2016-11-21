---
Title: Free Dynamic DNS for Remote Login via SSH
Date: 2016-05-31 20:10
Category: tech
...

I recently set up a hostname and mapped it to a dynamic IP address for my home machine so that I can log into it via SSH[^1] from *anywhere* without needing to know what the IP address is. This is handy because I need to do just that on a semi-regularly basis: I'll be out with my work laptop at a coffee shop, and need something that's on my personal machine at home, for example.[^2]

A friend [asked](https://twitter.com/toddheitmann/status/728222459413958656) me to describe it, so here I am. (Hi, Todd!) This was pretty straightforward for me, and it should be for you, too.

1. Pick one of the [many](https://duckduckgo.com/?q=free+dynamic+dns+providers&t=osx&ia=web) free dynamic DNS providers. I picked [No-IP](http://www.noip.com/free) after a very short bit of digging. In the future I may switch to a more full-featured solution, not least because I'm planning to separate out my DNS management from my hosting and my domain registrar later this year.[^3] For now, though, No-IP is good enough.
    - Register.
    - Pick a domain name.
    - Add your current IP address. (If you need to find out what it is, you can literally just ask the internet: [whatsmyip.org](http://www.whatsmyip.org) will tell you.)
2. Set up a local service to talk to the dynamic DNS provider, so that when your external IP address changes (and from time to time it will, if you're not paying your ISP for a dedicated IP address). You can do this one of two ways:
    - **By installing a service on your main machine.** No-IP and other large providers all have downloads where you can just install an app on your machine that goes out and talks to the service and keeps the IP address up to date.[^4]
    - **By configuring your router.** This is the route I took, because the router I have[^5] fully supports dynamic DNS services right out of the box.[^6] Look for something like *Dynamic DNS* and follow the configuration instructions there to get it talking to your dynamic DNS service provider. Mine has a built-in list which included No-IP; I just added my username and password and the domain name I specified back in Step 1, checked an *Enable DDNS* box, and connected.

That's it. Even if you're not a huge networking geek (which, for all my nerdiness, I really am not), you can set it up. From that point forward, if you have *other* things configured locally on your  machine for network access (e.g. enabling SSH by toggling *Remote Login* to *On* in the **Sharing** preferences pane on OS X), you can just use the new domain you configured instead of the IP address. If that domain was e.g. \<chriskrycho.example.com\>, you could just `ssh chris@chriskrycho.example.com` and be off to the races.

Have fun![^7]

[^1]: or [mosh](https://mosh.mit.edu), which I'm hoping to check out this week

[^2]: Or, when I was traveling and my Windows VM crashed while I was in the airport, and I was able to work from the VM on my home machine instead via SSH magic I'll cover in a future blog post.

[^3]: Having each of those in a separate place is nice: it means that if the others change, you only have to deal with *that* set of concerns. For example, if you move hosting providers, you don't *also* have to migrate all your DNS settings---just tweak the couple that are relevant to the move.

[^4]: [Here's the download page](https://www.noip.com/download) for No-IP, for example.

[^5]: [this one](http://www.newegg.com/Product/Product.aspx?Item=N82E16833704177&nm_mc=AFC-C8Junction&cm_mmc=AFC-C8Junction-Skimlinks-_-na-_-na-_-na&cm_sp=&AID=10446076&PID=5431261&SID=skim45704X1167592X2be13284148d669370b61074c119afc2), as [recommended](http://thewirecutter.com/reviews/best-wi-fi-router/) by The Wirecutter

[^6]: So will most open-source router firmwares, especially OpenWRT or DD-WRT, if they run on your router. I've done that in the past.

[^7]: This tiny post has a *hilarious* number of footnotes. I noticed this early on, and instead of reworking it... I just ran with it.
