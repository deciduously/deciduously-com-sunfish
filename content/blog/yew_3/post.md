---
cover_image: https://thepracticaldev.s3.amazonaws.com/i/rwta9vb9b44e38nj3i4v.png
date: 2018-11-20T12:00:00.000Z
title: Let's Build a Rust Frontend with Yew - Part 3
tags:
  - rust
  - webassembly
  - beginners
  - webdev
---

## Game On

This is the third and final part of a 3 part series. This post starts off with a project that looks something like [this](https://github.com/deciduously/hunt-the-wumpus/tree/master/part2). Here are links for [Part 1](https://dev.to/deciduously/lets-build-a-rust-frontend-with-yew---part-1-3k2o) and [Part 2](https://dev.to/deciduously/lets-build-a-rust-frontend-with-yew---part-2-1ech) if you need to catch up.

Part 2 left us with a cave we can wander around, but not much in the way of danger. The name of the game is "Hunt the Wumpus" and there's nary a wumpus in sight!

Open up `src/lib.rs`. Let's add one to our `Model`:

```rust
pub struct Model {
  arrows: u8,
  current_room: u8,
  messages: Vec<String>,
  wumpus: u8,
}
```

We need a placeholder starting position - there is no room 0, our cave rooms are 1-indexed:

```rust
fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
  let mut rng = thread_rng();
  let mut ret = Model {
    arrows: 5,
    current_room: 1,
    messages: Vec::new(),
    wumpus: 0,
  };
  // ..
}
```

We'll place him in a moment. That's not quite scary enough, though. In addition to the ravenous monstrosity loafing about there are two gigantic bats. If you end up in a room with a bat, it'll quite literally sweep you off your feet and deposit you elsewhere in the cave.

Now we're gonna crank the horror up to eleven. Forget the two chaos-inducing hellbats. There are also two rooms that are bottomless pits. What the flip, man. **Bottomless**. You'll die of thirst, after three days of falling. Gives me the crimineys, I'll tell you hwat.

We'll keep track of them too:

```rust
pub struct Model {
  arrows: u8,
  current_room: u8,
  messages: Vec<String>,
  wumpus: u8,
  bats: [u8; 2],
  pits: [u8; 2],
}
```

Let's go ahead and implement `Default` for `Model` with some zeros for everything that we can configure later:

```rust
impl Default for Model {
  fn default() -> Self {
    Self {
      arrows: 5,
      current_room: 1,
      messages: Vec::new(),
      wumpus: 0,
      bats: [0, 0],
      pits: [0, 0],
    }
  }
}
```

To place the horribleness, we'll use a helper function that will generate random numbers avoiding a list that we specify.

We're going to call out out to JS to generate the random number. First add the `#[macro_use]` annotation to the `extern crate stdweb` line in `lib.rs`:

```rust
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;
```

I don't want to clutter up `lib.rs` too much, so lets create a file called `src/util.rs`:

```rust
use stdweb::unstable::TryInto;

pub fn js_rand(bottom: u8, top: u8) -> u8 {
  let rand = js! { return Math.random(); };
  let base: f64 = rand.try_into().unwrap();
  (base * top as f64).floor() as u8 + bottom
}

pub fn gen_range_avoiding(bottom: u8, top: u8, avoid: Vec<u8>) -> u8 {
  let mut ret = avoid[0];
  while avoid.contains(&ret) {
    ret = js_rand(bottom, top);
  }
  ret
}
```

The `js_rand` function wraps up our interop so we deal with Rust types as much as we can - we only need JS for the entropy. The helper `gen_range_avoiding` will give us back a `u8` that doesn't appear in `avoid`.

We can also move `room_exits` from `lib.rs` into this file and mark it `pub`. Don't forget to add it to the top of `lib.rs`:

```rust
mod components;
mod util;

use self::{
  components::{controls::Controls, messages::Messages, stats::Stats},
  util::*,
};
```

To make this utility easier to use, let's give `Model` a method for it in `lib.rs`, along with a `configure_cave()` method to initiate our world and place all of our sadistic traps:

```rust
impl Model {
  fn configure_cave(&mut self) {
    self.messages.push(
      "You've entered a clammy, dark cave, armed with 5 arrows.  You are very cold.".to_string(),
    );
    self.wumpus = js_rand(1, 20);
    self.bats[0] = self.get_empty_room();
    self.bats[1] = self.get_empty_room();
    self.pits[0] = self.get_empty_room();
    self.pits[1] = self.get_empty_room();
    self.warning_messages();
  }

  fn get_empty_room(&self) -> u8 {
    gen_range_avoiding(
      0,
      20,
      vec![
        self.current_room,
        self.wumpus,
        self.bats[0],
        self.bats[1],
        self.pits[0],
        self.pits[1],
      ],
    )
  }
}
```

Now we can rewrite our `create` function:

```rust
fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
  let mut ret = Model::default();
  ret.configure_cave();
  ret
}
```

With all this danger lurking around every corner, we should give the player a few warnings as they're stepping around.

Let's add another method to `Model` to sniff around our surroundings. If any of our adjacent rooms has a hazard, we'll alert the player with a spooky message. Add this to the `impl Model` block:

```rust
fn warning_messages(&mut self) {
  for adj in &room_exits(self.current_room).unwrap() {
    let t = *adj;
    if self.wumpus == t {
      self
        .messages
        .push("You smell something horrific and rancid.".into());
    } else if self.pits.contains(&t) {
      self
        .messages
        .push("You feel a cold updraft from a nearby cavern.".into());
    } else if self.bats.contains(&t) {
      self
        .messages
        .push("You hear a faint but distinct flapping of wings.".into());
    }
  }
}
```

We can check for nearby hazards whenever we move:

```rust
fn update(&mut self, msg: Self::Message) -> ShouldRender {
  match msg {
    Msg::SwitchRoom(target) => {
      self.current_room = target;
      self.messages.push(format!("Moved to room {}", target));
      self.warning_messages();
      true
    }
  }
}
```

Before we start dealing with larger level states, let's go ahead and abstract out our `Game` from our `Model`. Create a new file called `src/game.rs`. We're going to pull a lot of the logic we had defined on `Model` and put it here instead.

```rust
use crate::util::*;

pub struct Game {
  pub arrows: u8,
  pub current_room: u8,
  pub messages: Vec<String>,
  pub wumpus: u8,
  bats: [u8; 2],
  pits: [u8; 2],
}

impl Game {
  fn configure_cave(&mut self) {
    self.messages.push(
      "You've entered a clammy, dark cave, armed with 5 arrows.  You are very cold.".to_string(),
    );
    self.wumpus = js_rand(1, 20);
    self.bats[0] = self.get_empty_room();
    self.bats[1] = self.get_empty_room();
    self.pits[0] = self.get_empty_room();
    self.pits[1] = self.get_empty_room();
    self.warning_messages();
  }

  fn get_empty_room(&self) -> u8 {
    gen_range_avoiding(
      0,
      20,
      vec![
        self.current_room,
        self.wumpus,
        self.bats[0],
        self.bats[1],
        self.pits[0],
        self.pits[1],
      ],
    )
  }

  pub fn warning_messages(&mut self) {
    for adj in &room_exits(self.current_room).unwrap() {
      let t = *adj;
      if self.wumpus == t {
        self
          .messages
          .push("You smell something horrific and rancid.".into());
      } else if self.pits.contains(&t) {
        self
          .messages
          .push("You feel a cold updraft from a nearby cavern.".into());
      } else if self.bats.contains(&t) {
        self
          .messages
          .push("You hear a faint but distinct flapping of wings.".into());
      }
    }
  }
}

impl Default for Game {
  fn default() -> Self {
    let mut ret = Self {
      arrows: 5,
      current_room: 1,
      messages: Vec::new(),
      wumpus: 0,
      bats: [0, 0],
      pits: [0, 0],
    };
    ret.configure_cave();
    ret
  }
}
```

Bring everything into scope in `lib.rs`:

```rust
mod components;
mod game;
mod util;

use self::{
  components::{controls::Controls, messages::Messages, stats::Stats},
  game::Game,
  util::*,
};
```

We also moved the "new game" setup into the `Default` implementation. We're going to have to make some changes to `lib.rs`. First, we're going to define a few different types of `Model` we want to be able to render. Change your `struct` to this `enum`:

```rust
pub enum Model {
  Waiting(String),
  Playing(Game),
}
```

Now we have a gamestate for when there isn't an active game. You can remove the old `impl Model` block - that logic ll ended up in `game.rs`. When the app starts, we're waiting to start a new game:

```rust
impl Default for Model {
  fn default() -> Self {
    Model::Waiting("New Game!".into())
  }
}

impl Component for Model {
  // ..
  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Model::default()
  }
  // ..
```

We need a message to kick off a new game:

```rust
#[derive(Debug, Clone)]
pub enum Msg {
  StartGame,
  SwitchRoom(u8),
}
```

This will require a few changes to our `update` function too. We have a new message to handle, and we need to do some extra checking to make sure we're in a gamestate that makes sense:

```rust
fn update(&mut self, msg: Self::Message) -> ShouldRender {
  use self::Msg::*;
  match msg {
    SwitchRoom(target) => match self {
      Model::Playing(game) => {
        game.current_room = target;
        game.warning_messages();
      }
      _ => unreachable!(),
    },
    StartGame => *self = Model::Playing(Game::default()),
  }
  true
}
```

We've now got to make sure we're playing a game before switching rooms but we can send the `StartGame` message to reroll the gamestate at any time.

Finally, we add a match arm for each game state in our `view`:

```rust
impl Renderable<Model> for Model {
  fn view(&self) -> Html<Self> {
    use self::Model::*;

    match self {
      Waiting(s) => html! {
        <div class="hunt",>
          <span class="over-message",>{s}</span>
          <button onclick=|_| Msg::StartGame,>{"Play Again"}</button>
        </div>
      },
      Playing(game) => html! {
          <div class="hunt",>
              <div class="header",>{"Hunt the Wumpus"}</div>
              <div class="window",>
                <Stats: arrows=game.arrows, current_room=game.current_room,/>
                <Controls: exits=room_exits(game.current_room).unwrap(), onsignal=|msg| msg,/>
              </div>
              <Messages: messages=&game.messages,/>
          </div>
      },
  }
}
```

Each state has it's own `html!` macro to render. For good measure, add a little style just below the final closing brace in `hunt.scss`:

```rust
.over-message {
  font-size: 22px;
  color: red;
}
```

Over in `game.rs` lets flesh out everything that we want to check on a move end. Add a new method in our `impl Game` block:

```rust
pub fn move_effects(&mut self) -> Option<String> {
  self.warning_messages();
  if self.current_room == self.wumpus {
    Some("You have been eaten slowly and painfully by the wumpus".into())
  } else if self.pits.contains(&self.current_room) {
    Some(
      "You have fallen into a bottomless pit and must now wait to die, falling all the while"
        .into(),
    )
  } else if self.bats.contains(&self.current_room) {
    // Switch us to a random room
    let current = self.current_room;
    let next = self.get_empty_room();
    self.messages.push(format!(
      "A gigantic bat whisks you from room {} to room {} before you can even blink",
      current, next
    ));
    self.current_room = next;
    self.warning_messages();
    None
  } else {
    None
  }
}
```

Now we've got some actual behavior! If we run into the wumpus or a bottomless pit, we die. If we hit a bat, `current_room` will get a new random value, and we get a new set of warnings for our new location.

I'm having this function return an `Option<String>`. We'll use this to decide if we want to end the game - a `None` will indicate the game should continue, and a `Some(string)` will trigger the end of the game.

Back in `lib.rs`, lets adjust our `update` function. Adjust the `SwitchRoom` message handler:

```rust
SwitchRoom(target) => match self {
       Model::Playing(game) => {
         game.current_room = target;
         if let Some(msg) = game.move_effects() {
           *self = Model::Waiting(msg);
         };
       }
       _ => unreachable!(),
     },
```

Great! Now we can wander around the maze with advance warning of all the horrors within. Click around a while - you'll eventually die. Isn't that fun?

Of course, one final step remains - we must be able to **shoot** this accursed beast.

First, let's create the message for it. Open up `lib.rs` and add the new message type:

```rust
#[derive(Debug, Clone)]
pub enum Msg {
  StartGame,
  ShootArrow(u8),
  SwitchRoom(u8),
}
```

There are a few things we need to handle when the payer makes a shot. If we hit the wumpus, the game will end and show a victory message. If we missed and it was our last arrow - we're out of luck - the wumpus will eventually find you. That's an immediate loss. Also, we're not necessarily subtle - each time we shoot there's a 75% chance we spook the Wumpus into an adjacent chamber. If that adjacent chamber happens to contain you, you're wumpus food. Here's what that might look like in Rust - add this as a new match arm in your `update` function:

```rust
      ShootArrow(target) => match self {
        Model::Playing(game) => {
          if game.wumpus == target {
            *self = Model::Waiting("With a sickening, satisfying thwack, your arrow finds its mark.  Wumpus for dinner tonight!  You win.".into());
          } else {
            game.arrows -= 1;
            game
              .messages
              .push("You arrow whistles aimlessly into the void".into());

            // If we exhausted our arrows, we lose
            if game.arrows == 0 {
              *self =
                Model::Waiting("You fired your very last arrow - you are now wumpus food".into());
            } else {
              // On each shot there's a 75% chance you scare the wumpus into an adjacent cell.
              let rand = js_rand(1, 4);
              if rand == 1 {
                game.messages.push(
                  "You listen quietly for any sign of movement - but the cave remains still."
                    .into(),
                );
              } else {
                game
                  .messages
                  .push("You hear a deafening roar - you've disturbed the wumpus!".into());
                let wumpus_exits = room_exits(game.wumpus).unwrap();
                let rand_idx = js_rand(0, 2);
                game.wumpus = wumpus_exits[rand_idx as usize];
                if game.wumpus == game.current_room {
                  *self = Model::Waiting(
                    "You scared the wumpus right on top of you.  Good going, mincemeat".into(),
                  );
                }
              }
            }
          }
        }
```

Great! Now all we need are some buttons to actually fire arrows. Luckily, we've already got almost everything we need. Over in `src/components/controls.rs`, lets make a little tweak to our `move_button` closure:

```rust
let move_button = |target: &u8| {
  use crate::Msg::*;
  let t = *target;
  html! {
      <div class="control-button",>
          <button onclick=|_| Msg::ButtonPressed(SwitchRoom(t)),>{&format!("Move to {}", target)}</button>
          <button onclick=|_| Msg::ButtonPressed(ShootArrow(t)),>{&format!("Shoot {}", target)}</button>
      </div>
  }
};
```

And that's the way the news goes! Happy Wumpus huntin'. Here's the [part 3](https://github.com/deciduously/hunt-the-wumpus/tree/master/part3) code to compare.

Please show me if you improve this app! I want to see what you come up with.
