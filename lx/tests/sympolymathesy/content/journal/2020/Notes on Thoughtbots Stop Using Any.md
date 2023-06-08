---
title: Notes on Thoughtbot’s “Stop Using 'any'”
subtitle: A couple tweaks and improvements to a good post!
summary: >
  Eliminating some needless type-casts and using some modern JS and TS idioms to improve on the code from a good post from Thoughtbot about TypeScript’s `any` type.
qualifiers:
  audience: >
    Developers working with (or just curious about) TypeScript.

date: 2020-10-31T18:42:00-0600
updated: 2020-11-21T17:17:00-0600

tags:
  - TypeScript
  - JavaScript
  - software development
  - type theory

---

Scrolling through my RSS feed this evening, I came across [this excellent post](https://thoughtbot.com/blog/typescript-stop-using-any-there-s-a-type-for-that) on the Thoughtbot blog. I strongly agree with nearly everything in it: when we land TypeScript more broadly at LinkedIn, we will probably lint against *ever* using `any`.

However, a couple things caught my attention as points for improvement in the code samples shared there, so I figured I’d share them more broadly so everyone can benefit from them.

Under the heading **I really don’t know what it is**, the authors offer this example of using `unknown` instead of `any`:

```ts
type ParsedType = {
  id: number
}

const parseApiResponse(
  response: Record<string, unknown>
): ParsedType => {
  const convertedResponse = (response as ParsedType)

  // without doing the type cast we would
  // get a type error here
  if (convertedResponse.id >= 0) {
    return convertedResponse
  } else {
    throw Error.new("Invalid response"
  }
}
```

This definitely *is* preferable to using `any`… but what if we could eliminate the (*also* totally unsafe!) cast as well? Turns out we can! TypeScript’s notion of [type narrowing](https://www.typescriptlang.org/docs/handbook/2/narrowing.html) gives us the ability to check for this *safely*:

```ts
type ParsedType = {
  id: number
}

const parseApiResponse = (
  response: Record<string, unknown>
): ParsedType => {
  if (typeof response.id === 'number' && response.id >= 0) {
    let parsed = { id: response.id };
    return parsed;
  } else {
    throw new Error("Invalid response")
  }
}
```

If the response were more detailed than this, and therefore required more validation, we could extend the checks in that `if` statement to cover more ground. What’s more, if we want to avoid the extra object allocation, we can do that by reaching for [some utility types](https://v5.chriskrycho.com/journal/writing-robust-typescript-libraries/) which let us be as robust as we like. And we could go further in that case and parse even a response typed as `unknown`:

```ts
type ParsedType = {
  id: number
}

function isValid(response: unknown): response is ParsedType {
  return (
    isObject(response) &&
    has('id', response) &&
    typeof response.id === 'number' &&
    response.id >= 0
  );
}

const parseApiResponse = (response: unknown): ParsedType => {
  if (isValid(response)) {
    return response;
  } else {
    throw new Error("Invalid response")
  }
}
```

Now we have *no* type-casts, because our runtime code—which is required if this is actually going to be safe in either JavaScript or TypeScript!—proves to the compiler that this is actually a `ParsedType`. This means that we can use `parseApiResponse` with a call like `fetch` and its `data.json()`, which returns a `Promise<any>`.[^1]

Second, in their section **I have to write a lot of code when I add types, `any` is less work**, they offer this code sample as justification of the (entirely correct, in my view!) claim that “if we are writing code without types, we will likely add defensive code to make sure arguments and variables have the correct shape for the program to perform as intended:”

```ts
const fullName = (user: any) => {
  if (user?.firstName && user?.lastName) {
    return `${user.lastName}, ${user.firstName}`
  }

  return user?.firstName || ""
}
```

As a replacement, they suggest this:

```ts
interface User {
  firstName: string
  lastName?: string
}

const fullName = ({ firstName, lastName }: User) => {
  if (lastName === undefined) {
    return firstName
  }

  return `${lastName}, ${firstName}`;
}
```

This is a great improvement![^2] However, I feel obliged to note that instead of the or `||` operator in their `any` example, it’s worth reaching for the [nullish coalescing `??` operator][nc], which treats `0`, `""`, etc. differently than `undefined` and `null`! Now, the second example is still better code, and we don’t even *need* it in that case, but nullish coalescing is a great tool to have in your toolbox. You can imagine that in the case where the `User` argument here *was* optional—perhaps as part of some other set of arguments, or because you were dealing with untrusted data, that it would still be useful:

```ts
interface User {
  firstName: string
  lastName?: string
}

const fullName = (user?: User) => {
  const first = user?.firstName ?? "";
  const last = user?.lastName ?? "";
  return `${first}, ${last}`;
}
```

This is contrived, to be sure, but it shows how useful [optional chaining][oc] and nullish coalescing can be even when you *do* have well-typed code.

In sum: that post from Thoughtbot had great recommendations, but with a couple tweaks we can make it even stronger!

[nc]: http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Nullish_Coalescing_Operator
[oc]: http://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Optional_chaining

*[RSS]: really simple syndication*

[^1]: Shameless plug: I would not throw an error here. Instead, I’d reach for a `Result` type, like the one in [the library a friend and I wrote a couple years ago](https://github.com/true-myth/true-myth "True Myth"). That way I would be able to have type-safe error handling, as well! That would end up looking something like this:

    ```ts
    import { Result } from "true-myth";

    class ApiError extends Error {
      constructor(readonly response: unknown) {
        super("Invalid response");
      }
      
      static from(response: unknown): ApiError {
        return new ApiError(response);
      }
    }

    const parseApiResponse =
      (response: unknown): Result<ParsedType, ApiError> =>
        isValid(response)
          ? Result.ok(response)
          : Result.err(ApiError.from(response));
    ```
    
    Now we have a well-typed error, which we can deal with as a value—no need for another `try`/`catch` block, and in fact we know *some* details about the kind of error we have!

[^2]: Now, I would *absolutely* write this as a ternary and a single-expression function body instead:

    ```ts
    interface User {
      firstName: string
      lastName?: string
    }

    const fullName = ({ firstName, lastName }: User) =>
      lastName ? `${lastName}, ${firstName}` : firstName;
    ```

    …but for our purposes in this post that doesn’t much matter. 😅