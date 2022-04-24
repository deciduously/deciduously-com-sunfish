---
cover_image: https://upload.wikimedia.org/wikipedia/en/thumb/d/d5/Fsharp%2C_Logomark%2C_October_2014.svg/1200px-Fsharp%2C_Logomark%2C_October_2014.svg.png
date: 2018-12-03T12:00:00.000Z
title: F# is Pretty Cool
description: A retrospective on my first few days with F#
tags:
  - fsharp
  - beginners
  - retrospective
---

I decided to tackle this year's Advent of Code in [F#](https://fsharp.org/). It's not only my first time using this language, it's my first time ever using .NET. I don't know anything about using the Common Language Infrastructure at all. I was expecting a rough, slow start as I got used to a brand new environment, and I'd be able to write about the process as I learn how to unstick myself.

Hasn't happened. Turns out F# is great, highly easy to use, and I haven't gotten stuck. In fact, it's probably the quickest I've made it from "brand-new language" to "solved an AoC-type problem", ever. So I'm just going to write that post instead.

I've certainly gotten stuck on the problems, and I'm not necessarily pleased with my implementations so far - all could be optimized - but my issues have nothing to do with F#. The documentation is thorough, and I'm generally a single Google away from the .NET function I'm looking for. Lots of documentation for C# will apply too if you can't find anything for F# specifically - my only trouble is not knowing any C# either, but it's Java-enough to follow along!

Now, to be fair, I don't think you'd have the same experience if it were your first MLish language. But having even just a very little bit myself in Haskell and OCaml I found nothing surprising here.

Almost everything I've needed I've found right on the [Tour of F#](https://docs.microsoft.com/en-us/dotnet/fsharp/tour) and the [Language Reference](https://docs.microsoft.com/en-us/dotnet/fsharp/language-reference/) covered everything else.

There's also this great website, downloadable as an offline ebook: [F# for Fun and Profit](https://fsharpforfunandprofit.com/).

Some things I like:

Pipes:

```fsharp
util.applyClaims fileName
|> Seq.filter (fun el -> List.length el > 1)
|> Seq.length
```

List computations:

```fsharp
// https://docs.microsoft.com/en-us/dotnet/fsharp/tour
let daysList =
    [ for month in 1 .. 12 do
          for day in 1 .. System.DateTime.DaysInMonth(2017, month) do
              yield System.DateTime(2017, month, day) ]
```

Active patterns:

```fsharp
// https://fsharpforfunandprofit.com/posts/convenience-active-patterns/
open System.Text.RegularExpressions
let (|FirstRegexGroup|_|) pattern input =
   let m = Regex.Match(input,pattern)
   if (m.Success) then Some m.Groups.[1].Value else None

// create a function to call the pattern
let testRegex str =
    match str with
    | FirstRegexGroup "http://(.*?)/(.*)" host ->
           printfn "The value is a url and the host is %s" host
    | FirstRegexGroup ".*?@(.*)" host ->
           printfn "The value is an email and the host is %s" host
    | _ -> printfn "The value '%s' is something else" str

// test
testRegex "http://google.com/test"
testRegex "alice@hotmail.com"

```

I've just found it nice and smooth to use. It's not hard to get from thought to code and have it work as intended. After a while learning all about building graphs in Rust, it's kinda nice to remember what that's like!

Some things I don't like:

- Compiler errors, but I'm spoiled with Rust/Reason
- Having to use the CLI to add things to solutions and references to packages and things. This is my own lack of familiarity with the ecosystem though.

That's really it, I like everything else a lot. It's the most fun I've had with an ML language so far, at least.

Try you some F#, today!

As an aside, does anyone have any experience using Clojure CLR? Seems not too popular, but a good idea in general.

This post isn't really about AoC, but here's an "obligatory" [repo](https://github.com/deciduously/aoc2018) link if you'd like to play around with it.
