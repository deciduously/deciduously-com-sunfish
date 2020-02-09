---
cover_image: https://thepracticaldev.s3.amazonaws.com/i/rwta9vb9b44e38nj3i4v.png
edited: 2018-11-19T12:00:00.000Z
title: Let's Build a Rust Frontend with Yew - Part 2
published: true
description: Build a Rust client-side application with Yew
tags: rust, webassembly, beginners, webdev
---
## **PART 2**

In the first part, we set up our development environment and ensured we can compile and run our webapp.  This part starts assuming your project folder mirrors [this one](https://github.com/deciduously/hunt-the-wumpus/tree/master/part1).  Please start with [Part 1](https://dev.to/deciduously/lets-build-a-rust-frontend-with-yew---part-1-3k2o) if you have not already done so - or you can skip this one and go right to [Part 3](https://dev.to/deciduously/lets-build-a-rust-frontend-with-yew---part-3-ch3) but you'll likely need to come back through here anyway.

Now we can start modelling the logic.  We'll start by defining the cave.  The traditional game is played in a cave where each room is a vertex of a regular dodecahedron:

![dodecahedron](https://upload.wikimedia.org/wikipedia/commons/3/33/Dodecahedron.png)

From each room we are connected to exactly three other rooms.

To model this we'll simply use a function to map room IDs to available exits.  This will allow us to traverse around the cave.  Place the following in `lib.rs`, above your `Model` declaration:

```rust
fn room_exits(id: u8) -> Option<[u8; 3]> {
  match id {
    1 => Some([2, 5, 8]),
    2 => Some([1, 3, 10]),
    3 => Some([2, 4, 12]),
    4 => Some([3, 5, 14]),
    5 => Some([1, 4, 6]),
    6 => Some([5, 7, 15]),
    7 => Some([6, 8, 17]),
    8 => Some([1, 7, 11]),
    9 => Some([10, 12, 19]),
    10 => Some([2, 9, 11]),
    11 => Some([8, 10, 20]),
    12 => Some([3, 9, 13]),
    13 => Some([12, 14, 18]),
    14 => Some([4, 13, 15]),
    15 => Some([6, 14, 16]),
    16 => Some([15, 17, 18]),
    17 => Some([7, 16, 20]),
    18 => Some([13, 16, 19]),
    19 => Some([9, 18, 20]),
    20 => Some([11, 17, 19]),
    _ => None
  }
}
```

Now let's store the player's current location in the `Model`:

```rust
pub struct Model {
  arrows: u8,
  current_room: u8,
}
```

Don't forget to add it to our initial model too:

```rust
  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Model {
      arrows: 5,
      current_room: 1,
    }
  }
```

Now we can start adding to our UI.  We'll need a new component that will be responsible for rendering the controls.  I like keeping all of these in a folder:

```
$ mkdir src/components
$ touch src/components/controls.rs
```

We'll start with a barebones component:

```rust
use yew::prelude::*;

pub struct Controls {
    title: String,
    exits: [u8; 3],
}

pub enum Msg {}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub exits: [u8; 3],
}

impl Default for Props {
    fn default() -> Self {
        Self { exits: [0, 0, 0] }
    }
}

impl Component for Controls {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Controls {
            title: "Controls".into(),
            exits: props.exits,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<Controls> for Controls {
    fn view(&self) -> Html<Self> {
        html! {
            <div class=("container", "container-controls"),>
                <div class="title",>{&self.title}</div>
                <div class="exits",>{format!("exits: {}, {}, {}", self.exits[0], self.exits[1], self.exits[2])}</div>
            </div>
        }
    }
}
```

Unlike our top-level component, this one accepts some props - we're going to pass in the exits to the room our player is in.  A couple of "gotchas" - take a look at the `html!` macro in the `Renderable` impl block.  We're attaching two classes to the top-level `div` - to do so, you need to wrap them up in a tuple like shown.  Also, if you're using an attribute in your tag like `<div class="title",>`, you need to include that trailing comma for the macro to work.  If you don't, you might end up with a very dense error message - check for these commas before panicking.  Rust macros tend to generate pretty opaque error info - one major drawback of the tech at this point in time.

Also of note - we *must* provide a `Default` impl for our `Props`.  I'm just setting it to `[0, 0, 0]`.

Let's position it within our app.  First, we have to organize our component module:

```
$ echo 'pub mod controls;' > src/components/mod.rs
```

When we add new components, don't forget to add the declaration to this file.  Back up in `lib.rs`, add the module directly after your `extern crate` declarations and bring it into scope:

```rust
mod components;

use self::components::controls::Controls;
```

Now we can attach it to the app.  Down in the `html!` macro let's add the component right below our `<span>` element displaying the arrows.  We'll also section off the stats printout and display the current room.  Adjust yours to match this:

```rust
<div class="hunt",>
    <div class="header",>{"Hunt the Wumpus"}</div>
    <div class="body",>
      <div class=("container""container-stats"),>
        <span class="title",>{"Stats"}</span>
        <br/>
        <span class="arrows",>{&format!("Arrows: {}", self.arrows)}</span>
        <br/>
        <span class="current-room",>{&format!("Current Room: {}"self.current_room)}</span>
      </div>
      <Controls: exits=room_exits(self.current_room).unwrap(),/>
    </div>
</div>
```

Once the rebuild completes, go back to your browser and confirm you see:

Stats
**Arrows: 5**
Current Room: 1
Controls
exits: 2, 5, 8

Pretty plain, but just what we asked for!  Before we get too far into the logic, let's give ourselves something resembling a layout.  This is just going to be a skeleton - I'm no CSS guru.  Feel free to make this whatever you like, this should be enough to get you started.

Replace `scss/hunt.scss` with the following:

```scss
.hunt {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  width: 100%;

  .header {
    flex: 0;
    font-size: 36px;
    font-weight: bold;
    text-align: center;
  }

  .window {
    display: flex;
    flex-direction: row;
  }

  .container {
      border: solid 1px #000;
      display: flex;
      flex-direction: column;
      overflow: hidden;
      margin: 10px;
      padding: 5px;

      >.title {
          border-bottom: dashed 1px #000;
          font-weight: bold;
          text-align: center;
      }
  }
}
```

Don't forget to run `yarn build:style` to regenerate the compiled CSS.

Let's also go ahead and take the opportunity to just break out the Stats out into their own component.  Make a new file `src/components/stats.rs`:

```rust
use yew::prelude::*;

pub struct Stats {
  title: String,
  arrows: u8,
  current_room: u8,
}

pub enum Msg {}

#[derive(PartialEq, Clone)]
pub struct Props {
  pub arrows: u8,
  pub current_room: u8,
}

impl Default for Props {
  fn default() -> Self {
    Self {
      arrows: 0,
      current_room: 0,
    }
  }
}

impl Component for Stats {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
    Stats {
      title: "Stats".into(),
      arrows: props.arrows,
      current_room: props.current_room,
    }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    true
  }
}

impl Renderable<Stats> for Stats {
  fn view(&self) -> Html<Self> {
    html! {
      <div class=("container", "container-stats"),>
        <span class="title",>{&self.title}</span>
        <span class="stat",>{&format!("Arrows: {}", self.arrows)}</span>
        <br/>
        <span class="stat",>{&format!("Current Room: {}", self.current_room)}</span>
      </div>
    }
  }
}
```

New we just add it to `src/components/mod.rs`:

```rust
pub mod controls;
pub mod stats;
```

and include it in our top level component in `lib.rs`:

```rust
mod components;

use self::components::{controls::Controls, stats::Stats};

// down to the bottom...

impl Renderable<Model> for Model {
  fn view(&self) -> Html<Self> {
    html! {
        <div class="hunt",>
            <div class="header",>{"Hunt the Wumpus"}</div>
            <div class="window",>
              <Stats: arrows=self.arrows, current_room=self.current_room,/>
              <Controls: exits=room_exits(self.current_room).unwrap(),/>
            </div>
        </div>
    }
  }
}
```

This gives us a simple flexbox layout that will be easy to extend.  Re-run `yarn build:css-once` and reload `localhost:8000` in your browser to make sure the new style got picked up.

Now we're ready to get **interactive** with it.

Our next order of business is moving around the cave.  All of our actual update logic is going to happen in our top-level component.  When we first created `lib.rs`, we just made an empty `Msg` type:

```rust
#[derive(Debug, Clone)]
pub enum Msg {}
```

To switch `current_room`, we're going to send a `Msg` containing the target room. Let's add the variant first:

```rust
#[derive(Debug, Clone)]
pub enum Msg {
  SwitchRoom(u8),
}
```

Now we have to handle that message.  Inside the `impl Component for Model` block we currently have a stub for `update()`, returning `true`.  Now lets actually use the `Self::Message` parameter it accepts:

```rust
  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::SwitchRoom(target) => {
        self.current_room = target;
        true
      }
    }
  }
```

Don't forget to remove the underscore from `_msg` in the parameter list!

The great thing about using an `enum` for your messages is that the compiler won't let you miss any when you `match` on them - it must be exhaustive.  We also get to easily destructure the variant.  This pattern is not unlike what Elm offers.  You just need to make sure each match arm returns a boolean - or if you like, you can simply return `true` after the `match` block.  Controlling on a per-message basis may allow for more granular performance control - some messages may not require a re-render.

This message is simple - it just switches `current_room`.  Next we need to generate these messages.  Let's dive back in to `src/components/controls.rs`.  We can use `crate::Msg` to refer to the toplevel message our buttons will generate.

We can now create a message that can be passed within this component:

```rust
pub enum Msg {
    ButtonPressed(crate::Msg)
}
```

We also need to add the callback to our props.  Yew has a type ready to go:

```rust
pub struct Controls {
    title: String,
    exits: [u8; 3],
    onsignal: Option<Callback<crate::Msg>>,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub exits: [u8; 3],
    pub onsignal: Option<Callback<crate::Msg>>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            exits: [0, 0, 0],
            onsignal: None,
        }
    }
}
```

Finally, add it to our component initalization:

```rust
fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
    Controls {
        title: "Controls".into(),
        exits: props.exits,
        onsignal: props.onsignal,
    }
}
```

Now we can dynamically create buttons to generate our `crate::Msg`.  We already have the room targets coming in to the component - we just need a way to create a different button for each.  We can abstract this logic out with a local closure in our `view` function:

```rust
impl Renderable<Controls> for Controls {
    fn view(&self) -> Html<Self> {
        let move_button = |target: &u8| {
            use crate::Msg::*;
            let t = *target;
            html! {
                <span class="control-button",>
                    <button onclick=|_| Msg::ButtonPressed(SwitchRoom(t)),>{&format!("Move to {}", target)}</button>
                </span>
            }
        };
        html! {
            <div class=("container", "container-controls"),>
                <div class="title",>{&self.title}</div>
                <div class="exits",>{ for self.exits.iter().map(move_button) }</div>
            </div>
        }
    }
}
```

We then map `move_button` over the exits in our state.  Another gotcha - you've got to dereference `target` outside of the `html!` macro: `let t = *target`.  If our type wasn't `Copy` like `u8`, we'd need to clone it here.

Now we need to handle the message.  Let's fill in our `update`:

```rust
fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
        Msg::ButtonPressed(msg) => {
            if let Some(ref mut callback) = self.onsignal {
                callback.emit(msg);
            }
        }
    }
    false
}
```

No need to re-render on the click.  We'll handle that later when the state actually changes.  We return `false` to make sure we dont waste time on an exra render.  Now we just add the prop to `lib.rs`, down in the `view` function:

```rust
<Controls: exits=room_exits(self.current_room).unwrap(), onsignal=|msg| msg,/>
```

When the button is clicked the `msg` will fire and our toplevel `update` will handle changing the state.  Now we can pass any message we want up as a callback.

There's one final change to make before it all works - we need to tell any component that takes `Props` what to do when those props change.  Define these  `change` functions in the `impl Component for <...>` blocks of these respective components:

First, `controls.rs`:

```rust
fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.exits = props.exits;
    self.onsignal = props.onsignal;
    true
}
```

Then `stats.rs`:

```rust
fn change(&mut self, props: Self::Properties) -> ShouldRender {
  self.arrows = props.arrows;
  self.current_room = props.current_room;
  true
}
```

Now make sure your `yarn watch:rs` watcher is running and open up `localhost:8000`.  You should be able to use the buttons to "explore" the maze.

To keep track of where we've been, let's display a running history for the player.  First, we'll add a field to our toplevel state in `lib.rs`:

```rust
pub struct Model {
  arrows: u8,
  current_room: u8,
  messages: Vec<String>,
}

impl Component for Model {
   // ..
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Model {
      arrows: 5,
      current_room: 1,
      messages: Vec::new(),
    }
  }
  // ..
}
```

We'll add a new component in a new file `src/components/messages.rs`:

```rust
use yew::prelude::*;

pub struct Messages {
  title: String,
  messages: Vec<String>,
}

pub enum Msg {}

#[derive(PartialEq, Clone)]
pub struct Props {
  pub messages: Vec<String>,
}

impl Default for Props {
  fn default() -> Self {
    Props {
      messages: Vec::new(),
    }
  }
}

impl Component for Messages {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
    Messages {
      title: "Messages".into(),
      messages: props.messages,
    }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    true
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.messages = props.messages;
    true
  }
}

impl Renderable<Messages> for Messages {
  fn view(&self) -> Html<Self> {
    let view_message = |message: &String| {
      html! {
          <li>{message}</li>
      }
    };
    html! {
        <div class=("container", "container-messages"),>
            <div class="title",>{&self.title}</div>
            <div class="scroller",>
                <ul>{ for self.messages.iter().rev().map(view_message) }</ul>
            </div>
        </div>
    }
  }
}
```

We're showing the messages in reverse - otherwise, this isn't too different from `controls.rs`.  Protip - I use a snippet something like this when I'm starting a new component!

Don't forget to add it to `src/components/mod.rs`:

```rust
pub mod controls;
pub mod messages;
pub mod stats;
```

And add it to `lib.rs`:

```rust
use self::components::{controls::Controls, messages::Messages, stats::Stats};

// ..

impl Renderable<Model> for Model {
  fn view(&self) -> Html<Self> {
    html! {
        <div class="hunt",>
            <div class="header",>{"Hunt the Wumpus"}</div>
            <div class="window",>
              <Stats: arrows=self.arrows, current_room=self.current_room,/>
              <Controls: exits=room_exits(self.current_room).unwrap(), onsignal=|msg| msg,/>
            </div>
            <Messages: messages=&self.messages,/> // add it down here
        </div>
    }
  }
}
```

Now let's add a little style in `scss/hunt.scss`.  Add the following below the `>.title` block inside the `.container` block:

```scss
>.scroller {
    overflow: auto;
}
```

and then add right at the end:

```scss
.hunt {
// ..
  .container-messages {
    flex: 0 0 192px;
    ul {
      list-style-type: none;
    }
  }
}
```

To pull in the changes, run `yarn build:style`.

Now let's add some messages!  We can welcome the player to their likely doom when the game initiates in `lib.rs`:

```rust
fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
  let mut ret = Model {
    arrows: 5,
    current_room: 1,
    messages: Vec::new(),
  };
  ret.messages.push(
    "You've entered a clammy, dark cave, armed with 5 arrows.  You are very cold.".to_string(),
  );
  ret
}
```

We'll also log each move:

```rust
  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::SwitchRoom(target) => {
        self.current_room = target;
        self.messages.push(format!("Moved to room {}", target));
        true
      }
    }
  }
```

Nifty!  Our cave isn't terribly interesting though.  There's some low-hanging fruit, here - there's gotta be a wumpus to hunt!

Join me in [Part 3](https://dev.to/deciduously/lets-build-a-rust-frontend-with-yew---part-3-ch3) to make a game out of this treacherous dodecacave.

To compare, here's the completed [part 2](https://github.com/deciduously/hunt-the-wumpus/tree/master/part2) code.
