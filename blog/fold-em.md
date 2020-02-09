---
cover_image: http://www.wopc.co.uk/images/countries/thailand/vegas-gold-111-large.jpg
edited: 2018-12-03T12:00:00.000Z
title: Know When to Fold 'Em
published: true
description: Some standard functions expressed as folds
tags: beginners, haskell, functional
---
Do you have a favorite function?  I do.  It's really a family of functions, but today I'd like to talk about *folds*.

I'm going to be illustrating with Haskell, but this concept is not Haskell-specific nor should you need to be terribly familiar with Haskell to read the examples.  In the first section I unpack what a fold even is and in the second we re-implement a few library functions in terms of a fold.

## What's A Fold?

Folds are not an uncommon concept in mainstream languages - if you're good and comfy, skip this section.  If not, though, it will help to know how they work.

In a purely functional paradigm the way we take a collection of values and make sure we do something with every member of the collection is to consume the collection recursively.  That is, we're going to pass our whole collection into some sort of function which is going to do some sort of processing.  At the end of the function it's going to call itself again with a smaller part of the list as the argument - the part we haven't processed through the function yet.  It will do this again and again, calling itself with smaller and smaller parts of the collection until the whole thing is processed.  Easy peasy.  A `fold` is a specific type of recursive function that takes in a data structure, a collection of some type, and a function to use for each member - specifically the first element of the collection on each iteration.  It eventually yields just one single value after the list has been fully drained.

Types are one thing that I find easier to talk about in Haskell than English.  Here's the type signature for `foldr`:

```haskell
foldr :: (a -> r -> r) -> r -> [a] -> r
```

It's fine if you stared blankly at this, that's usually step one of unraveling a type signature.  They all work the same way though, so we can walk our way through.  We know this is a function that takes three arguments because everything evaluates to one value in the end - so the compiler will expect three bits of information while processing this to get to that final `r`.  Parentheses in type signatures work as you'd expect - that first part is grouped, signifying it's a single argument with the type `a -> r -> r` instead of three separate arguments.  The second unknown type is conventionally shown with a `b` - I'm using `r` to indicate it's our return type.  If you went to look this up online, you'll probably see a `b` instead.  It doesn't matter what type, it could be anything.  This second type placeholder could even be another `a` and often is, but it doesn't *have* to be for the function to be correct so we use a different letter.

The first thing is our processing function.  This itself is a function which takes two arguments.  It takes in a single element of our `[a]`, or a list of `a` types, and some value of the type that we're returning and returns a new value with our expected return type.  The next argument is a single instance of that return type - the "destination" so to speak.  We know we're going to be getting a single value from this fold, and we have a function that takes a cell and our current running result and gives us back the new result, so we can drop that cell from the next run through the recursion.  On the first run through, though, we need somewhere to deposit the result of the computation, so `foldr` asks for a container as the second argument of type `r` to apply the result to.  This initial value we pass in is going to be transformed every run through the function and is eventually what gets returned.

If this all was too abstract, here's a simple example that might look more familiar - let's fold some basic addition into a collection:

```haskell
nums :: [Int]
nums = [1, 2, 3, 4, 5]

addEmUp ns :: [a] -> r
addEmUp ns = foldr (+) 0 ns
```

That's a lot less noisy.  In this example calling `addEmUp nums` will yield `15 :: Int`.  First, I defined a `[Int]` - a list of `Int`s - called `nums`.  Then I created a function `addEmUp` which is really an alias for a specific `fold` - notice how it doesn't do anything other than specify which arguments to use with the fold.  That's why the type signature for `addEmUp` is a lot simpler - it only takes the `[a]` collection, in this case `nums`.  So our `a` is `Int`.  The first argument, the processor, is `(+)` - the addition operator.  Operators are functions and this one takes in two values and produces a third.  Let's compare to our expected type: `a -> r -> r`.  In this case `a` is `Int` and we want an `Int` at the end, so we can substitute that type in for `r` too.  If you add an `Int` to an `Int`, lo and behold, an `Int` will pop out.  So our processor, addition, has type `Int -> Int -> Int`, which fits!  It's totally fine if `a` and `r` or any two unspecified types are the same, we just note that they don't *have* to be.

