---
title: Making Illegal States Unrepresentable—In TypeScript
subtitle: >
    Showing how Scott Wlaschin’s approach in F^♯^ translates to a language with a very different type system.
date: 2020-05-25T10:40:00-0600
updated: 2020-09-07T21:18:00-0600
qualifiers:
    audience:
        Software engineers who are already persuaded of the value of type systems, and are interested in using them more effectively. (I’m not trying to persuade people that type systems are valuable here!)
    epistemic: >
        I’ve done this a *lot* over the last few years; I can’t imagine working without it anymore.
thanks: >
    Thanks to [Scott Wlaschin](https://fsharpforfunandprofit.com/) for giving permission to reuse his materials this way!
tags:
  - TypeScript
  - software development
  - type theory
  - F#
  - functional programming

---

One of the most important posts I’ve encountered in the last half decade was Scott Wlaschin’s [Designing with types: Making illegal states unrepresentable][wlaschin]. The big idea is to use types—in the original blog post, in F^♯^—to make it so you *never* have to worry about a whole class of bugs in your business logic.

[wlaschin]: https://fsharpforfunandprofit.com/posts/designing-with-types-making-illegal-states-unrepresentable/

In the rest of this post, I’m just going to take Wlaschin’s original idea and translate it fairly directly to TypeScript. Accordingly, it’s worth understanding that there is basically *nothing* original in the rest of the post. I’ve borrowed the headings directly from Wlaschin’s post, and quoted him directly in a number of cases; this is all entirely his  Wlaschin!

## Making Illegal States Unrepresentable

Wlaschin starts by introducing the idea of a `Contact` type, which represents something like an entry in an address book app. Initially we might model it like this:

```ts
class Contact {
  name: Name;
  emailContactInfo: EmailContactInfo;
  postalContactInfo: PostContactInfo;

  constructor(
    name: Name,
    emailContactInfo: EmailContactInfo,
    postalContactInfo: PostalContactInfo
  ) {
    this.name = name;
    this.emailContactInfo = emailContactInfo;
    this.postalContactInfo = postalContactInfo;
  }
};
```

:::note

For scenarios like this where the `constructor` does nothing but set up some properties with values passed in, TypeScript provides a constructor-only shorthand which looks like this:

```ts
class Contact {
  constructor(
    public name: Name,
    public emailContactInfo: EmailContactInfo,
    public postalContactInfo: PostalContactInfo
  ) {}
}
```

I will be using this style throughout the rest of the post, because these classes are just a lightweight way of setting up a kind of “record” to pass around some data. For the same reason, I will also mark all of these types as `readonly`: the intent is “don’t change these in any given instance.”

```ts
class Contact {
  constructor(
    public readonly name: Name,
    public readonly emailContactInfo: EmailContactInfo,
    public readonly postalContactInfo: PostalContactInfo
  ) {}
}
```
:::

## Background

First, a little setup. The types `Name`, `EmailContactInfo`, and `PostContactInfo` are simple types that wrap around a string, so you can’t accidentally mix them up and use them where the other one belongs. The simplest way to write these is something like this:

```ts
class Name {
  constructor(private readonly value: string) {}
}

class EmailContactInfo {
  constructor(private readonly value: string) {}
}

class PostalContactInfo {
  constructor(private readonly value: string) {}
}
```

Once you have these, things that take a `string` will not accept `Name` or `EmailContactInfo` or `PostalContactInfo`, and things that take a `Name` will not accept an `EmailContactInfo`, and so on. This works specifically because we used a `private` type; if it were public the types would be compatible because they have the same structure. If we *didn’t* do this, and just used strings, we could easily intermix these on accident. (For an alternative approach, see [<b>Appendix: “Tagged Unions” in TypeScript</b>](#appendix-tagged-unions-in-typescript) below. You can read Wlaschin’s writeup of how to do this in F^♯^ [here][wlaschin-background].)

[wlaschin-background]: https://fsharpforfunandprofit.com/posts/designing-with-types-single-case-dus/

Wlaschin’s example goes one step further with `EmailContactInfo` and makes it so that you can’t actually create an invalid email—to handle things like `hello@potato`, which is not a valid email address. We can do that in TypeScript with a *private constructor* and some *static helper methods*:

```ts
class EmailContactInfo {
  private constructor(private readonly value: string) {}

  static create(value: string): EmailContactInfo | undefined {
    return isValid(value)
      ? new EmailContactInfo(value)
      : undefined;
  }

  static isValid(value: string): boolean {
    return /^\S+@\S+\.\S+$/.test(value);
  }
}
```

:::note

That’s not an *especially* robust validation of emails, but it’s sufficient for the rest of this example.

:::

## Making illegal states unrepresentable

Now we’re ready to actually dig into the meat of the example. Wlaschin suggests a fairly realistic example (I’ve seen constraints just like this pretty often):

> Now let’s say that we have the following simple business rule: <i>“A contact must have an email or a postal address”</i>. Does our type conform to this rule?
> 
> The answer is no. The business rule implies that a contact might have an email address but no postal address, or vice versa. But as it stands, our type requires that a contact must always have both pieces of information.
> 
> The answer seems obvious – make the addresses optional…

In TypeScript, making the two fields optional would look like this (note the `?` after the two optional fields):

```ts
type Contact = {
  name: Name;
  emailContactInfo?: EmailContactInfo;
  postalContactInfo?: PostalContactInfo;
};
```

But, as Wlaschin notes, this would let us have a `Contact` which doesn’t have *either* kind of contact info. This violates the business rule, where a contact must have one or both. It’s very tempting to reach for optional types in TypeScript, both because the syntax is super convenient, especially now that [optional chaining] and [nullish coalescing] are available, and because *so* many things in JavaScript <abbr title="application programming interface">API</abbr>s *are* optional. However, we can do better!

[optional chaining]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/undefined
[nullish coalescing]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Nullish_coalescing_operator

Wlaschin points out:

> If we think about the business rule carefully, we realize that there are three possibilities:
> 
> - A contact only has an email address
> - A contact only has a postal address
> - A contact has both a email address and a postal address> 

Like F^♯^—though with very different syntax and details—TypeScript supports modeling this clearly and accurately, using [union types]. Wlaschin’s original writeup uses a tuple to represent the variant where the contact has *both* email address and postal address. TypeScript *also* [has tuples][ts-tuples], though they’re a bit less common in TypeScript than in F^♯^, because TypeScript has to overload the meaning of array types, whereas they are first-class types in F^♯^.

[union types]: https://www.typescriptlang.org/docs/handbook/unions-and-intersections.html#union-types
[ts-tuples]: https://www.typescriptlang.org/docs/handbook/basic-types.html

```ts
type ContactInfo =
  | EmailContactInfo
  | PostalContactInfo
  | [EmailContactInfo, PostalContactInfo]
  ;
```

We could also use a lightweight object type here, but as we’ll see in a moment, the tuple is actually easier to work with for this specific case.

Then the original `Contact` type becomes:

```ts
class Contact {
  constructor(
    public readonly name: Name,
    public readonly contactInfo: ContactInfo
  ) {}
};
```

At this point, we have actually written a type that achieves the goal of making illegal states unrepresentable. What does it look like to use it in practice, though?

### Constructing a `ContactInfo`

We’ll start by showing how we construct a `Contact` when we have a name and an email. Remembering that we have the possibility of `EmailContactInfo.create` returning `undefined`, here’s how we’d handle that:

```ts
class Contact {
  // the existing definition

  static fromEmail(
    name: string,
    email: string
  ): Contact | undefined {
    let maybeEmail = EmailContactInfo.create(email);
    if (maybeEmail) {
      let contactName = new Name(name);
      return new Contact(contactName, maybeEmail);
    } else {
      return undefined;
    }
  }

  static fromPostalAddress(
    name: string,
    address: string
  ): Contact {
    let contactName = new Name(name);
    let postalInfo = new PostalContactInfo(address);
    return new Contact(contactName, postalInfo);
  }
  
  static fromEmailAndPostal(
    name: string,
    email: string,
    address: string
  ): Contact | undefined {
    let maybeEmail = EmailContactInfo.create(email);
    if (maybeEmail) {
      let contactName = new Name(name);
      let postalInfo = new PostalContactInfo(address);
      return new Contact(contactName, [email, postalInfo]);
    } else {
      return undefined;
    }
  }
}
```

Now we have a variety of tools we can use to build `Contact`s. If we already have a valid `EmailContactInfo`, and/or a `PostalContactInfo`, we can use the constructor directly:

```ts
let fromEmailDirectly = new Contact(name, email);
let fromPostalDirectly = new Contact(name, postal);
let fromBothDirectly = new Contact(name, [email, postal]);
```

We can also do it using the static helpers, which can be convenient when we haven’t already validated the data elsewhere:

```ts
let fromInvalidEmail =
  Contact.fromEmail("Chris", "hello@banana");
let fromValidEmail =
  Contact.fromEmail("Chris", "hello@chriskrycho.com");
let fromPostal =
  Contact.fromPostal("Chris", "Colorado");
let fromBoth =
  Contact.fromEmailAndPostal("Chris", "hello@chriskrycho.com", "Colorado");
```

If we had similar validation for the postal address—for example, because we were making it behave like an actual address!—we would have another layer of validation in the variants that use `PostalContactInfo`. (That’s exactly what Wlaschin’s design does; I’ve left it aside here to keep things moving!)

### Updating a `ContactInfo`

Having made this change, we’re now in a position to see what it looks like to work with this data when it already exists. Again, the type system helps us out here, to make sure we *continue* to have valid data. As Wlaschin notes:

> Now… we have no choice but to handle all three possible cases:
> 
> - If a contact previously only had an email address, it now has both an email address and a postal address, so return a contact using the EmailAndPost case.
> - If a contact previously only had a postal address, return a contact using the PostOnly case, replacing the existing address.
> - If a contact previously had both an email address and a postal address, return a contact with using the EmailAndPost case, replacing the existing address.

We can add a method to the `Contact` type which does this correctly.

:::note

There are two ways we could make this method work: it could change the item it’s working with, or it could return a new copy of the item instead. In this example, I’m treating classes not as bundles of self-contained mutable state, but as lightweight records, so I’m going to return a fresh copy. Maybe [someday][proposal-record-tuple] JavaScript will actually get native records (and tuples)!

:::

[proposal-record-tuple]: https://github.com/tc39/proposal-record-tuple

```ts
class Contact {
  // everything we’ve seen already

  updatePostalAddress(newAddress: string): Contact {
    let postalInfo = new PostalContactInfo(newAddress);

    let newContactInfo: ContactInfo;
    if (this.contactInfo instanceof EmailContactInfo) {
      // email only -> email and postal
      return new Contact(this.name, [this.contactInfo, postalInfo]);
    } else if (this.contactInfo instanceof PostalContactInfo) {
      // ignore existing address
      return new Contact(this.name, postalInfo);
    } else {
      // ignore existing address
      let [email, _] = this.contactInfo;
      newContactInfo = [email, postalInfo];
    }
  }
}
```

Notice here that we don’t have to worry about validating the email again: the fact that it’s present as an `EmailContactInfo` on our type means it is *already* valid. This is one of the benefits of using richer types to represent these kinds of things!

We can use this code like this, given an existing `Contact`:

```ts
let oldAddress = "North Carolina";
let contact = Contact.fromPostal("Chris", oldAddress);

let newAddress = "Colorado";
let updatedContact = contact.updatePostalAddress("Colorado");
```

<aside>

I kept these names because they make it closer to the original example, but I would actually probably use a name like `withNewPostalAddress` if I were writing the code from scratch. It reads very nicely that way:

```ts
let updatedContact = contact.withNewPostalAddress("Colorado");
```

</aside>

## Why bother to make these complicated types?

Here, I’ll just quote Wlaschin in full, but substituting the TypeScript example for the F^♯^ he gives:

> At this point, you might be saying that we have made things unnecessarily complicated. I would answer with these points:
> 
> First, the business logic is complicated. There is no easy way to avoid it. If your code is not this complicated, you are not handling all the cases properly.
> 
> Second, if the logic is represented by types, it is automatically self documenting. You can look at the union cases below and immediate see what the business rule is. You do not have to spend any time trying to analyze any other code.
> 
> ```ts
> type ContactInfo = 
>   | EmailContactInfo
>   | PostalContactInfo
>   | [EmailContactInfo, PostalContactInfo]
>   ;
> ```
> 
> Finally, if the logic is represented by a type, any changes to the business rules will immediately create breaking changes, which is a generally a good thing.

## Conclusion

I’ve been using this approach for the last few years, and it’s really, *really* helpful! For a really helpful introduction to this way of thinking and working with types, I heartily recommend the following materials:

- [Effective ML Revisited](https://blog.janestreet.com/effective-ml-revisited/), Yaron Minksy—the blog post that really started the modern popularization of a lot of these ideas
- [Designing with Types – F^♯^ for Fun and Profit](https://fsharpforfunandprofit.com/series/designing-with-types.html)—the series from which this post was adapted
- [Making Impossible States Impossible](https://www.youtube.com/watch?v=IcgmSRJHu_8), Richard Feldman, Elm Conf 2016
- [<cite>Domain Modeling Made Functional</cite>](https://fsharpforfunandprofit.com/books/), by Scott Wlaschin
- [Parse, Don’t Validate](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/), Alexis King

You may notice that all of these are in languages like F^♯^, Elm, and OCaml. That’s for good reason: for a long time, that family of languages was the only place that had the type systems to make this doable. Over the last decade, quite a few more have emerged, including Rust, Swift—and TypeScript!

## Appendix: “Tagged Unions” in TypeScript

In this post, I just reached for bare types and used the `instanceof` operator. This works well with TypeScript’s type system, and fits fairly naturally with JavaScript, but there’s an alternative we could use that is also worth calling out: building our own “tagged unions” in TypeScript.

In F^♯^, Elm, Rust, etc., the tagged union types (which are called “unions” in F^♯^, “custom types” in Elm, and “enums” in Rust) have a *built-in* tag, created by the compiler. In TypeScript, there is nothing of the sort—because there is nothing of the sort in JavaScript. However, TypeScript *does* understand the idea of unions (as we saw above). It *also* has the ability to treat literals as different types, like this:

```ts
type MyName = 'Chris';
let me: MyName = 'John'; // TYPE ERROR
```

We could even use the literal as a type inline:

```ts
let me: 'Chris' = 'John'; // TYPE ERROR
```

We can turn this plus unions into a *tagged* union, by using a literal as the tag on a type, and then TypeScript will understand it. Earlier, we used the fact that `private` fields make TypeScript treat different classes as distinct, despite the structural type system. Here, we could make them public but readonly instead (though hiding the details might be good for *other* reasons), and use a tag to distinguish them.

Two notes on this example:

- I’ve changed the wrapped values from being named `value` to be the name of the thing wrapped, `name`, `email`, and `address`. This will make the example below clearer.
- The `as const` syntax here says to treat the type as a literal.

```ts
class Name {
  readonly type = 'Name';

  constructor(public readonly name: string) {}
}

class EmailContactInfo {
  readonly type = 'EmailContactInfo';

  private constructor(public readonly email: Email) {}

  static create(value: string): EmailContactInfo | undefined {
    // same implementation as before
  }

  static isValid(value: string): boolean {
    // same implementation as before
  }
}

class PostalContactInfo {
  readonly type = 'PostalContactInfo';

  constructor(public readonly address: string) {}
}
```

Here, it might also make sense to create a custom tagged type for the `EmailAndPostalInfo` variant:

```ts
class EmailAndPostal {
  readonly type = 'EmailAndPostal';
  
  constructor(
    public readonly data: [EmailContactInfo, PostalContactInfo]
  ) {}
}
```

Then we can define the `ContactInfo` type like this:

```ts
type ContactInfo =
  | EmailContactInfo
  | PostalContactInfo
  | EmailAndPostal
  ;
```

When using `ContactInfo`, we can now use the `type` tag to figure out which one we’re working with:

```ts
class Contact {
  // the same as in the main example

  describe(): string {
    switch (this.info.type) {
      case 'EmailContactInfo':
        return `contact ${this.name} at ${this.info.email}`;

      case 'PostalContactInfo':
        return `${this.name} lives in ${this.info.address}`;

      case 'EmailAndPostal':
        let [email, postal] = this.info.data;
        return `${this.name} lives in ${postal}; contact at ${email}`;

      default:
        assertNever(this.info);
    }
  }
}
```

Two things to notice:

-   The `assertNever` type in the `default` branch comes from the TypeScript handbook; because it takes `never` as its argument, it makes sure that we have exhaustively covered every case represented by `this.info.type` here. If we added another case to the `ContactInfo` union, this would cause a type error!

-   We only have access to `this.info.email` in the `'EmailContactInfo'` branch. Trying to access it in the `case 'PostalContactInfo':` branch will produce a type error:

     > Property 'email' does not exist on type 'PostalContactInfo'

The tradeoffs with this approach are *different* than they are with the `instanceof` approach. We have to carry around a bit more information, and while that’s not incredibly expensive, it *could* add up over time in a large app. It also requires that people working on the app actually understand these and what they’re for, and people are more likely to already know `instanceof` and how to use it. However, we get to use the `switch` approach, which can clarify things a *lot* over a chain of `if`/`else if`/`else` statements. In particular, I think it is much easier to see that we are just covering each case of the type that `info` might be in this code than in the `instanceof` code above!
