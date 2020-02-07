---
cover_image: https://res.cloudinahttps://thepracticaldev.s3.amazonaws.com/i/lk886f5xd4t64pa2cw9i.jpg
edited: 2019-09-06T12:00:00.000Z
title: The Builder Pattern
published: true
description: A walkthrough of the builder pattern in Rust
tags: rust, beginners, tutorial, patterns
---
# Matchmaking Like Its 2001

We're going to help John McCrea of Cake find the woman of his slightly sarcastic, oddly specific dreams.  He's a particular man:

{# {% youtube X5KmB8Laemg %} #}

Yep, you and I both think it: this sounds like a job for the Rust compiler.  This band was truly ahead of its time.  Let's model the problem:

```rust
/// Girl type
struct Girl {}

impl Girl {
    /// Construct a Girl
    fn new() -> Self {
        Self {}
    }
}

/// Determine whether given girl matches spec
fn is_dream_girl(girl: &Girl) -> bool {
    // we don't know anything about spec yet, so odds are no
    false
}

fn main() {
    let girl = Girl::new();
    println!("Match: {}", is_dream_girl(&girl));
}
```

Running this with `cargo run` yields the expected output: `Match: false`.

So, what is it we're looking for specifically?  Luckily, our man starts right in with the preferences, on the first line he tells us he wants "a girl with a mind like a diamond".  Let's add a member field to test for:

```rust
#[derive(Clone, Copy, PartialEq)]
enum Mind {
    Computer,
    Diamond,
    Garden,
    Scalpel,
    Sieve,
    Unknown,
}

struct Girl {
    mind: Mind,
}

impl Girl {
    fn new(mind: Mind) -> Self {
        Self { mind }
    }
}
```

Now the test function can check for the requested variant:

```rust
fn is_dream_girl(girl: &Girl) -> bool {
    girl.mind == Mind::Diamond
}

fn main() {
    let girl = Girl::new(Mind::Diamond);
    println!("Match: {}", is_dream_girl(&girl));
}
```

Great!  Now we get `Match: true` when passing in this `Girl`.  Hold on, though - we've got some more criteria.  Next, we need "a girl who knows what's best".  That's pretty easy - either she does or she doesn't:

```rust
struct Girl {
    mind: Mind,
    knows_best: bool,
}

impl Girl {
    fn new(mind: Mind, knows_best: bool) -> Self {
        Self { mind, knows_best }
    }
}

fn is_dream_girl(girl: &Girl) -> bool {
    girl.mind == Mind::Diamond && girl.knows_best
}
```

Just add it to the parameter list:

```rust
fn main() {
    let girl = Girl::new(Mind::Diamond, true);
    println!("Match: {}", is_dream_girl(&girl));
}
```

Groovy.  Now we need "shoes that cut and eyes that burn like cigarettes".  It sounds like we'll need to associate some pairs of strings:

```rust
type Attribute = (String, String);

/// Girl type
struct Girl {
    items: Vec<Attribute>,
    mind: Mind,
    knows_best: bool,
}
```

An attribute will be a tuple like `("shoes", "cut")`.  We can ask for the shoes and eye attributes in the constructor:

```rust
impl Girl {
    fn new(mind: Mind, knows_best: bool, shoes: &str, eyes: &str) -> Self {
        let mut ret = Self {
            items: Vec::new(),
            mind,
            knows_best,
        };
        ret.push_item("shoes", shoes);
        ret.push_item("eyes", eyes);
        ret
    }

    fn push_item(&mut self, item_name: &str, attribute: &str) {
        self.items.push((item_name.into(), attribute.into()));
    }
}
```

We'll just check through the items to make sure we get what we want:

```rust
fn is_dream_girl(girl: &Girl) -> bool {
    let mut found_shoes = false;
    let mut found_eyes = false;
    for item in &girl.items {
        if item.0 == "shoes" && item.1 == "cut" {
            found_shoes = true;
        } else if item.0 == "eyes" && item.1 == "burn like cigarettes" {
            found_eyes = true;
        }
    }
    girl.mind == Mind::Diamond && girl.knows_best && found_shoes && found_eyes
}
```

Awesome!  We just need to construct the `Girl` with the new attributes:

```rust
fn main() {
    let girl = Girl::new(Mind::Diamond, true, "cut", "burn like cigarettes");
    println!("Match: {}", is_dream_girl(&girl));
}
```

Okay.  Hold on.  Do you see the problem here?  Let's skim ahead...

```
I want a girl with the right allocations
Who's fast and thorough
And sharp as a tack
She's playing with her jewelry
She's putting up her hair
She's touring the facility
And picking up slack
...
```

It just continues from there!  This `Girl` constructor is already getting out of hand and we just barely made it out of the first stanza.  What if John changes his mind?  He might decide something's not as important, or add a new criterion.  This code is not amenable to changes like that, every call site is dependent on this exact parameter list given in this exact order, but people don't work like that.  There could be all sorts of variations.

## The Pattern

Let's re-implement this program leveraging the Builder Pattern.  When a `Girl` is first constructed, we just want to start with some sensible defaults:

```rust
struct Girl {
    items: Vec<Attribute>,
    mind: Mind,
    knows_best: bool,
}

impl Girl {
    fn new() -> Self {
        Self::default()
    }
}

impl Default for Girl {
    fn default() -> Self {
        Self { mind: Mind::Unknown, knows_best: false, items: Vec::new() }
    }
}
```

Everything else is a blank slate.  This way we can just use `Girl::new()` with no parameters and get a starting point.  To add more, we can define methods:

```rust
impl Girl {
    // ..

    fn set_mind(&mut self, mind: Mind) -> &mut Self {
        self.mind = mind;
        self
    }
}
```

This method takes a mutable reference and returns one, so we can construct first and adjust later:

```rust
fn main() {
    let mut girl = Girl::new();
    girl.set_mind(Mind::Diamond);
    println!("Match: {}", is_dream_girl(&girl));
}
```

Let's add the rest:

```rust
impl Girl {
    fn push_item(&mut self, item_name: &str, attribute: &str) {
        self.items.push((item_name.into(), attribute.into()));
    }

    fn toggle_knows_best(&mut self) -> &mut Self {
        self.knows_best = !self.knows_best;
        self
    }
}
```

Now we can add them one at a time, regardless of what we know at the time of construction:

```rust
fn main() {
    let mut girl = Girl::new();
    girl.set_mind(Mind::Diamond);
    girl.toggle_knows_best();
    girl.push_item("shoes", "cut");
    girl.push_item("eyes", "burn like cigarettes");
    println!("Match: {}", is_dream_girl(&girl));
}
```

This is so much easier to work with as the specification grows and evolves.

More complex scenarios may require you to use a separate type, like GirlBuilder, and take ownership at each step.  This will allow you to do this all in a one-liner: `let girl = GirlBuilder::new().set_mind(Mind::Diamond).toggle_knows_best();`  This does limit your configuration options, for instance if you want to conditionally call some builder method in an `if` expression.  If possible, the non-owning pattern here is more flexible.

Here's hoping we can help Mr. McCrea finally settle down after all this time.

## Challenge

Prepare yourself for five years from now when this song is an "oldie".
