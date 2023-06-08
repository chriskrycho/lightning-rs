---
title: Ulysses Publishing With WordPress on Linux
subtitle: A tech tip for other folks using WordPress on custom Linux setups.
qualifiers:
    audience: >
        Anyone who uses XML-RPC with WordPress on Linode, especially would-be publishers-from Ulysses.
summary: >
    Dealing with Ulysses’ “compatbility problem” and insecure connection warnings on new Linode One-Click WordPress app configurations.
tags:
    - WordPress
    - Ulysses
    - Linux
    - Debian
    - TLS
    - SSL
    - HTTPS
    - blogging
date: 2020-02-02T19:40:00-0600
updated: 2020-02-08T17:20:00-0600

---

I spent some time today migrating my wife’s long-dormant (but soon to be no-longer dormant!) [website](https://jaimiekrycho.com) from a shared hosting setup to a dedicated Linode setup I can manage myself. I ran into an interesting issue when *finishing* the setup, and figured I’d document it clearly to explain what the issue is and how to fix it.

The defaults supplied out of the box on Linode’s One-Click app setup for WordPress have two issues for supporting this flow, which will :

1.  Right out of the box, you will see Ulysses respond with an error indicating that there is a “compatibility problem.” This shows up because Ulysses communicates with WordPress via its XML-RPC API… and the default configuration from Linode [blocks XML-RPC](https://www.linode.com/community/questions/18414/does-linode-block-xml-rpc):[^xml-rpc-api]

    ```apache
    <files xmlrpc.php>
        order allow,deny
        deny from all
    </files>
    ```

    You can fix this by simply deleting that block.

    (There are a couple other blog posts out there on this same subject, and they recommend doing a bunch of other workarounds, all intended basically to allow XML-RPC connections to work while not exposing this particular file name. These workarounds as well as the original default exist because XML-RPC is occasionally a vector for attacks on servers. In my case, I’m not currently all that concerned about that; if it comes up I’ll deal with it then.)

2.  The `ServerName` value in the Apache config does not correctly work for [Certbot](https://certbot.eff.org) to set up your site to automatically forward HTTP connections to HTTPS. Unfortunately, HTTPS connections are a (soft, but *highly* recommended) requirement for Ulysses to connect to the setup, and if forwarding isn’t enabled, Ulysses complains (as it should!). The problem here is that the default Apache config on the Linode One-Click WordPress app supplies the *IP address of the server—rather than the domain name for your site—as the `ServerName` value. Changing that fixes the Certbot issue, and thereby unblocks the Ulysses-WordPress connection.

    In our case, I needed to change it from `ServerName <ip address>` to `ServerName jaimiekrycho.com`, in both the `/etc/apache2/sites-enabled/wordpress.conf` and `/etc/apache2/sites-enabled/wordpress-le-ssl.conf` files, and then to run Certbot again to reinstall the certificate and configure it to forward all HTTP connections to HTTPS. At least on my machine, it wouldn’t do that last step until I had rewritten those `ServerName` entries.

Once I had made those two changes, everything worked nicely! I hope this ends up helping you if you run into the same.

[^xml-rpc-api]: If you’re thinking that it would be really nice if WordPress offered a modern JSON API instead of being stuck with XML-RPC, well… I’m with you.

    :::note
    A correction from [a reader][jws]! WordPress [*does* have a JSON API][wp-api], and has for almost half a decade now! I have *no idea* why Ulysses is using XML-RPC instead of that API; at first blush [it certainly looks like it could][wp-api-post]. My bad for not checking this and just assuming the problem was on WordPress’ end rather than Ulysses’.
    :::

*[XML-RPC]: XML (“extensible markup language”) remote procedure call
*[API]: application programming interface
*[HTTPS]: HyperText Transfer Protocol Secure
*[HTTP]: HyperText Transfer Protocol
*[IP]: Internet Protocol
*[JSON]: JavaScript Object Notation

[wp-api]: https://developer.wordpress.org/rest-api/
[wp-api-post]: https://developer.wordpress.org/rest-api/reference/posts/
[jws]: https://jeremywsherman.com
[creating]: https://developer.wordpress.org/rest-api/reference/posts/#create-a-post