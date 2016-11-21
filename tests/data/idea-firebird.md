---
Title: FirebirdSQL and IntelliJ IDEA (etc.)
Author: Chris Krycho
Date: 2014-03-28 09:00
Summary: >
    Configuration instructions for FirebirdSQL JDBC with JetBrains IntelliJ IDEA
    platform (including PyCharm, RubyMine, WebStorm, etc.).
Tags: Software Development
...

Setting up IntelliJ IDEA's built-in database tools to work with FirebirdSQL
requires a particular setup configuration, which I'm documenting here for public
consumption.

These setup tools *should* be applicable to any of JetBrains' other Java-based
IDEs which include database support (e.g. PyCharm, RubyMine, WebStorm, etc.).
*Note:* the following apply to IntelliJ IDEA 12 and the associated platforms,
but *not* to the IDEA 13 platform, which made substantial changes to how
databases are configured. The underlying details are consistent, but the
interface has changed. I have tested on PyCharm 3.1 to confirm that.

This was all done on OS X 10.9, so I also make no guarantees that this works on
other platforms, though the likelihood that it behaves the same on Linux is
fairly good. I will update the post if and when I have confirmed that it does.

Steps to configuring a database correctly for use with IDEA/etc. Note that
steps 1--3 are fairly obvious; the real point of interest is in steps 4 and 5,
which took me the longest time to figure out.

1.  Download the latest version of the Firebird [Java drivers][jdbc] for your
    operating system and your Java version. (You can check your Java version by
    running `java -version` at the command line.) Extract the downloaded zip
    file. The extracted folder should include a file named
    `jaybird-full-<version>.jar` (`<version>` is currently 2.2.4).

2.  In IDEA, in the database view, add a new data source: in the Database view
    (accessible via a menu button on the right side of the screen), right click
    and choose **New -> Data Source**.

3.  Under **JDBC driver files**, browse to the location where you extracted the
    Jaybird driver files and select `jaybird-full-<version>.jar`.

4.  Under **JDBC driver class**, choose `org.firebirdsql.jdbc.FBDriver`.

5.  Under **Database URL**, specify `jdbc:firebirdsql://localhost:3050/`
    followed by *either* the full path to the database in question or a
    corresponding alias.[^alias] A full path might look like this on Windows:

        jdbc:firebirdsql://localhost:3050/C:/my_project/the_database.db

    With an alias, you would instead have:

        jdbc:firebirdsql://localhost:3050/the_alias

    Then specify valid values for the **User** and **Password** fields from
    your existing configuration of the database.

6.  Click the **Test Connection** button and make sure the configuration works.

That should do it. Note that the driver choice and path configuration both
matter. On OS X, I found that only the `FBDriver` with this (and one other,
older-style and therefore not recommended) path setup worked successfully.

Observations, corrections, additional information, and miscellaneous comments
welcomed on [App.net][adn] or [Twitter][tw].

[^alias]: I strongly recommend configuring an alias in the aliases.conf file in
    the Firebird home directory (usually set as `$FIREBIRD_HOME` during
    installation on \*nix systems). This lets you move the database around at
    will, update just the configuration file, and not have to update any
    references to the database file whatsoever.

[jdbc]: http://www.firebirdsql.org/en/jdbc-driver/
[adn]: https://alpha.app.net/chriskrycho
[tw]: https://www.twitter.com/chriskrycho