Our second argument was just a `0` - an `Int`.  We've just decided that's a perfectly fine `r` type so the second argument makes sense as an initializer for our return type.  That just leaves us with `[a]`.  Thankfully we've left that part of the type intact and are passing it in as the argument to `addEmUp`!  For this simple example, the fully qualified type of this `foldr` reads: `(Int -> Int -> Int) -> Int -> [Int] -> Int`.  Just a bunch of `Int`s.

When Haskell goes to evaluate this it will start with the full collection.  When we get to the first run through the processor will grab the first cell and then look for our accumulated result.  We haven't done anything yet so it's just `0` - we told it that in the second argument.  The first value is `1`.  Our accumulator added to our base value is `1`.  Then, we recur!  Only this time we've already processed the one, so we're calling this same function again but only on the rest of the collection, and using our newly minted `1` as the accumulator instead of the base value `0`:

```haskell
foldr (+) 0 [1, 2, 3, 4, 5]
foldr (+) 1 [2, 3, 4, 5]
```

See what happened there?  We processed the one and dropped it so our collection got shorter and we have a running total.  Expanding:

```haskell
  foldr (+) 3 [3, 4, 5]
= foldr (+) 6 [4, 5]
= foldr (+) 10 [5]
= foldr (+) 15 []
= 15
```

When a recursive function tries to recur on an empty list it knows it's done and returns the final value - in this case `15`.  We've managed to iterate without looping!  Instead we folded an operation in: `[1 + 2 + 3 + 4 + 5]`.  It's almost like we replaced the commas with our operator a step at a time from right to left.  In that way, we were able to reuse the same exact function over and over again while only changing what we pass in based on the output of the previous run.  Recursion, yo.

If this sounds outrageously inefficient, calling loads and loads of functions all the time with very similar values, well, it is.  To mitigate that overhead, Haskell performs something called "[tail-call](https://en.wikipedia.org/wiki/Tail_call) optimization" which I won't detail here but essentially means that instead of allocating a new stack frame for each successive call it's able to reuse the same stack frame and substitute the new vals and then just jump execution back up, `GOTO`-style, provided the function recurs in "tail position", which means it's the last part of the function to execute.  If you're not familiar with stack frames we're getting way beyond the scope of this post - it's not required knowledge here but interesting in general and important to understand if you'd like to use a functional language in anger.  In toy programs, the elegant functional solutions are generally fine, but as your apps scale it can start to cause problems, and languages which allow a more hybrid style generally recommend you fall back to more imperative patterns at that point.  A good old `for` loop will as a rule of thumb perform better on large amounts of data than a one-liner using a `forEach` or something similar - sometimes by orders of magnitude.  In Haskell dealing with these performance problems involves other sorts of patterns as well, as there's no `for` loop to speak of.  I recommend you do some poking around!

This example could have been rewritten: `addEmUp = foldr (+) 0` - if the argument is the final term in the definition and the argument list it can be dropped.  This process is known as an [eta-reduction](https://en.wikipedia.org/wiki/Lambda_calculus#%CE%B7-conversion) in the lambda calculus lingo.  The compiler instead sees this definition as a curried function expecting one more value.  If it gets called with that value it will fully evaluate the expression.

## All The World's a Fold

To illustrate how useful this function is let's rewrite some other common standard functions in terms of a fold.

Now, `foldl'` may be appropriate instead here - I'm not going to get bogged down in when to prefer one over the other.  The [Haskell Wiki](https://wiki.haskell.org/Foldr_Foldl_Foldl') has more information, and for consistency's sake these will all be right folds.

