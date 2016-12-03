Title: Learning QML, Part 1
Author: Chris Krycho
Date: 2014-04-11 15:30
Tags: software development

For part of my work with Quest Consultants, I've been picking up Qt's QML
toolkit to use in building out the UI. The declarative syntax and ability to
define one's own model in non-C++- or Python-specific ways is quite nice. That
said, the learning process has had more than a few bumps along the way. I
decided to go ahead and write those up as I go, both for my own reference and
in the hope that it may prove useful to others as I go.

QML is a *Javascript-like* language for *declarative programming* of a user
interface. So it's a Javascript-based language that sort of behaves like HTML.
In fact, it behaves like Javascript in terms of how you define, access, and
update properties, and you can embed full-featured (mostly) Javascript functions
and objects in it.

But when you have nested QML Types, you end up with them behaving more like
HTML.

The weirdest bit, and the thing that I'm having the hardest time adjusting to,
is that you can only edit properties of root Types when you're working with an
instance of that Type. And those Types are defined by _documents_.

So, to give the simplest possible example, let's say I defined a new type called
`Monkey`, in the `Monkey.qml` file, like this:

    // Monkey.qml
    import QtQuick 1.1

    Item {
        id: monkey_root
        property int monkey_id: -1
        property string monkey_name: "I don't have a name!"

        Item {
            id: monkey_foot
            property string monkey_foot_desc: "The monkey has a foot!"
        }
    }

I can use that in another file. If they're in the same directory, it's
automatically imported, so I can just do something like this:

    //main.qml
    import QtQuick 1.1

    // Rectangle is exactly what it sounds like. Here we can display things.
    Rectangle {
        id: the_basic_shape
        height: 400
        width: 400
        color: green

        Monkey {
            id: monkey_instance
            monkey_id = 42
            monkey_name = "George"  // he's kind of a curious little guy
        }

        Text {
            text: monkey_instance.monkey_name
            color: "red"
        }
    }

That creates a (really ugly) rectangle that prints the `Monkey`'s name in red
text on a green background. It's impossible to access directly the `monkey_foot`
element, though, which means that composing more complex objects in reusable
ways is difficult. In fact, I haven't come up with a particularly good way to do
it yet. At least, I should say that I haven't come up with a good way to create
high-level reusable components yet. I can see pretty easily how to create
low-level reusable components, but once you start putting them together in any
_specific_ way, you can't recompose them in other ways.

From what I've gotten my head around so far, this ends up being less flexible
than either HTML templating languages (which are, or at least can be, completely
declarative) or normal Javascript (which is obviously _not_ declarative). Mind
you, it's all sorts of *interesting*, and I have a pretty decent idea what I'm
going to do to implement our UI with it, but it's taken me most of the day to
get a good handle on that, and my head still feels a bit funny whenever I'm
trying to see how best to create composable components.

Note, too, that this is the *only* way to create a new basic type of object in
QML: it has to be the root level object in a QML document. I would *really* like
to be able to access internal declarations---to have named internal
types/objects. Unfortunately, QML doesn't let you do this. I suspect this has to
do with how the QML type system works: it actually binds these types to C++
objects behind the scenes. This is a non-trivially helpful decision in terms of
the performance of the application, but it certainly makes my brain a little bit
twitchy.

There are two basic consequences of this structure. First, any types you need to
be able to use in other QML objects have to be defined in their own QML
documents. Second, it is (as near as I can see so far, at least) difficult to
create good generic QML types of more complex structures that you can then use
to implement specific variations. For example: if you want to create accordions,
you can create a fair number of the low-level elements in generic ways that you
can reuse, but once you get to the relationships between the actual model,
delegate, and view elements, you will need to create them in custom forms for
each distinct approach.

This is more like creating HTML documents than Javascript, which makes sense,
*if* you remember that QML is Javascript-based but *declarative*. You just have
to remember that while you can define some reusable components, the full-fledged
elements are like full HTML pages with a templating system: you can include
elements, but not override their internal contents. In QML, you can override
*some* of their contents, which is nice---but that is not the primary way to go
about it.
