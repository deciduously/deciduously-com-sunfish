---
cover_image: https://res.cloudinahttps://thepracticaldev.s3.amazonaws.com/i/lk886f5xd4t64pa2cw9i.jpg
edited: 2019-12-16T12:00:00.000Z
title: Pressure-Free AoC
published: true
description: A reflection on taking my time with Advent of Code problems
tags: adventofcode, devjournal, watercooler, beginners
---
I'm really glad I didn't start [Advent of Code 2019](https://adventofcode.com/2019) until halfway through December.

I am *not good* at follow through with this challenge, and in previous years I think I made two mistakes:

1. I used it to learn brand new, funky languages (Clojure, F#, Haskell) instead of leveraging a skill I've cultivated and care about continuing to hone (Rust, C++, JavaScript).
2. I rushed through the solutions.

I'm not super competitive by nature, but when I realized I was at a comfort level with programming such that I could complete the first few days quickly, I got too in my head about it.  The pressure to "deliver" leads me to inevitably burn out sometime in week 2 and never come back.  The days I do get solved are usually not well optimized, as I was just looking for the correct output to get my little star.  Once I solved it I didn't spend enough time revisiting to learn something about the problem - there's another one waiting!

Additionally, using those types of fancy functional languages gave me a bad habit of looking for "clever" concise solutions to problems without thinking about practicality or readability.  While that's a fun exercise, and not *completely* without its educational merit, it doesn't pack quite the same punch as actually practicing how to craft quality software.

I wrote about one such case here, where I missed an algorithm I even already knew in favor of a simple brute-force approach which needlessly abused my laptop's CPU, costing me my speedrun:

{# {% post deciduously/a-tale-of-two-functions-44h5 %} #}

If I had just slowed down, I would have seen the "proper" way myself and probably still gotten there that morning, but AoC is a trip.

I still plan to use these old repos I started to learn those cool languages if and when I return, but 2019 is already so much more satisfying than those experiments ever were.

This year I started Day 1 on December 12, so there was no hope of catching up.  I used Rust, a language I started abandoning projects in [3 years ago](https://github.com/deciduously/dice), so getting organized was not a problem.  It turns out already knowing the idioms and standard library of your AoC language is useful for getting off the ground.  Go figure.  My runner-up was C++ for maximum industry-relevance, but I'm writing enough of that for school and Rust has less hassle and more beginner-friendly tooling.  I feel it's about as educational on an abstracted problem-solving level.

I even took the time to set up some scaffolding, something I'd never gotten around to before:

```rust
// src/lib.rs
use std::{
    fs::File,
    io::{self, BufReader, ErrorKind::*, Read},
};

const INPUT_DIR: &str = "inputs";

fn get_puzzle_string(day: u8) -> Result<String, io::Error> {
    let filename = format!("{}/day{}.txt", INPUT_DIR, day);
    let mut ret = String::new();

    if let Ok(file) = File::open(&filename) {
        // Read it from disk
        let mut buf = BufReader::new(file);
        buf.read_to_string(&mut ret)?;
        Ok(ret)
    } else {
        Err(io::Error::new(InvalidData, format!("You need to log in to adventofcode.com via a web browser and download the Day {} puzzle input!", day)))
    }
}
```

As well as the rudimentaryest of CLIs:

```rust
use aoc2019::*;
use std::env::args;

const DAYS_IMPLEMENTED: u32 = 5;

fn main() {
    if let Some(day) = args().nth(1) {
        if let Ok(day) = day.parse::<u32>() {
            if day <= DAYS_IMPLEMENTED && day > 0 {
                println!("Day {}", day);
                match day {
                    1 => day1::run(),
                    2 => day2::run(),
                    3 => day3::run(),
                    4 => day4::run(),
                    5 => day5::run(),
                    _ => unreachable!(),
                }
            } else {
                eprintln!("Day must be between 1 and {} inclusive", DAYS_IMPLEMENTED);
            }
        } else {
            eprintln!("Day must be a number 1-{}", DAYS_IMPLEMENTED);
        }
    } else {
        eprintln!("You must select a day 1-{} to run", DAYS_IMPLEMENTED);
    }
}
```

Even tiny little quality-of-life improvements like these make the experience of stepping through these challenges much more fun - no more manually wrangling inputs or calling specific problem entry point functions:

```txt
$ cargo run -- 4
   Compiling aoc2019 v0.1.0 (H:\code\aoc2019)
    Finished release [optimized] target(s) in 3.44s
     Running `target\release\aoc.exe 4`
Day 4
921
603
```

That didn't take more than a few minutes to put together, but when rushing for stars I'd never even bothered.  This also will let me expand from here - if I want to benchmark each run, or add visualizations, I now have a clear structure for everything instead of just throwing logic wherever it fits.

I knocked off the first day very quickly, but then Day 2 was a step up in complexity.  If I were "on the clock", I'd have cut corners, but Rust really shines when you take your time to fully model the problem and craft your solution from all sides.

When I sit down to solve anything in Rust, I like to write a whole pile of code modelling the domain in terms of types and the relationships between them before solving problems in the space.  It's not quick and tends towards the overly verbose, but by the end I usually have a pretty good understanding of the problem space and decently abstracted toolkit for working with it.  In some cases I already have a solution ready to go before even reading what the actual test case is.

This style fits really well for Advent of Code problems because each day is presented in two parts, with the second part building upon the first in some unknown way.  Depending on how you've approached your implementation for Part 1, you may have a lot of work to get to Part 2 - in some cases starting from scratch entirely - or you may already be surprisingly close.

After solving each, I've been going back and hardcoding the puzzle solutions as tests in addition to the sample data tests:

```rust
#[test]
fn test_solutions() {
    assert_eq!(IdRange::from_str(PUZZLE).unwrap().total_inputs(false), 921);
    assert_eq!(IdRange::from_str(PUZZLE).unwrap().total_inputs(true), 603);
}
```

The only problem is that some of these puzzle solutions really put your CPU to work.  If you've written a well-designed, optimized implementation it shouldn't be terrible, but can still slow down the test runner.  The shown Day 4 tests run quickly for me in `release` mode but take several seconds in test mode, which `cargo test` uses.  I don't pull in any crates, so decided to just run optimizations in test mode anyway.  You can override this in `Cargo.toml`:

```toml
[profile.test]

opt-level = 3
```

This way you still get debug symbols built in during testing but it can crunch through all the puzzle solutions in a fraction of a second.

Over the weekend I knocked out the first four days, and now starting Day 5 have found that that extra effort paid off.  I was feeling a little silly about my verbosity - just Days 1-4 have already inflated my repo to nearly 1,000 lines of Rust - but this challenge reuses the "Intcode Computer" you build in Day 2 and extends further from there.  This little toy [virtual machine](https://en.wikipedia.org/wiki/Virtual_machine) is going to be used and extended throughout the month.

If I had gone the quick-and-dirty route to crunch through the specific given inputs had initially thought of, Day 5 would have likely meant starting from scratch and writing *even more* code.  Because I took my time to think about the design and set up a well-abstracted, encapsulated Intcode VM, I'm going to be able to minimally modify what I've already got and have it run both challenges.  I don't anticipate needing to rewrite much, if any, code that I've already created.

Taking the pressure off has turned out to be exactly what I needed to make the most of this super cool event.  This just might be the first set of challenges I complete, but, you know, don't wait up...

The [GitHub](https://github.com/deciduously/aoc2019) link, for the curious.

*cover image: [reddit](https://www.reddit.com/r/adventofcode/comments/e9sxog/beautiful/)*
