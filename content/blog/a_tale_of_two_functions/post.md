---
cover_image: http://theboardgamingway.com/wp-content/uploads/2015/02/Hastings-from-Bayeux-Tapestry.jpg
date: 2018-12-08T12:00:00.000Z
title: A Tale of Two Functions
tags:
  - adventofcode
  - fsharp
  - beginners
---

## In Which Ben Learns 1 Is The Loneliest Number

So, my biggest shortcoming as a developer is my toolkit of algorithms at my fingertips by instinct. It's not that I'm not pretty familiar with the basics, at least, but I still haven't put in the time necessary to immediately look at a problem and say "oh, this is that that other problem". At least not at 7 in the morning warming up from the frigid trudge up the hill well before my shift. Advent of Code makes fools of us all.

This is the story of how I instinctively reached for the dumb thing _even knowing it was dumb_ instead of taking a second and thinking about it and wasted precious, precious leaderboard points because of it. The humanity.

This is a beginner-level post, even if you aren't terribly comfy with F#/ML.

## The Exposition

Day 5 has us comparing successive characters. If they're a _pair_ of one lower case and one upper case of the same letter both are dropped from the set, and otherwise the process continues. We're done when we're out of pairs.

We'll start with the wrong way. Because this is my 5th problem ever in F# and it's been some time since I've used an ML, I wanted to do it recursively! Hooray! I just forgot that doesn't always mean the same thing.

I also am on a swap week - I'm used to a cushy 90 minutes before I switch off and do numbers all morning until lunch, but this week I only had 60! The clock was ticking on this one but I was amped from Day 4 where my first go did the trick, more or less, and went in cocky. Luckily, I know all about how to solve stuff recursively, I have a few days of ironing out the unfamiliar edges of the languages behind me, and this problem looks like a piece of cake. I'm not going for style on the first time through, I'm going for that sweet, sweet answer.

What I missed at this _crucial_ juncture is (as so many others noted rather quickly) that this only takes a single pass to do. As soon as you swap a pair you should check _right then and there_ if you need to swap again and keep doing that until you're through - that's all it takes! Viola, processed. It's not unlike the [Matching Parenthesis](http://people.cs.ksu.edu/~rhowell/DataStructures/stacks-queues/paren.html) problem - clearly the intended solution. The example given in the problem description even does that operation right there in front of you, by the way. You can't miss it.

I made no such magical leap, though. I missed it. In my first instinct I just saw an operation that needed doing and tried-and-true way to ensure it got done.

I knew I'd want to compare two elements at once as we go through to check if they react, and I'd need a way to tell it to _run through again_ if we made changes to see if any new pairs popped up. After all, this general pattern worked for me on Day 1, part 2:

```fsharp
let rec addFreqWithState acc visited whole remaining =
    match remaining with
    | [] -> addFreqWithState acc visited whole whole
    | head::tail ->
      let newval = acc + head
      if Set.contains newval visited then
          newval
      else
          addFreqWithState newval (Set.add newval visited) whole tail
```

Now, if you're shaking your head by this point, good. You should be. Heck, I was. I looked at the input string - it's huge. This thing is about to do a ton of work, I just knew it before writing any code, but I didn't think it could possibly take that long and I'd just come back later and find a better solution after I got my little happy star - winter, amirite?

I'll just store what I need as parameter to the recursive function - a boolean for whether or not we're done and the original string to start over with. In fact, I'll just drain one into the other and flop them! How simple, how nice. Almost warm and cozy, like a nice cup of ML should be.

## The First Go

I'll start by building the base case:

```fsharp
let rec reactString altered result input =
    match result with
    | [] -> if altered then reactString false "" (string result |> List.ofSeq) else result
```

If it made any changes on this run, then recur again resetting everything, using the new `result` to create our input list of chars. If it didn't - so, you know, it just ran **all the way through again** doing zero work to ascertain this, it can finally give us back the damn result string.

Okay. One case in and it already hurts, but time is money. Let's write the other part and get on with it.

```fsharp
let rec reactString altered result input =
    match result with
    | [] -> if altered then reactString false "" (string result |> List.ofSeq) else result
    | head::next::tail ->
      if doesReact head next then
        reactString true result tail
      else
        reactString altered (result + string head) ([next] @ tail)
```

I get at the first two by destructuring the `input` list and calling them `head` and `next`. I check if they react:

```fsharp
let doesReact first second =
    (System.Char.ToUpper first = System.Char.ToUpper second) && first <> second

```