If you have `ghc` installed you can follow along in `ghci`, or if you prefer put these in a file:

```haskell
-- stdFolds.hs
module StdFolds where

-- the functions!
```

Then load this module at the REPL with `:l stdFolds.hs`.

### Or

We'll start with `Or`.  We want a function that can "or" a list - that is, return `True` if any of its members are `True` and otherwise `False`.  The type of our function is easy:

```haskell
myOr :: [Bool] -> Bool
```

We pass in a list of `Bool` and just want one back.  This means our accumulator parameter should just be a `Bool`.  On each element of the list, we want to compare against this accumulator and replace it with the appropriate value - the built-in `(||)` operator will do just that!  We'll only retain a `False` is both sides are `False`, and otherwise the accumulator will end up with `True` - which will *always* reutrn `True` when checked against any other boolean:

```haskell
myOr :: [Bool] -> Bool
myOr = foldr (||) False
```

We're using eta reduction, like above - our data structure itself is dropped from the definition.  Example output:

```
*StdFolds> let a = [True, False, True]
*StdFolds> myOr a
True
*StdFolds> let b = [False, False, False]
*StdFolds> myOr b
False
```

### Any

Now lets write a function that checks if any `a` in a given `[a]` satisfies a predicate `a -> Bool`:

```haskell
myAny :: (a -> Bool) -> [a] -> Bool
```

We're actually performing nearly the same function as with `myOr` - the only difference is that instead of just using the element of our collection, we're passing it through a predicate.  We can compose our predicate with the `(||)` operator:

```haskell
myAny :: (a -> Bool) -> [a] -> Bool
myAny f = foldr ((||) . f) False
```

Example output:

```haskell
*StdFolds> let a = [0, 1, 2]
*StdFolds> myAny (== 3) a
False
*StdFolds> myAny (== 1) a
True
*StdFolds>
```

We can actually do better, though.  We've already dropped the collection with eta reduction, but using the super handy `flip` function we can re-arrange so our `f` is at the end too:

```haskell
myAny :: (a -> Bool) -> [a] -> Bool
myAny f = flip foldr False ((||) . f)
```

Now we can drop it entirely by simply using function composition:

```haskell
myAny :: (a -> Bool) -> [a] -> Bool
myAny = flip foldr False . ((||) .)
```

This version is identical to our first attempt, just de-cluttered.

### Elem

The `elem` function is just a special case of `any`, with a specific predicate.  Because we've re-arranged `myAny` to take the function in the final position we can just use that:

```haskell
myElem' :: Eq a => a -> [a] -> Bool
myElem' = myAny.(==)
```

We've eta-reduced out the parameter, but we can still apply the typeclass constraint of `Eq` to it.

Example output:

```
*StdFolds> let a = [0, 1, 2]
*StdFolds> myElem 3 a
False
*StdFolds> myElem 1 a
True
```

Without using `myAny` we just use the same function body:

```haskell
myElem :: Eq a => a -> [a] -> Bool
myElem = flip foldr False . ((||) .) . (==)
```

### Map

We're going to use a similar pattern to build our own `map`.  This has a slightly different signature - given a function from `a` to `b` we want to get a `[a]` from a `[b]`:

```haskell
myMap :: (a -> b) -> [a] -> [b]
```

The difference from `myAny` is that instead of just accumulating into a `Boolean`, we want to construct a new list.  Instead of comparing to the accumulator with `(||)` we can use the list construction operator `(:)`:

```haskell
myMap :: (a -> b) -> [a] -> [b]
myMap = flip foldr [] . ((:) .)
```

Otherwise, it's the same thing!  Example output:

```haskell
*StdFolds> let a = [0, 1, 2]
*StdFolds> let f = (* 2)
*StdFolds> myMap f a
[0,2,4]
*StdFolds>
```

 Here's a [gist](https://gist.github.com/deciduously/8b2babeb07782c046aa0dab9f1392634).
