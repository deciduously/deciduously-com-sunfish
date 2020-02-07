---
cover_image: https://res.cloudinahttps://thepracticaldev.s3.amazonaws.com/i/lk886f5xd4t64pa2cw9i.jpg
edited: 2019-09-03T12:00:00.000Z
title: Callbacks, Trait Objects & Associated Types, Oh My!
published: true
description: A reflection on taking my time with Advent of Code problems
tags: devjournal, rust
---
Last week, I promised a dive into Rust procedural macros as I work to design a DSL for my `Widget` UI building-block trait.  What I've learned is not to ever promise things like that, because I didn't even touch that aspect this week.  You may want to scooch back off the edge of your seat for at least another week.

Instead, my `Widget`s are clickable now!  A `Button` you can't click is pretty useless, so I figured I'd get that system functional before trying anything fancy on top of it.

## Callbacks

In order to keep the rendering stuff decoupled from the game logic, I need users of this library to be able to define clickable functionality however they please.  The solution I settled on is a little bit Flux-ish - to create a clickable widget, you provide a callback that returns an action type.  When a click is registered, the grid dives through its children to see where exactly the click was located.  If a match is found, that action will bubble up through successive `handle_click` calls until some parent widget above it decides to handle that action.

This affords a pretty high degree of flexibility - *any* widget in your application can choose to handle a message.  This allows you to use container widgets that can handle their children as a group, and only pass up what's necessary for a global state change if needed.

The first problem was representing this callback in a way that's clone-able and easy to store in a struct.  I ended up using an [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html), or reference-counted smart pointer:

```rust
pub struct Callback<T> {
    f: Rc<dyn Fn() -> T>,
}

impl<T> Callback<T> {
    /// Call this callback
    pub fn call(&self) -> T {
        (self.f)()
    }
}

impl<T> Clone for Callback<T> {
    fn clone(&self) -> Self {
        Self {
            f: Rc::clone(&self.f),
        }
    }
}
```

To construct them, you can call `Callback::from()` on a closure, as used in the demo app:

```rust
let button = Button::new(
    // Display name
    &format!("{:?}", self.value),
    // Optional size, if None will calculate based on display name
    Some((VALUES.die_dimension, VALUES.die_dimension).into()),
    // Border color
    Color::from_str("black").unwrap(),
    // Click action
    Some(Callback::from(move || -> FiveDiceMessage {
        FiveDiceMessage::HoldDie(id)
    })),
);
```

The caveat is that I haven't figured out how not to require that the return type have a `'static` lifetime:

```rust
impl<T, F: Fn() -> T + 'static> From<F> for Callback<T> {
    fn from(func: F) -> Self {
        Self { f: Rc::new(func) }
    }
}
```

This is part of what led me towards the action-reducer type thing - the actions themselves can be just plain data like the example!  You define an enum for your messages:

```rust
#[derive(Debug, Clone, Copy)]
pub enum FiveDiceMessage {
    HoldDie(usize),
    RollDice,
    StartOver,
}
```

And then a reducer to handle them:

```rust
impl Game {
    // ..

    /// Handle all incoming messages
    fn reducer(&mut self, msg: FiveDiceMessage) {
        use FiveDiceMessage::*;
        match msg {
            HoldDie(idx) => self.hold_die(idx),
            RollDice => self.roll_dice(),
            StartOver => self.reset(),
        }
    }

    // ..
}
```

It's not a real "reducer", we're mutating in place instead of using pure functions, but it's a similar pattern.

Ideally, it'd be nice to be able to accept more generic callbacks, and not lock users into this pattern.  I'm good with this for now though.  However, It was a little bit tricky integrating this into my `Widget` definition.  In the previous post, I gave the trait definitions I was working with:

```rust
/// Trait representing things that can be drawn to the canvas
pub trait Drawable {
    /// Draw this game element with the given top left corner
    /// Only ever called once mounted.  Returns the bottom right corner of what was painted
    fn draw_at(&self, top_left: Point, w: WindowPtr) -> Result<Point>;
    /// Get the Region of the bounding box of this drawable
    fn get_region(&self, top_left: Point, w: WindowPtr) -> Result<Region>;
}

