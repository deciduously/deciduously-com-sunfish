---
cover_image: https://www.gkbmachines.com/wp-content/uploads/2016/12/CB6a.jpg
edited: 2019-02-05T12:00:00.000Z
title:  Parser Combinators are Easy
published: true
description: A quick Parser Combinator tutorial
tags: beginners, functional, javascript
---
Let's say we've been sent some brand new points.  However, the Point Guru is having a burst of 'creativity' today and has devised a crazy transmission string:

```javascript
const input = '.:{([2 3]][[6 2]][[1 2])][([1 4]][[2 1])][([6 9])}:.'
```

This is clearly bonkers and you shouldn't have to put up with it.  Sadly, she's your only connect for points in sets of varying sizes, though, and the points themselves look alright, so you have to roll up your sleeves and get 'em outta there.

I don't know about you, but I (until now!) have always sighed and reached for a regular expression at this point, or started mucking about with string manipluations.  It'll be ugly as hell, but it'll work.   You can pull out each list with capture groups and then either use another regex on the captures or use string splitting and iterators to get what you need.  It likely won't be much fun, and will be completely illegible at a glance at the end (unless regex is really your thing).

BUT WAIT!  There's another way!  And it's even easier than it sounds!
![meme](https://i.imgur.com/PiOsDjV.jpg)
(this is my first ever meme!)

Looking at this string, we immediately see it for what it is - a list of points.  The tricky part is just telling the computer what you mean.  With parser combinators, we can!  Parser combinator libraries allow you to define little tiny parsers that you can compose in order to parse anything at all, from a string like this to a programming language.  Parser combinators can initially look complicated because of phrases like `monadic LL(infinity)` and some complex looking syntax in certain languages, but it's actually incredibly simple, and lots of fun to use.  Each little part is reusable if you keep your parts as small as possible.  This way, we really can sorta just tell JavaScript (or what have you) what we need using units tht make sense to us.

I'm using the [Parsimmon](https://github.com/jneen/parsimmon) library to illustrate, but there are many others for JS and lots of other languages have libraries for this as well.

With Parsimmon, we create a "language" that contains mini parsers, composed of ever smaller parsers.  Here's a very basic example:

```javascript
// index.js
const P = require('Parsimmon')

const CrazyPointParser = P.createLanguage({
    Num: () => P.regexp(/[0-9]+/).map(Number)
})
```

When we first looked at this string, we immediately understood it as ultimately a list of *numbers*.  This is the very basic unit, which we grab with the `regexp` combinator to match 1 or mare characters in the specified range.  It's a much smaller regular expression that the monstrosity alluded to above - readable at a glance.  Each parser gets `map`ped over with how we want the data to be represented - in this case we want this string to be a JavaScript `Number`.

This code can be verified by using the following below:

```javascript
let a = '23'

try {
    console.log(CrazyPointParser.Num.tryParse(a))
} catch (err) {
    console.log('Oops! ' + err)
}
```

Running `node index.js` should output `23` - not `'23'`.  We've parsed a number!  Now we can use this parser in bigger parsers.  The next natural unit to look at is the point - `[8 76]`.  Two numbers separated by a space.

```javascript
const CrazyPointParser = P.createLanguage({
    Num: () => P.regexp(/[0-9]+/).map(Number),
    Point: (r) => P.seq(P.string('['), r.Num, P.string(' '), r.Num, P.string(']')).map(([_open, x, _space, y, _close]) => [x, y])
})
```

The `P.seq()` combinator is used to chain combinators together in a sequence to match.  This time the `r` we pass as an argument is short for `rules` and allows us to refer to the other combinators defined in this language.  Then we just use the `P.string()` combinator to match the separators exactly, and use our `r.Num` combinator to handle recognizing and converting the numbers themselves.  Then over in the map, we are passed an array of each part of the match.  We ignore the brackets and the space returning by the `P.string()` combinators and just return the values our `Num` combinator took care of for us.  Change the test snippet to:

```javascript
let b = '[78 3]'
try {
    console.log(CrazyPointParser.Point.tryParse(b))
} catch (err) {
    console.log('Oops! ' + err)
}
```

Executing this will now return `[ 78, 3 ]`.  Now, these points are further grouped into sets of varying size and (inexplicably) separated by the string `']['`.  We can create a mini parser for just that separator and then leverage the `sepBy()` combinator to handle these sets:

```javascript
const CrazyPointParser = P.createLanguage({
    // ...
    Sep: () => P.string(']['),
    PointSet: (r) => P.seq(P.string('('), r.Point.sepBy(r.Sep), P.string(')')).map(([_open, points, _close]) => points)
})
```

We don't need to include the `map` portion on our `Sep` parser - we just want to return the match as is (it'll be discarded later).  In our `PointSet` parser, `r.Point.seqBy(r.Sep)` will return zero or more `Point`s separated by whtever seaparater we provide as an array, dropping the separators themselvles.  Try it out:

```javascript

let c = '([2 3]][[6 2]][[1 2])'

try {
    console.log(CrazyPointParser.PointSet.tryParse(c))
} catch (err) {
    console.log('Oops! ' + err)
}
```

This will output `[ [ 2, 3 ], [ 6, 2 ], [ 1, 2 ] ]`.  We're almost there!  The full string is just a bunch of `PointSet`s, separated by that same separator with some frilly caps on each end:

```javascript
const CrazyPointParser = P.createLanguage({
    // ...
    PointSetArray: (r) => P.seq(P.string('.:{'), r.PointSet.sepBy(r.Sep), P.string('}:.')).map(([_open, pointSets, _close]) => pointSets)
})
```

And that's it!  Our parser will now successfully parse the whele input string, in only a handful of lines.  Here's the whole snippet:

```javascript
const P = require('Parsimmon')

const input = '.:{([2 3]][[6 2]][[1 2])][([1 4]][[2 1])][([6 9])}:.'

const CrazyPointParser = P.createLanguage({
    Num: () => P.regexp(/[0-9]+/).map(Number),
    Sep: () => P.string(']['),
    Point: (r) => P.seq(P.string('['), r.Num, P.string(' '), r.Num, P.string(']')).map(([_open, x, _space, y, _close]) => [x, y]),
    PointSet: (r) => P.seq(P.string('('), r.Point.sepBy(r.Sep), P.string(')')).map(([_open, points, _close]) => points),
    PointSetArray: (r) => P.seq(P.string('.:{'), r.PointSet.sepBy(r.Sep), P.string('}:.')).map(([_open, pointSets, _close]) => pointSets)
})

try {
    console.log(CrazyPointParser.PointSetArray.tryParse(input))
} catch (err) {
    console.log('Oops! ' + err)
}
```

Output:

```
$ node index.js
[ [ [ 2, 3 ], [ 6, 2 ], [ 1, 2 ] ],
  [ [ 1, 4 ], [ 2, 1 ] ],
  [ [ 6, 9 ] ] ]
```

We can even get fancy - just replace our `Point` combinator with:

```javascript
    Point: (r) => P.seq(P.string('['), r.Num, P.string(' '), r.Num, P.string(']')).map(([_open, x, _space, y, _close]) => {
        return {
            x: x,
            y: y,
        };
    }),
```

Now we get:

```
$ node index.js
[ [ { x: 2, y: 3 }, { x: 6, y: 2 }, { x: 1, y: 2 } ],
  [ { x: 1, y: 4 }, { x: 2, y: 1 } ],
  [ { x: 6, y: 9 } ] ]
```

This parser is easy to poke and prod at, or swap out components entirely - each part works independently of each other part.

There are libraries for parser combinators in a number of langauges - here's an example of what `PointSet` might look like in Rust using [`combine`](https://github.com/Marwes/combine), assuming we've already defined `sep()` and `point()` parsers:

```rust
fn point_set<I>() -> impl Parser<Input = I, Output = Vec<Point>>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (char('('), sep_by(point(), sep()), char(')')).map(|(_, points, _)| points)
}
```

Syntax aside it's the same thing - composing arbitrary amounts of arbitrarily small parsers to parse any format you'd like.  For Rust, there's also [`nom`](https://github.com/Geal/nom) which leverages macros instead of traits but at the end of the day it's all the same good stuff.

Got a favorite parser combinator library?  Let me know about it!