One of the first initial gotchas right out of the gate with F# is the equality operators - instead of `==` and `!=` you're working with `=` and `<>`.

If they do react, then we make sure we note that in the boolean we're passing along and recur with `tail` - everything after the two we just checked.

We did totally move on from any new pair we created in the result, but it's cool, yo. We'll catch 'em on the next go-round! (oof).

If they didn't react, we're recurring through `input` again but "draining" it into `result` - add the `head` and keep `next` up with the input list for the next iteration.

At this point the compiler helpfully reminds me there's lists with one element, and I have to deal with that reality. Thanks, compile-time enforced correctness! I don't really want to think about it, so we'll "base case" that too - here's our final iteration:

```fsharp
  let rec reactString altered result input =
    match input with
    | [] -> if altered then reactString false "" (string result |> List.ofSeq) else result
    | [a] -> if altered then reactString false "" (string result + string a |> List.ofSeq) else result + string a
    | head::next::tail ->
      if doesReact head next then
        reactString true result tail
      else
        reactString altered (result + string head) ([next] @ tail)
    |> Seq.length
```

It's almost the same as for `[]` - it definitely won't react so we don't check - but we pass it along either back into the input list if needed or add to our accumulated `result` string.

It ain't pretty, but it'll do.

And do it did - pretty much on the first try, which has been my favorite thing about F#. Not first try, exactly, but the first successful compile usually does what I meant. Getting the actual problem answers just involve running this once and then running it a bunch of times on different permutations of the input, removing specific letters at a time and trying again, so here's where any real work is happening. It did what I asked of it, and my answers were correct.

I literally aged while it did it, though. I started unlocking cabinets, I went to the bathroom, I chatted with Mike down the hall, another early-bird. Didn't finish. I went and grabbed the mail, filtered my emails - nothing.

I left my laptop open on my desk. It's an old laptop - late 2011 Thinkpad. It's doing its best. Curses!

It finishes, just twelve minutes until work begins. I had misread the problem - it didn't want the letter that was most optimal, it wanted the resulting length of that string. The result of that massive computation, that measly `'j'` was staring at me, taunting me. I had to run it _again_. Minutes were ticking by and I still didn't have what I needed - even though I did have "the right answer".

Luckily, made the code change in under two minutes. And started it again.

Endless minutes go by. 8 AM comes. I start work, glancing every few minutes as I get my day organized. The phone starts ringing and the emails start coming as my colleagues roll in and I don't get to check back until maybe an excruciating hour later and there it is, smug as ever - the right friggin' answer. Ouch.

## The Realization

It took not two seconds. I opened the thread, got to the top post from @aspittel, and got two lines in to the function

```python
def react(text):
    stack = []
```

_Ohhhhh_. Oh right. Make a stack. It all was so crystal clear in a moment. But alas - the time had come. I had a bunch of contract adjustments to do before I could dive back in.

## The Fix

Fast-forward to lunch, and I simply translate hers:

