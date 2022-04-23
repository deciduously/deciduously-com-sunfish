---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--uhQkmAms--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://thepracticaldev.s3.amazonaws.com/i/bsrei93lrc5zhp2qcgg0.jpg
date: 2019-07-06T12:00:00.000Z
title: Haskell as Training Wheels
tags:
  - haskell
  - beginners
  - functional
---

I don't write a lot of Haskell. In fact, I don't really write _any_ Haskell. My total lifetime output is well under 1000 lines. Every time I sit down to write some Haskell, though, I get reminded of why I like it so much.

Functional programming can be a tricky paradigm to get your head around, but I don't think it's fundamentally challenging. Rather, it's at odds with the instincts you've already built. Thus, when trying to program functionally in a more familiar language, it's quite easy to cheat. JavaScript is a great example. Modern JS is a great language for functional programming, but there is nothing keeping you on the rails, so to speak. You might be leaning on imperative crutches without even realizing you're doing it. Sure, you might be using `reduce` or newfangled stuff like `flatMap` all over the place, but the language itself doesn't care about how sound your code is, it will do almost anything you ask and never complain if you're breaking rules to make things easier on yourself.

Haskell forces you stay in the box. Its draconian compiler means that your code doesn't run unless you've done it right. This feels like a limitation at first, but by forcing you to solve problems functionally it...well...forces you to solve problems functionally.

What's prompting this is yesterday's Dev.to challenge:

{# {% link thepracticaldev/daily-challenge-8-scrabble-word-calculator-41f6 %} #}

I had a long train ride yesterday, and figured a coding challenge would be a great way to pass the time. Immediately upon reading the problem spec, I had an outline of how to solve this problem. I've been spending most of my time over the last few weeks writing either C++ or Rust, so my mental outline was very imperative. You'd traverse the string iteratively, increment a score based on markers that are present, and then adjust that score based on any extra bonuses. I'm reasonably certain this instinctive solution would have worked with a little massaging.

Where's the fun in that, though?

So, of course, I decided to whip out my dusty old Haskell compiler and see if I still knew how to drive it.

I find the most useful one-sentence summary of "functional programming" to be that instead of telling the computer how to compute the result, you just tell the computer what the result _is_. This can be easier said than done, and requires you to re-frame how you think about the problem.

In this specific problem, we are given a string and must return how high that string scores per Scrabble rules. There are a few curveballs - the `*` character is used to double or triple scores, the `^` indicates a blank so the previous letter shouldn't be scored, and words can be appended with a multiplier like `hello(d)` or `hello(t)` to signal that the final result should be doubled or tripled.

The way to frame this is to think about what a scrabble score actually is. Instead of building up the score a piece at a time, say, adding the letters one by one and then checking to see if it needs adjustment, we want an equation that will score _any word_. This looks something like the following:

```haskell
rawScore * wordMultiplier + sevenLetterBonus
```

This equation fits any input - we can just default the multiplier to 1 and the bonus to 0, so that most words that don't need these get `rawScore * 1 + 0`, which is clearly equivalent to `rawScore`.

So, that's what the answer _is_. We just need to manipulate the string passed in so that each of these values is correctly populated by the time we get there. The simplest part is `sevenLetterBonus`. If the raw word is exactly 7 letters, we add 50 points. Our input string may have extra bits like the asterisk or the multiplier suffix, so just strip those to get the actual word:

```haskell
stripMarkers :: String -> String
stripMarkers = filter (\c -> c /= '*' && c /= '^') $ takeWhile (/= '(')
```

Good. Again, this function just describes what the end result is - it's the original word up to a `(` character, with the marker characters filtered out. Quite declarative. Then we can build the bonus:

```haskell
sevenLetterBonus = if (length $ stripMarkers w) == 7 then 50 else 0
```

Perfect, this now works on any input. The multiplier, too, is easy. Some inputs will have a suffix, and if so, check which. If not, the multiplier is 1:

```haskell
wordMultiplier =
    let
        suffix = dropWhile (/= '(') w
    in
        if length suffix > 0 then
            case suffix !! 1 of
                't' -> 3
                'd' -> 2
                _   -> 1
        else 1
```

It only looks at any part of the string after a `(` character, and acts accordingly. This also already handles any string we throw at it - most will hit that `else` block because there is no `(` present and get assigned a 1, which won't change the raw score.

The trickiest part of this for me functionally was handling the asterisks. My instincts tell me to solve this with an iterative loop, but Haskell is not going to let me get away with that. If it did, I likely would have been tempted to take the easy way out. But, of course, I couldn't.

Scoring a list of letters is easy - you just replace each letter with it's numerical value, and sum the list:

```haskell
sum $ map (\c -> scores ! c) $ word
```

The mapping function is just performing a lookup in a mapping from characters to ints. In order for this to work, we need to have a string containing just the letters that will be scored. In order for this little snippet to work on any input, that input should be pre-processed to only contain letters. Any letter we want omitted can be, well, omitted, and letters to count multiple times can just appear multiple times. I don't know if I handled this as cleanly or as elegantly as a Haskeller would have, but this does the trick:

```haskell
expandMarkers :: String -> String
expandMarkers [] = []
expandMarkers (c:[]) = [c]
expandMarkers (c:rest) =
    case head rest of
        '*' ->
            if (head $ tail rest) == '*' then
                [c] ++ [c] ++ [c] ++ (expandMarkers $ drop 2 rest) else
                [c] ++ [c] ++ (expandMarkers $ tail rest)
        '^' -> expandMarkers $ tail rest
        _ -> [c] ++ expandMarkers rest
```

It consumes the string recursively. On each letter, it looks one forward, and then continues the process based on what it finds. An asterisk will get removed and replaced with a copy of the letter we're on, unless the _next_ one is also an asterisk, in which case it will replace both of them, and a carat will cause the character we're on to just not appear in the result. This function turns `he*ll^o**` into `heelooo` - ready to be scored as is via the simple character-to-int substitution.

The full code just puts all this together:

```haskell
import Data.Map (Map, (!))
import qualified Data.Map as Map

scores :: Map Char Int
scores = Map.fromList pairs
    where
        pairs = [
            ('a', 1),
            ('b', 3),
            ('c', 3),
            ('d', 2),
            ('e', 1),
            ('f', 4),
            ('g', 2),
            ('h', 4),
            ('i', 1),
            ('j', 8),
            ('k', 5),
            ('l', 1),
            ('m', 3),
            ('n', 1),
            ('o', 1),
            ('p', 3),
            ('q', 10),
            ('r', 1),
            ('s', 1),
            ('t', 1),
            ('u', 1),
            ('v', 4),
            ('w', 4),
            ('x', 8),
            ('y', 4),
            ('z', 10)]

scoreWord :: String -> Int
scoreWord w =
    let
        sevenLetterBonus = if (length $ stripMarkers w) == 7 then 50 else 0
        wordMultiplier =
            let
                suffix = dropWhile (/= '(') w
            in
                if length suffix > 0 then
                    case suffix !! 1 of
                        't' -> 3
                        'd' -> 2
                        _ -> 1
                else 1
        --
        preparedWord = expandMarkers $ takeWhile (/= '(') w
        rawScore = sum $ map (\c -> scores ! c) $ preparedWord
    in
        rawScore * wordMultiplier + sevenLetterBonus

-- transform doubles, triples, carats
-- if we hit an asterisk, replace it with the previous letter
-- if we hit a carat, drop the previous letter
expandMarkers :: String -> String
expandMarkers [] = []
expandMarkers (c:[]) = [c]
expandMarkers (c:rest) =
    case head rest of
        '*' ->
            if (head $ tail rest) == '*' then
                [c] ++ [c] ++ [c] ++ (expandMarkers $ drop 2 rest) else
                [c] ++ [c] ++ (expandMarkers $ tail rest)
        '^' -> expandMarkers $ tail rest
        _ -> [c] ++ expandMarkers rest

-- remove suffix and all markers for deciding on the 7-letter bonus
stripMarkers :: String -> String
stripMarkers w = filter (\c -> c /= '*' && c /= '^') $ takeWhile (/= '(') w
```

This solution, that pre-processes every input into something that can be easily scored the same way, bears little to no resemblance to the code I wrote in my head when I read the spec. That's _pretty cool_, and I think it's a decent solution in any language.

That's why Haskell is worth it. In order to make it go, you cannot fall back on instinct. You have to actually solve the problem differently, and it won't work until it does. I have no delusions about this being nice, idiomatic Haskell, but _it is working Haskell_, which means I've come up with a working solution that I can now bring to a more familiar environment and implement. If I had sat down to write this in JavaScript instead, I would not have arrived in the same place without a lot more thought and self-discipline, because I would have just written it out how I thought about it first, and deprived myself of the experience of looking at the problem in a new way. Now the next time I approach a similar problem, my toolbox has expanded and my first instinct might actually look more like this. Thanks, Haskell!

Photo by Michal Vrba on Unsplash
