---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--AKg3LOMU--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://dev-to-uploads.s3.amazonaws.com/i/ugtvxwhh7gxdh36kxlfl.jpg
date: 2020-02-11T12:00:00.000Z
title: Get Started Writing Scheme
description: A brief introduction to the Scheme language and ecosystem.
tags:
  - beginners
  - scheme
  - functional
  - tutorial
---

TL;DR: Download [Racket](https://racket-lang.org/), run `plt-r6rs my-file.scm`. Bookmark [this book](https://www.scheme.com/tspl4/) and check it when you have questions.

[Scheme](https://en.wikipedia.org/wiki/Scheme_%28programming_language%29) is an old programming language, hailing from the mid-seventies. Its hallmark is minimalism - there really isn't that much _language_ here. That should not be confused with _expressive power_ - you can use Scheme to code up anything you currently write in your favourite language.

However, this minimalism makes it specifically suited towards academic study. The language gets out of the way and lets you focus on the actual logic.

Some people who took the traditional CS college route were probably exposed to this tool as part of your curriculum. Not all curricula use it, though - mine doesn't - and self-learners might not stumble across it. I'm of the opinion that you're missing out on a perfectly suited tool for studying algorithms and programming languages. Scheme is a great language for talking about computation itself, almost as a formal notation. There's no noise there to distract you - it's just you and your algorithms.

Here's a common example. This is how one (specifically me) might code up [Euclid's algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm) for finding the greatest common divisor of two numbers in [C](https://en.wikipedia.org/wiki/C_%28programming_language%29):

```c
// euclid.c

#include <stdio.h>

/**Calculate the greatest common divisor of positive integers m and n.
 * Returns -1 on error
 */
int gcd(int,int);

int main(void)
{
    int m = 119, n = 544, res;
    res = gcd(m,n);
    printf("%d", res);
    return 0;
}

int gcd(int dividend, int divisor)
{
    int m = dividend, n = divisor;
    // Out of bounds
    if (n <= 0 || m <= 0)
        return -1;
    for (;;)
    {
        // 1. Let `r` be the remainder of diving `m / n`.
        int r = m % n;
        // 2. If `r == 0`, complete.  Return `n`.
        if (r == 0)
            return n;
        // 3. `m <- n`, `n <- r`, goto 1.
        m = n;
        n = r;
    }
}
```

C is a good language for algorithmic study for a similar reason - it's a small language. There is not a ton of syntactic noise, and even less implicit magic - the programmer bears the brunt of the burden when it comes to telling the computer what you need. This limitation isn't actually a limitation at all - you're completely in the driver's seat, and when studying and optimizing algorithms to build understanding, that's exactly what you want.

However, it's still a little bit noisy. What's `<stdio.h>`? What's `"%d"`? Why does it say `int` everywhere?

Here's an implementation of the same algorithm in Scheme:

```scheme
; euclid.scm

(import (rnrs))

(define (gcd m n)
    (if (and (>= m 0) (>= n 0))
        (cond [(= n 0) m]
              [else (gcd n (mod m n))])
        (- 1)))

(display (gcd 119 544))
```

That's a lot more concise! You get to code your syntax tree directly in tree form.

To define a binding, you use `define`:

```scheme
(define my-name "Ben")
(display my-name)
```

This also works for functions, and the above snippet is using a shorthand for this. Every function in Scheme is actually an anonymous lambda that you are welcome to create a binding for, and because that's such a common operation we can use this syntactic sugar. Without it, this program looks like this:

```scheme
(define gcd
    (lambda (m n)
        (if (and (>= m 0) (>= n 0))
            (cond [(= n 0) m]
                  [else (gcd n (mod m n))])
            (- 1))))
```

The function itself is an anonymous lambda for which we define a binding `gcd`. When you use the shorter `(define (gcd m n) (...))`, this is actually what happens.

This regularity extends to all parts of the language. A function call is a list wrapped in parens with the function itself in first position:

```scheme
(= n 0) ; n == 0
(mod m n) ; n % m
```

Instead of `condA && condB`, we use `(and condA condB)`. This nearly entirely removes ambiguity - just follow the paren nesting and you will be able to trace the logic. The `if` expression looks like this:

```scheme
(if predicate onTrue onFalse)
```

You can replace all three arguments with the actual forms you need. If your code starts getting too convoluted, you can either `define` blocks outside of your function, or use [`let`](https://www.scheme.com/tspl4/start.html#./start:h4) to create a local binding. The `cond` statement is a shorthand for an `if` with multiple arms, like a `switch`.

One gotcha is that you can't even write a negative number directly: `-1` won't work in the failure arm. With only one argument, `-` means "negate" - so `(- 3 2)` subtracts, yielding `1`, and `(- 1)` negates, yielding `-1`.

Notably, this version is written in recursive fashion, whereas our C was imperative. Scheme is standardized to support proper [tail recursion](https://stackoverflow.com/questions/33923/what-is-tail-recursion), so while this function is syntactically recursive, because the recursion occurs in tail position it will actually be executed more like an imperative loop, reusing the stack frame and swapping the operands.

You are able to write any imperative algorithm you come across in a recursive fashion, though they may not always be similarly convenient. If it's not clear immediately how you might do so for an algorithm you're working with, you might want to break out some Scheme and try it!

The standard I'm using is called `R6RS`, or more generally `RNRS` to refer to version-agnostic Scheme. Much like C++ or ECMAScript, Scheme exists as a standard only, and many [competing implementations](http://www.r6rs.org/implementations.html) exist. Somewhat frustratingly these implementations actually diverge somewhat significantly from each other, but the core language will be consistent - that's the whole point of a standard! The R6RS standard can be found [here](http://www.r6rs.org/).

There's actually an R7RS scheme now, too, but don't worry too much about it unless you start getting serious about Scheme work. There's apparently some contention in the community about minimalism and values and whatever - you know, politics. I'm not here for politics, I'm here for algorithms. R6RS is completely fine.

I've found the easiest way to start using Scheme is via [Racket](https://racket-lang.org/). Racket is a system that is [much more powerful](https://felleisen.org/matthias/manifesto/) than just RNRS Scheme, but supports a feature called `#lang` directives that let you restrict Racket to specific standards like this. You can place this `#!r6rs` directive directly in your file and use the standard Racket build tooling, but Racket also ships with a CLI tool called `plt-r6rs`. You can invoke this directly on a scheme file, no `#lang` required:

```
$ plt-r6rs euclid.scm
17
```

More information about using R6RS via Racket can be found [here](https://docs.racket-lang.org/r6rs/index.html). Here's the [Scheme book](https://www.scheme.com/tspl4/) again, and the [R6RS standard](http://www.r6rs.org/) again. Those links should get you up and running for your own experimentation. If you're itching for the next level of Scheme enlightenment, RUN (do not walk) directly to [The Little Schemer](https://www.ccs.neu.edu/home/matthias/BTLS/) and make yourself a sandwich.

Have fun!

_Photo by Chinh Le Duc on Unsplash_
