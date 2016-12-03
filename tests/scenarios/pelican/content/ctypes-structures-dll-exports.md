---
Title: Python Enums, ctypes.Structures, and DLL exports
Subtitle: Illustrating the Simplest Use of ctypes Structures
Summary: >
    Unfortunately, the official docs for <code>ctypes</code> leaves a few things 
    out—namely, the most basic use case with <code>from_param</code>! Here's a simple, 
    working example from my own development work.
Tags: software development
Date: 2015-05-28 18:00
Slug: ctypes-structures-and-dll-exports
...

For one of my contracts right now, I'm writing a `ctypes` Python interface to
existing C code. I got stuck and confused for quite a while on getting the
interface to a given function to build correctly, and along the way had to try
to understand the `from_param` class method. The official docs are... fine...
but the examples provided don't cover the most common/basic use case: defining a
simple, *non-ctypes* data type as an argument to a DLL-exported function.

Let's say you have a C function exported from a DLL; for convenience we'll make
it something rather silly but easy to understand:

```c
/** my_exported.h */
#include "exports.h"

typedef enum {
    ZERO,
    ONE,
    TWO
} MyEnum;

MY_API int getAnEnumValue(MyEnum anEnum);
```

The implementation just gives back the integer value of the function:

```c
int getAnEnumValue(MyEnum anEnum) {
    return (int)anEnum;
}
```

As I said, a *very* silly example. Note that you don't technically need the
`(int)` cast there; I've just put it in to be explicit about what we're doing.

How would we use this from Python? Assuming we have a DLL named `my_dll` which
exports the `getAnEnumValue` function, we'd load it up roughly like this:[^1]

```python
import ctypes as c

my_dll = c.cdll.LoadLibrary('my_dll')
```

Then, we bind to the function like this:

```python
get_an_enum_value = my_dll.getAnEnumValue
```

Now, when you do this, you usually also supply the `argtypes` and `restype`
values for these functions. If you're like me, you'd think, "Oh, an enum---a
perfect opportunity to use the `Enum` type in Python 3.4+!" and then you'd do
something like this:

```python
import ctypes as c
from enum import IntEnum

class MyEnum(IntEnum):
    ZERO = 0
    ONE = 1
    TWO = 2

my_dll = c.cdll.LoadLibrary('my_dll')
get_an_enum_value = my_dll.getAnEnumValue
get_an_enum_value.argtypes = [MyEnum]
get_an_enum_value.restype = c.c_int
```

That seems sensible enough, but as it is, it won't work: you'll get an error:

```
TypeError: item 1 in _argtypes_ has no from_param method
```

This is because `argtypes` values *have* to be either existing `ctypes`
types[^2] or supply either:

  - a `from_param` classmethod, or
  - an `_as_parameter_` attribute

You can use `ctypes.Structure` subclasses natively that way, because the
`Structure` class supplies its `from_param` classmethod. The same is *not* true
of our custom enum class, though. As the docs put it:

> If you have defined your own classes which you pass to function calls, you
> have to implement a `from_param()` class method for them to be able to use
> them in the argtypes sequence. The `from_param()` class method receives the
> Python object passed to the function call, it should do a typecheck or
> whatever is needed to make sure this object is acceptable, and then return the
> object itself, its `_as_parameter_` attribute, or whatever you want to pass as
> the C function argument in this case. Again, the result should be an integer,
> string, bytes, a `ctypes` instance, or an object with an `_as_parameter_`
> attribute.

So, to make the enum type work, we need to add a `from_param` class method or an
`_as_parameter_` attribute to it. Thus, either of these options will work:

```python
class MyEnum(IntEnum):
    ZERO = 0
    ONE = 1
    TWO = 2

    # Option 1: set the _as_parameter value at construction.
    def __init__(self, value):
        self._as_parameter = int(value)

    # Option 2: define the class method `from_param`.
    @classmethod
    def from_param(cls, obj):
        return int(obj)
```

In the constructor-based option, the `value` argument to the constructor is the
value of the `Enum` instance. Since the value of anan `IntEnum` is always the
same as the integer to whcih it is bound, we can just return `int(value)`.

The `from_param` approach works a little differently, but with the same results.
The `obj` argument to the `from_param` method is the object instance, in this
case the enumerated value itself. *Any* `Enum` with an integer value can be
directly cast to `int` (though it is possible for `Enum` instances to have other
values, so be careful), and since we have an `IntEnum` here, we can again just
return `int(obj)` directly.

Now, let's say we want to apply this pattern to more than a single `IntEnum`
class, because our C code defines more than one enumeration. Extracting it to be
common functionality is simple enough: just create a class that implements the
class method, and inherit from it.

```python
class CtypesEnum(IntEnum):
    """A ctypes-compatible IntEnum superclass."""
    @classmethod
    def from_param(cls, obj):
        return int(obj)


class MyEnum(CtypesEnum):
    ZERO = 0
    ONE = 1
    TWO = 2
```

Our final (working!) Python code, then, would be:

```python
# Import the standard library dependencies
import ctypes as c
from enum import IntEnum


# Define the types we need.
class CtypesEnum(IntEnum):
    """A ctypes-compatible IntEnum superclass."""
    @classmethod
    def from_param(cls, obj):
        return int(obj)


class MyEnum(CtypesEnum):
    ZERO = 0
    ONE = 1
    TWO = 2


# Load the DLL and configure the function call.
my_dll = c.cdll.LoadLibrary('my_dll')
get_an_enum_value = my_dll.getAnEnumValue
get_an_enum_value.argtypes = [MyEnum]
get_an_enum_value.restype = c.c_int

# Demonstrate that it works.
print(get_an_enum_value(MyEnum.TWO))
```

The output will be `2`, just as you'd expect!

An important note: The type definition we've provided here will work for
`argtypes` or `restype` assignments, but *not*  as one of the members of a
custom `ctypes.Structure` type's `_fields_` value. (Discussing how you'd go
about  doing that is beyond the scope of this post; the most direct approach is
just to use a `ctypes.c_int` and note that it is intended to be used with a
given `IntEnum`/`CtypesEnum` type.)

---

Thanks to [\@oluseyi] for being my [rubber ducky] while I was working this out
earlier this week!

[\@oluseyi]: https://alpha.app.net/oluseyi
[rubber ducky]: http://en.wikipedia.org/wiki/Rubber_duck_debugging


[^1]: I'm leaving out the part where we build the DLL, and also the part where
    we locate the DLL, and only using the Windows convention. If you're on a
    \*nix system, you should use `'my_dll.so'` instead, and in any case you need
    to make sure the DLL is available in the search path.

[^2]: I *love* the redundancy of "`ctypes` types," don't you?