{# ---
title: A Tale of Two Functions
published: true
description: A brief look back at my Advent of Code error in F#
tags: #adventofcode #fsharp #beginners
cover_image: http://theboardgamingway.com/wp-content/uploads/2015/02/Hastings-from-Bayeux-Tapestry.jpg

---

# In Which Ben Learns 1 Is The Loneliest Number

So, my biggest shortcoming as a developer is my toolkit of algorithms at my fingertips by instinct. It's not that I'm not pretty familiar with the basics, at least, but I still haven't put in the time necessary to immediately look at a problem and say "oh, this is that that other problem". At least not at 7 in the morning warming up from the frigid trudge up the hill well before my shift. Advent of Code makes fools of us all.

This is the story of how I instinctively reached for the dumb thing _even knowing it was dumb_ instead of taking a second and thinking about it and wasted precious, precious leaderboard points because of it. The humanity.

This is a beginner-level post, even if you aren't terribly comfy with F#/ML.

## The Exposition

Day 5 has us comparing successive characters. If they're a _pair_ of one lower case and one upper case of the same letter both are dropped from the set, and otherwise the process continues. We're done when we're out of pairs.

We'll start with the wrong way. Because this is my 5th problem ever in F# and it's been some time since I've used an ML, I wanted to do it recursively! Hooray! I just forgot that doesn't always mean the same thing.

I also am on a swap week - I'm used to a cushy 90 minutes before I switch off and do numbers all morning until lunch, but this week I only had 60! The clock was ticking on this one but I was amped from Day 4 where my first go did the trick, more or less, and went in cocky. Luckily, I know all about how to solve stuff recursively, I have a few days of ironing out the unfamiliar edges of the languages behind me, and this problem looks like a piece of cake. I'm not going for style on the first time through, I'm going for that sweet, sweet answer.

What I missed at this _crucial_ juncture is (as so many others noted rather quickly) that this only takes a single pass to do. As soon as you swap a pair you should check _right then and there_ if you need to swap again and keep doing that until you're through - that's all it takes! Viola, processed. It's not unlike the [Matching Parenthesis](http://people.cs.ksu.edu/~rhowell/DataStructures/stacks-queues/paren.html) problem - clearly the intended solution. The example given in the problem description even does that operation right there in front of you, by the way. You can't miss it.

I made no such magical leap, though. I missed it. In my first instinct I just saw an operation that needed doing and tried-and-true way to ensure it got done.

I knew I'd want to compare two elements at once as we go through to check if they react, and I'd need a way to tell it to _run through again_ if we made changes to see if any new pairs popped up. After all, this general pattern worked for me on Day 1, part 2:

```fsharp
let rec addFreqWithState acc visited whole remaining =
    match remaining with
    | [] -> addFreqWithState acc visited whole whole
    | head::tail ->
      let newval = acc + head
      if Set.contains newval visited then
          newval
      else
          addFreqWithState newval (Set.add newval visited) whole tail
```

Now, if you're shaking your head by this point, good. You should be. Heck, I was. I looked at the input string - it's huge. This thing is about to do a ton of work, I just knew it before writing any code, but I didn't think it could possibly take that long and I'd just come back later and find a better solution after I got my little happy star - winter, amirite?

I'll just store what I need as parameter to the recursive function - a boolean for whether or not we're done and the original string to start over with. In fact, I'll just drain one into the other and flop them! How simple, how nice. Almost warm and cozy, like a nice cup of ML should be.

## The First Go

I'll start by building the base case:

```fsharp
let rec reactString altered result input =
    match result with
    | [] -> if altered then reactString false "" (string result |> List.ofSeq) else result
```

If it made any changes on this run, then recur again resetting everything, using the new `result` to create our input list of chars. If it didn't - so, you know, it just ran **all the way through again** doing zero work to ascertain this, it can finally give us back the damn result string.

Okay. One case in and it already hurts, but time is money. Let's write the other part and get on with it.

```fsharp
let rec reactString altered result input =
    match result with
    | [] -> if altered then reactString false "" (string result |> List.ofSeq) else result
    | head::next::tail ->
      if doesReact head next then
        reactString true result tail
      else
        reactString altered (result + string head) ([next] @ tail)
```

I get at the first two by destructuring the `input` list and calling them `head` and `next`. I check if they react:

```fsharp
let doesReact first second =
    (System.Char.ToUpper first = System.Char.ToUpper second) && first <> second

```

One of the first initial gotchas right out of the gate with F# is the equality operators - instead of `==` and `!=` you're working with `=` and `<>`.

If they do react, then we make sure we note that in the boolean we're passing along and recur with `tail` - everything after the two we just checked.

We did totally move on from any new pair we created in the result, but it's cool, yo. We'll catch 'em on the next go-round! (oof).

If they didn't react, we're recurring through `input` again but "draining" it into `result` - add the `head` and keep `next` up with the input list for the next iteration.

At this point the compiler helpfully reminds me there's lists with one element, and I have to deal with that reality. Thanks, compile-time enforced correctness! I don't really want to think about it, so we'll "base case" that too - here's our final iteration:

```fsharp
  let rec reactString altered result input =
    match input with
    | [] -> if altered then reactString false "" (string result |> List.ofSeq) else result
    | [a] -> if altered then reactString false "" (string result + string a |> List.ofSeq) else result + string a
    | head::next::tail ->
      if doesReact head next then
        reactString true result tail
      else
        reactString altered (result + string head) ([next] @ tail)
    |> Seq.length
```

It's almost the same as for `[]` - it definitely won't react so we don't check - but we pass it along either back into the input list if needed or add to our accumulated `result` string.

It ain't pretty, but it'll do.

And do it did - pretty much on the first try, which has been my favorite thing about F#. Not first try, exactly, but the first successful compile usually does what I meant. Getting the actual problem answers just involve running this once and then running it a bunch of times on different permutations of the input, removing specific letters at a time and trying again, so here's where any real work is happening. It did what I asked of it, and my answers were correct.

I literally aged while it did it, though. I started unlocking cabinets, I went to the bathroom, I chatted with Mike down the hall, another early-bird. Didn't finish. I went and grabbed the mail, filtered my emails - nothing.

I left my laptop open on my desk. It's an old laptop - late 2011 Thinkpad. It's doing its best. Curses!

It finishes, just twelve minutes until work begins. I had misread the problem - it didn't want the letter that was most optimal, it wanted the resulting length of that string. The result of that massive computation, that measly `'j'` was staring at me, taunting me. I had to run it _again_. Minutes were ticking by and I still didn't have what I needed - even though I did have "the right answer".

Luckily, made the code change in under two minutes. And started it again.

Endless minutes go by. 8 AM comes. I start work, glancing every few minutes as I get my day organized. The phone starts ringing and the emails start coming as my colleagues roll in and I don't get to check back until maybe an excruciating hour later and there it is, smug as ever - the right friggin' answer. Ouch.

## The Realization

It took not two seconds. I opened the thread, got to the top post from @aspittel, and got two lines in to the function

```python
def react(text):
    stack = []
```

_Ohhhhh_. Oh right. Make a stack. It all was so crystal clear in a moment. But alas - the time had come. I had a bunch of contract adjustments to do before I could dive back in.

## The Fix

Fast-forward to lunch, and I simply translate hers:

{# {% devcomment 7bid %} #}

Mine looks almost identical, just ML-style. Instead of a for loop, I'm folding into an `Array`. It doesn't take long - maybe 5 minutes to get it to compile:

```fsharp
let reactQuickly input =
    Seq.fold (fun s c ->
      let last = if Array.length s > 0 then Some (Array.last s) else None
      match last with
      | Some x ->
        if c <> x && (x = System.Char.ToUpper c || x = System.Char.ToLower c) then
          Array.sub s 0 (Array.length s - 1)
        else Array.append s [| c |]
      | None -> Array.append s [| c |]) [| |] input
        |> Array.length
```

While I generally like ML-type syntax, even above most other languages I've tried, I've gotta say her Python version looks very nice and clean in comparison. They do the same thing.

On each iteration, `s` is our result array - the `stack` in her implementation. I use `c` for the character from the input we're looking at - sometimes I just prefer `el` here to convey the element of the list we're folding over.

To get at two at a time, instead of looking forward we look back into the stack. We've got access to it right there in the function. If it's empty we store a `None` so we know to just push whatever the string starts with on the first iteration and otherwise we check the current character against the top of the stack.

Instead of `stack.pop` we just return a subset of our accumulator, which has the same effect. That's it though.

To check if it worked, all I did was replace the word `reactString` with `reactQuickly`. Same answers in three seconds on that old laptop, under one second on my desktop at home.

It turns out one pass is _fewer passes_ than lots and lots of passes. Go figure.

See [here](https://github.com/deciduously/aoc2018/blob/master/src/Day5/Library.fs) for the complete file.

Mine looks almost identical, just ML-style. Instead of a for loop, I'm folding into an `Array`. It doesn't take long - maybe 5 minutes to get it to compile:

```fsharp
let reactQuickly input =
    Seq.fold (fun s c ->
      let last = if Array.length s > 0 then Some (Array.last s) else None
      match last with
      | Some x ->
        if c <> x && (x = System.Char.ToUpper c || x = System.Char.ToLower c) then
          Array.sub s 0 (Array.length s - 1)
        else Array.append s [| c |]
      | None -> Array.append s [| c |]) [| |] input
        |> Array.length
```

While I generally like ML-type syntax, even above most other languages I've tried, I've gotta say her Python version looks very nice and clean in comparison. They do the same thing.

On each iteration, `s` is our result array - the `stack` in her implementation. I use `c` for the character from the input we're looking at - sometimes I just prefer `el` here to convey the element of the list we're folding over.

To get at two at a time, instead of looking forward we look back into the stack. We've got access to it right there in the function. If it's empty we store a `None` so we know to just push whatever the string starts with on the first iteration and otherwise we check the current character against the top of the stack.

Instead of `stack.pop` we just return a subset of our accumulator, which has the same effect. That's it though.

To check if it worked, all I did was replace the word `reactString` with `reactQuickly`. Same answers in three seconds on that old laptop, under one second on my desktop at home.

It turns out one pass is _fewer passes_ than lots and lots of passes. Go figure.

See [here](https://github.com/deciduously/aoc2018/blob/master/src/Day5/Library.fs) for the complete file.