/// Trait representing sets of 0 or more Drawables
/// Each one can have variable number rows and elements in each row
pub trait Widget {
    /// Get the total of all regions of this widget
    fn get_region(&self, top_left: Point, w: WindowPtr) -> Result<Region>;
    /// Make this object into a Widget.  Takes an optional callback
    fn mount_widget(&self) -> MountedWidget;
}
```

I need to add a method to `Widget` that detects a click and bubbles up whatever the callback returns:

```rust
pub trait Widget {
    // ..
    /// Handle a click in this region
    fn handle_click(
        &mut self,
        top_left: Point,
        click: Point,
        w: WindowPtr,
    ) -> Result<Option<???>>;
}
```

The method needs to return whatever is coming out of these stored callbacks if the passed click falls inside this widget.  Thing is, `Callback<T>` has gone and gotten itself all generic.  This poses a problem, because we can't parameterize the trait itself with this type, like this:

```rust
pub trait Widget<T> {
    // ..
    /// Handle a click in this region
    fn handle_click(
        &mut self,
        top_left: Point,
        click: Point,
        w: WindowPtr,
    ) -> Result<Option<T>>;
}
```

I need to be able to construct `Widget` trait objects, and that `T` is `Sized`.  That's no good - a trait object is a [dynamically-sized type](https://doc.rust-lang.org/nomicon/exotic-sizes.html) and *cannot* have a known size at compile time.  Depending on a monomorphized generic method means that you do have that information - you can't have your cake and eat it too.  I kinda blew past this point last time but it bears a little more explanation.

## Trait Objects

This library utilizes dynamic dispatch to allow for different applications and different backends to swap in and out using a common interface.  To utilize it, you instantiate the following struct:

```rust
/// Top-level canvas engine object
pub struct WindowEngine {
    window: Rc<Box<dyn Window>>,
    element: Box<dyn Widget>,
}
```

The `Widget` and `Window` traits just define some methods that need to be available - they don't describe any specific type.  When we actually do put a real type in a `Box` to put in this struct, we completely lose the type information and only retain the trait information.  A [vtable](https://en.wikipedia.org/wiki/Virtual_method_table) is allocated instead with pointers to each method the trait specifies, and the pointer to it actually *also* contains this vtable pointer to complete the information needed to run your code.  This means, though, that we can't use monomorphized generic types behind the pointer, because we literally don't even know what type we have.  It's all handled through runtime reflection via these vtable pointers, you cannot use anything else.  This is a good thing, it lets us define `Widget`s of all different shapes and sizes (memory-wise) and use them all identically.  That's why `Widget<T>` is so problematic, though - it requires knowing all about what types are in play at compile time, which we emphatically do not.

## Associated Types

Luckily there's a simple, ergonomic solution.  Instead of parameterize the trait, you can just associate a type as part of the trait definition:

```rust
pub trait Widget {
    type MSG; // Associated message type

    /// Handle a click in this region
    fn handle_click(
        &mut self,
        top_left: Point,
        click: Point,
        w: WindowPtr,
    ) -> Result<Option<Self::MSG>>;
}
```

Now we can parameterize the instantiated struct without needing the `Widget` definition itself to carry any baggage:

```rust
pub struct WindowEngine<T: 'static> {
    window: Rc<Box<dyn Window>>,
    element: Box<dyn Widget<MSG = T>>,
}
```

Now when the vtable is created, it still has all the information it needs for your specific application's state management without constraining your type at all beyond `'static`.  The WindowEngine itself gets monomorphized with that type, i.e. `WindowEngine<FiveDiceMessage>`, so all `Widgets` this `WindowEngine` contains will use the same type.

When you write this type, you just fill it in at the application level:

```rust
impl Widget for Die {
    type MSG = FiveDiceMessage; // Specify associated type
    fn mount_widget(&self) -> MountedWidget<Self::MSG> {
         // ..
    }
    fn get_region(&self, top_left: Point, w: WindowPtr) -> WindowResult<Region> {
        // ..
    }
    fn handle_click(
        &mut self,
        top_left: Point,
        click: Point,
        w: WindowPtr,
    ) -> WindowResult<Option<Self::MSG>> {
        // ..
    }
}
```

Associated types are a feature I knew about from using some standard library traits.   For example, [std::str::FromStr](https://doc.rust-lang.org/std/str/trait.FromStr.html) has you specify the error type to use:

```rust
impl FromStr for Color {
    type Err = WindowError; // Associated Err type

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "black" => Ok(Color::new(0, 0, 0)),
            "red" => Ok(Color::new(255, 0, 0)),
            "blue" => Ok(Color::new(0, 0, 255)),
            "green" => Ok(Color::new(0, 255, 0)),
            // ..
        }
    }
}
```

I hadn't thought about a use case where I'd need to write my own trait with one of these until I fell into the situation backwards.  So it goes.

### Lingering Concern - PhantomData?

There's one weird bit that I don't feel fully comfortable with.  I have a generic `Text` widget designed to just plop a string on the canvas, that's not clickable.  Its `handle_click` method doesn't return anything, so I use the `None` variant in the `Widget` impl:

```rust
impl<T: 'static> Widget for Text<T> {
    type MSG = T;
    // ..
    fn handle_click(&mut self, _: Point, _: Point, _: WindowPtr) -> Result<Option<Self::MSG>> {
        Ok(None)
    }
}
```

However, this still requires that we parameterize `Text` with the message type of this particular widget tree, even though it's never used, because the return type still contains the `Some(T)` variant's type.  This is what gets it to stop yelling at me:

```rust
/// A widget that just draws some text
pub struct Text<T> {
    phantom: std::marker::PhantomData<T>,
    text: String,
}

impl<T> Text<T> {
    pub fn new(s: &str) -> Self {
        Self {
            phantom: std::marker::PhantomData,
            text: s.into(),
        }
    }
}
```

Per the [docs](https://doc.rust-lang.org/beta/std/marker/struct.PhantomData.html), `PhantomData` is a zero-sized type that just "tells the compiler that your type acts as though it stores a value of type T, even though it doesn't really."  This sounds like what I'm doing here, but I don't have a good sense of whether this is a kludge that I should try to refactor or the correct way to handle this situation.

## Oh, my!

Questions and doubts aside, it all works as planned.  Maybe, *juuuust* maybe, we'll hit up those procedural macros sometime.

*Photo by 🇸🇮 Janko Ferlič - @specialdaddy on Unsplash*
