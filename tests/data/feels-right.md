Title: Feels Right
Summary: Little details in how things work can make all the difference when it comes to the experience of using software. So be diligent, and do it right.
Author: Chris Krycho
Date: 2014-04-04 21:30
Tags: software development, design

I had spent most of the last week and a half working on getting
[FirebirdSQL][fb] configured and ready to use for a project I'm working on with
[Quest Consultants][quest]. It was slow going. The tool is decent, but the
documentation is spotty and it felt like everything was just a bit of a slog---to
get it working correctly, to get it playing nicely with other pieces of the
development puzzle, to get it working across platforms.[^1] Then, because I had
done something a *little* bit silly in my eagerness to get up and going last
week and written code without a testable configuration, I hit a wall today. The
queries weren't working. I had made a [bug][so].

I spent a substantial part of the day chasing down that bug, and then a
conversation with user *agronholm* on the [SQLAlchemy][sa] IRC channel
([freenode/#sqlalchemy][sa-irc]) got me thinking. The Firebird team describes
one of their options as an "embedded" server, but *agronholm* pointed out that
what they really mean is *portable*. It's running a standalone server and
client, but it's not part of the same thread/process (like SQLite is). Then
*agronholm* very helpfully asked---my having mentioned my preference for
[PostgreSQL][postgres] earlier---"Does Postgres not have a portable version?"
Two minutes later, we had both found [PostgreSQL Portable][portable], and I
rejoiced.

It took me less than half an hour to get it downloaded and set up and to confirm
that it would work the way we need for this particular piece of software.
(Firebird had taken me a good three hours, what with digging through badly
organized and not terribly clear documentation.) It took me less than half an
hour more to get PostgreSQL to the same point that I'd finally gotten Firebird
to after multiple hours working with it. And I was so *very* happy. What had
been an especially frustrating work day now had me quietly smiling to myself
constantly for the last two and a half hours as I [finished][so-answer] tracking
down the bug that had set me on this path in the first place.

Several years ago, when I first started doing web development, I got my feet wet
in database work with MySQL---probably the single most common starting point for
anyone going that route, courtesy of the ubiquity of the standard Linux-Apache-
MySQL-PHP stack.[^2] A year after that, I picked up some work that was already
using PostgreSQL and fell in love almost immediately.[^3] Something just felt
*better* about running `psql` than running `mysql` on the command line.
Postgres' implementation of the SQL standard felt more natural. Even the tiniest
little details like the way tables display when you query them in `psql` was
nicer. In less than a week, I was sold and haven't looked back. While I've used
MySQL out of convenience on shared hosting from time to time, PostgreSQL is
unquestionably my preferred database target.

Today's experience brought that all home again. That grin on my face all
afternoon felt a bit silly, but it highlights the difference that really good
software design makes. I am not just talking about how it looks here---though,
to be sure, PostgreSQL is prettier than FirebirdSQL---but how it works.
PostgreSQL feels responsive, its command set makes a lot of sense and is easy to
use, and it is *extremely* well documented. In fact, I would go so far as to say
that it is the best documented open source software I have ever used, as well as
among the very most robust. (The only other open source software I find to be as
incredibly rock-solid and reliable as PostgreSQL is the Linux kernel. I am by no
means an expert on either, or on open source software in general, but the Linux
kernel is an unarguably amazing piece of work. So is PostgreSQL.) All those tiny
little details add up.

It's a good reminder for me as I write software that yes, the things I care
about---the small matters that would be so easy to overlook when customers
express no interest in them---really do matter. People may not know that things
like typography make a difference in their experience, but those subtle, often
imperceptible things matter. They may not consciously notice the differences in
your interface design (even a command line interface), but it will change their
experience of the software. Do it poorly, or even in a just-good-enough-to-get-
by fashion, and you'll annoy or simply bore them. Do it well, and you might just
delight them---even if they can't tell you why.

---

## Examples

To make my point a little more visible, I thought it might be useful to post
samples of SQL to accomplish the same task in the two different database
dialects.

### FirebirdSQL:[^4]

    CREATE TABLE projects (
      id INT NOT NULL PRIMARY KEY,
      title VARCHAR(32) NOT NULL,
      file_name VARCHAR(32) NOT NULL,
      file_location VARCHAR(256) NOT NULL,
      CONSTRAINT unique_file UNIQUE (file_name, file_location)
    );
    CREATE SEQUENCE project_id_sequence;
    SET TERM + ;
    CREATE TRIGGER project_id_sequence_update
      ACTIVE BEFORE INSERT OR UPDATE POSITION 0
      ON projects
    AS
    BEGIN
      IF ((new.id IS NULL) OR (new.id = 0))
        THEN new.id = NEXT VALUE FOR project_id_sequence;
    END+
    SET TERM ; +

### PostgreSQL

    CREATE TABLE projects (
      id SERIAL NOT NULL PRIMARY KEY,
      title VARCHAR(32) NOT NULL,
      file_name VARCHAR(32) NOT NULL,
      file_location VARCHAR(256) NOT NULL,
      CONSTRAINT unique_file UNIQUE (file_name, file_location)
    );

It is not just that the PostgreSQL example is shorter and clearer---it is that
it is shorter and clearer because its designers and developers have taken the
time to make sure that the shorter, cleaner way works well, and have documented
it so you can know how to use that shorter cleaner way without too much
difficulty.

[^1]: I do most of my development on a Mac, but do all the testing on the target
    platform (Windows) in a VM.

[^2]: At this point, I would only use one of those by default if I were building
    a web app: Linux. I'd use [nginx][ng] instead of Apache,
    [PostgreSQL][postgres] instead of MySQL, and [Python][python] (though
    [Ruby][rb], Javascript via [node.js][node], [C# and the .NET stack][.net],
    or just about anything *but* PHP would do fine).

[^3]: *Almost* immediately because at that point configuration on OS X was a bit
    of a pain. That is [no longer the case][pg.app].

[^4]: To be perfectly fair to Firebird, it is improving. The upcoming 3.0 series
    release will make these two a lot more similar than they are at present, and
    clean up a number of other issues. What it won't do is get the *feel* of
    using Firebird more like that of using Postgres, or make the installation
    procedure smoother or easier, or make the documentation more complete.

[fb]: http://www.firebirdsql.org
[quest]: http://www.questconsult.com
[so]: http://stackoverflow.com/questions/22865573/sqlalchemy-successful-insertion-but-then-raises-an-exception
[sa]: http://docs.sqlalchemy.org/en/rel_0_9/
[sa-irc]: irc://irc.freenode.net/sqlalchemy
[postgres]: http://www.postgresql.org
[portable]: http://sourceforge.net/projects/postgresqlportable/
[so-answer]: http://stackoverflow.com/questions/22865573/sqlalchemy-successful-insertion-but-then-raises-an-exception/22872598#22872598
[ng]: http://wiki.nginx.org/Main
[python]: https://www.python.org
[rb]: https://www.ruby-lang.org/
[node]: http://nodejs.org
[.net]: http://msdn.microsoft.com/en-us/vstudio/hh341490
[pg.app]: http://postgresapp.com "Postgres.app"
