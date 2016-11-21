Title: Facebook's "Security" Requirements
Subtitle: No 2FA Unless You Let Us Track You
Author: Chris Krycho
Date: 2015-02-21 12:35

I went to set up 2-step login (AKA 2-factor authentication, or what Facebook calls "Login Approvals") on Facebook yesterday morning, and was greeted with this lovely message when I clicked "enable":

> Your current Firefox settings might make it hard to use Login Approvals. It's probably because:
>
> - You sometimes clear your cookies.
> - Your browser is set to automatically clear cookies whenever it closes.
> - You use your browser's "private browsing" or "incognito" mode.
> - You're using a new browser.
>
> It may take a few days after fixing these issues before you will be able to enable Login Approvals. You also may need to log out and then log in again after fixing these settings for the changes to take effect.
>
> Visit the Help Center for step-by-step directions on how to fix these settings.

I use Firefox for the social media access I do online---and because I don't like being tracked, I tell Firefox not to remember history and to delete cookies as soon as I close the browser, and I run [μBlock][μBlock][^block] and [Disconnect].[^track]

[μBlock]: https://github.com/gorhill/uBlock
[Disconnect]: https://disconnect.me/

When you attempt to enable 2-step login, Facebook checks your security policy... and *will not let you turn it on* if your settings are like mine. They supply the message above, with no option to proceed anyway. Of course, there is no technical issue with using 2-step login with a browser configured this way. I use it for GitHub, Google, my domain registrar, and every other service with 2-step login.

Facebook probably has two motives here. The better one is user experience: it *would* be frustrating if you are a non-tech-savvy user who doesn't understand the consequences of setting this given the browser settings I have. But of course, if they were primarily just concerned with that, they could give the warning and then let users say, "Go ahead; I know what I'm getting into." The second, less obvious but almost certainly more important motive from Facebook's point of view, is to discourage people from using a browser the way I do. They want to be able to monetize my Facebook use better, and this means not just my time on Facebook, but my time all over the web. Facebook wants to know what I'm looking at any time I'm surfing *anywhere* so that they can tailor their ads to me.

I'm not interested in being tracked that way.

Apparently, Facebook isn't interested in letting people have actual, modern security unless they're willing to be tracked that way.

We have a problem here.

As it turns out, of course, people like me aren't particularly valuable customers to Facebook anyway, so they probably don't mind the fact that they're losing more and more of our time. But losing that time they are. My use of Facebook is diminishing at an ever-increasing rate, for countless little reasons like this, where Facebook's ad-driven motivations push them to treat me poorly. Too bad for them.

[^block]: If anyone tells you that blocking ads is "stealing", they're talking up nonsense. The Internet is built in such a way that if nothing else you can always just request the plain text version of a website, and that's extremely important for many reasons, including accessibility. I *choose* to leave ads on for any number of sites I want to support, but at the end of the day it's every publisher's choice how theyw ant to make money. If a newspaper supports itself with ads and coupons, I have every right to throw them in the trash without a glance; the same is true online.

[^track]: Yes, I know this isn't foolproof and I'm still being tracked. It's impossible *not* to be tracked to some degree or another. What I am doing here is *decreasing* the degree to which companies can track me.