---
cover_image: https://thepracticaldev.s3.amazonaws.com/i/gmxpjdhz2cgv8irnpcjc.jpg
edited: 2018-11-14T12:00:00.000Z
title: Stalk a Click through a Re-Frame/actix_web App
published: true
description: An overview of a webapp using Rust and ClojureScript
tags: rust, clojure, beginners, webdevners, webdev
---
# Rollin' on 20s

## Why

I don't see a ton of material about either of these amazing tools, especially at the beginner level, and this is a solid way to get an overview of all the important bits.

## The Thing

It's a toy dice roller, hosted in this [github repo](https://github.com/deciduously/roll).  The user can submit either regular old dice rolls: `1d6, 2d10`, or define their awn aliases.  For example, you could save `3d8` to `goblin` and then just type `goblin`.  It also accepts a multiplier to repeat the roll: `2 2d6` or `27 goblin`.  Multiple commands are run in sequence: `2d9 goblin` is fine as a single command and will run both.

## The Tools

The backend uses [actix_web](https://actix.rs) to connect to a [SQLite](https://sqlite.org/index.html) database for storing the aforementioned aliases.  The frontend is implemented with the [Re-Frame](https://github.com/Day8/re-frame) framework which is built on top of [Reagent](https://reagent-project.github.io/), a method of defining React components in ClojureScript.  Re-Frame provides a functional state management solution - you're on your own for that with plain Reagent (not unlike React.js).

## The Journey

I'm going to examine just one pathway end to end - the click to submit a roll command.  We'll assume we've got the above example alias already defined: `goblin: 3d8`.  I'm not gonna throw 27 goblins at you on your first go, though, I'm not *Satan*...let's try the command `3 goblin` and see how we fare.

I'm zeroing in on Re-Frame and actix_web stuff, so some function bodies will be omitted in the interest of time.  This post managed to get pretty long anyway without digging into every line!  I'll throw in links to relevant files throughout for the full context.

### Gather the input

Re-Frame provides a rigid structure for defining different parts of your application.  We'll jump through a few of them, but our click's journey starts (as many do) with a textbox and a button.  All UI code is located in [views.cljs](https://github.com/deciduously/roll/blob/master/src/cljs/roll/views.cljs).

Reagent, if you're unfamiliar, is *awesome*.  Seriously, I'm even giving you [the link again](https://reagent-project.github.io/).  It takes a lot of ceremony out of React and distills components to their core.  Each component is a function, and it emulates [hiccup](https://github.com/weavejester/hiccup)'s syntax to allow you to define your HTML in the form of succinct Clojure vectors.

You use keywords like `:div` - a `[:div]` vector will expand to `<div></div>`.  Everything else is a child of that element.  Each vector optionally takes an options map as above, or an even quicker shorthand: `[:span#firstName.name.focus "SPAN!"]` expands to `<span id=\"firstName\" class=\"name focus\">SPAN!</span>`.  Lisps with all their tree-ness right out in the open like that are natural choices for representing and manipulating tree structures like the DOM.  Perfect for prototyping React apps!

Here's the specific component:

```clojure
(defn command-input
  "Command input"
  []
  [:div
   "Command: "
   [:input {:type "text"
            :id "field"
            :name "cmd"}]
   [:input {:type "button"
            :value "Submit"
            :on-click #(re-frame/dispatch
                        [::events/submit-command (-> (.getElementById js/document "field") .-value)])}]])
```

Potentially unfamiliar Clojure-ness aside, this is pretty easy to read.  When called in a Reagent tree, this funciton is a Reagent component that defines an input textboxes and a button, similar to code you'd write using any frontend tool.

Our trip starts (as so many do) when the user has entered some stuff and clicks the button.  The behavior is in the click handler:

```clojure
#(re-frame/dispatch [::events/submit-command (-> (.getElementById js/document "field") .-value)])`
```

`#()` defines an anonymous function in Clojure.  In JavaScript this looks like `() => {/* stuff */}`.  Any arguments are `%, %2, %3` etc if used: `#(%)` => `(fn [arg1] (arg1))`.  This one doesn't have any.

JS interop is dirt simple in ClojureScript.  We're calling `document.getElementById('field')`, rearranged so that Clojure-style the function is in the first position of the s-expression.  Subsequent arugments would follow `"field"`.  It's really that easy.  To access the value property of that element, you use the `.-value` syntax - otherwise CLJS will think you're trying to call a method `value()`.

This snippet uses the thread macro `->`, which works like a pipe.  It lets you write chained operations without nesting parens too deeply, which Lisps are notorious for.  Perhaps unnecessary with just two operations, but I find this more readable and consistency is always nice.

### Enter Re-Frame

This `submit-command` event is defined along with all the other events this application deals with, in [events.cljs](https://github.com/deciduously/roll/blob/master/src/cljs/roll/events.cljs).  Nice and neat.  This is what I love so much about working with Re-Frame.  Once you get your head around the model which is not as complicated as it sounds at first it's always unambiguous where any new code should go.  It's also got one of the best READMEs on GitHub, but that's just, like, my opinion, man.

Notice how we're not actually calling a function here to handle the event - we're passing a data structure containing the name of our event to `re-frame/dispatch` which is going to handle that for us in FIFO order.  Lets look at this event specifically:

```clojure
(re-frame/reg-event-fx
 ::submit-command
 (fn-traced [_ [_ cmd]]
   {:http-xhrio {:method :get
                 :uri (str "http://localhost:8080/roll/" (clojure.string/replace cmd #" " "/"))
                 :timeout 8000
                 :response-format (ajax/json-response-format {:keywords? true})
                 :on-success [::save-roll]
                 :on-failure [::bad-http-result]}}))
```

You create an event by registering its *effects* for the dispatcher with the aptly named `reg-event-fx` function.  Notice how we just give it the name and then immediately open a `fn` - not unlike the `defn` macro.  `fn-traced` just allows this event to plug in to the excellent [re-frame-10x](https://github.com/Day8/re-frame-10x) devtools - it's just a lambda otherwise.

The arguments to the `fn` are `[cofx event]`.  We're not using any co-effects yet. We will, don't fret, but for this event I'm ignoring them with `_`.  The `event` argument is then destructured.  Remember the `event` vector?  We made it ourselves a moment ago: `[::events/submit-command (-> (.getElementById js/document "field") .-value)]`.  That first part is just the name of the event, which we don't need - there's another `_` - and we're storing whatever the user entered as `cmd`.

This event leverages the officially supported [http-fx](https://github.com/Day8/re-frame-http-fx) library for performing AJAX requests.  This library provides the `:http-xhrio` effect handler.  This is also very straightforward to use - you pass it an options map with the request you're making.  It's got all the parts you'd expect to need to define.

Our specific `cmd` of `3 goblin` shows up in the URI, no surprises there.  We replace the space with a `/`: `http://localhost:8080/roll/3/goblins`.

This library has you specify the formats you're using - we're going JSON all the way.

Also of note is that we define both what happens on success (`200`) or on failure (anything else).  Both of these are simply other events defined in the same source file.  The Re-Frame dispatcher will call the proper follow-up once the response comes back.

However, before we can take a look at that, we've gotta actually generate the response!  Let's head on over to the backend and take a look at that `GET /roll` handler.

### Back of the House

The whole outline of our server is defined in [`main.rs`](https://github.com/deciduously/roll/blob/master/src/main.rs), beginning on line 78.

Actix comes with built-in support for CORS - any resource registered in this initial setup will gain the correct behavior automatically.  As with many Rust APIS, we're using a builder pattern to define the configuration of the app.  Once all the configuration is done, we finish it off with `register()`.  The resource in question is on [line 89](https://github.com/deciduously/roll/blob/dd747bb59b7d25ebe8a047d2f2d37f42e3f71bae/src/main.rs#L89):

```rust
.resource("/roll/{tail:.*}", |r| {r.method(http::Method::GET).with(roll)})
```

This defines the endpoint, specifies the method, and calls the specific handler `roll`.  `{tail:.*}` means that anything in the URL after `roll/` will be passed to the handler in the request as `tail`.  When a request hits the server, it tries each resource defined in succession.  If it matches this endpoint and method, this handler will be called from [`handlers.rs`](https://github.com/deciduously/roll/blob/master/src/handlers.rs):

```rust
// GET /roll/{cmd}
pub fn roll(req: HttpRequest) -> impl Responder {
    let cmd = &req.match_info()["tail"];
    let cmds = ((&cmd)
        .split('/')
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.to_string()))
        .collect::<Vec<String>>();
    roll_strs(&cmds)
}
```

I find actix_web extraordinarily ergonomic.  For one, it was an early embracer of the fancy-pants `impl Trait` syntax there in the return type.  In order to work as a handler, your function just needs to return any type that implements the `Responder` trait, and `actix_web` provides many out of the box, like for `String` and even `Json` (it's got [`serde`](https://serde.rs/) baked in).  Alternatively you can implement it yourself like we're about to do.

After getting `3/goblin` with `&req.match_info("tail")`, we just turn it into `vec!["3", "goblin"]` and pass it to `roll_strs()`.  This is our return value for `roll()`, so we know whatever this function returns will implement `Responder`.

### The Meat 'n' Potatoes

As promised `roll_strs()` returns a custom type, `Outcomes`, for which it's necessary to manually implement `Responder`:

```rust
#[derive(Serialize)]
pub struct Outcomes {
    pub outcomes: Vec<Outcome>,
}

pub fn roll_strs(s: &[String]) -> Outcomes {
    validate_input(s).unwrap().run()
}
```

The custom type is just a wrapper for a `Vec<Outcome>`.  This is an Outcome:

```rust
#[derive(Clone, Debug, Serialize)]
pub struct Outcome {
    roll: String,
    rolls: Vec<u32>,
}
```

These structs define the shape of our JSON response.  The response for `3 goblin` will be shaped like this:

```json
{"outcomes":
    [
        {"roll":"3d8","rolls":[6,1,3]},
        {"roll":"3d8","rolls":[7,1,8]},
        {"roll":"3d8","rolls":[8,1,5]}
    ]
}
```

`Responder` is not a difficult trait to implement.  It's only got one function, `respond_to`:

```rust
impl Responder for Outcomes {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, _req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = serde_json::to_string(&self)?;

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}
```

We can easily create `Json` from our custom types because of the `Serialize` trait we auto-derived - all it takes is `serde_json::to_string(&Outcomes)?`.  Then we build a successful `HttpResponse`, give it the expected `Content-Type`, and include our JSON string as the response body.  If we had been unable to build the json for whatever reason, the `?` at the end of `serde_json::to_string()` would have returned an `actix_web::Error` - this will also result in an `HttpResponse` going back to the user, but with an unsucessful code.

For brevity's sake I'll skip the machinery - there's nothing revolutionary about getting an `Outcome` from an input like `3d8` in Rust.  It's all housed in [`roll.rs`](https://github.com/deciduously/roll/blob/master/src/roll.rs) for the curious.

First, though, we've gotta grab `3d8` from `goblin`, and know to roll it three times.  The body of `roll_strs` calls runs us through the goodies in [`command.rs`](https://github.com/deciduously/roll/blob/master/src/command.rs) first.  Let's take a look.

### Command Parsing

First, we `validate_input(s)`.  Here's the signature - nothing fancy in the body:

```rust
pub fn validate_input(s: &[String]) -> io::Result<Command> {
   // parsing with regular expressions
}
```

In short, we look at the series of strings passed in and try to return a `Command`:

```rust
#[derive(Debug, PartialEq)]
pub enum Command {
    Roll(Vec<Roll>),              // One or more XdX args
    Multiplier(u32, Vec<String>), // an integer repeater, and then either rolls or lookups
    Lookup(Vec<String>),          // we get the roll from the db, there shouldn't be anything else
}
```

Our `3 goblin` example got parsed  by `validate_input()` to `Command::Multiplier(3, vec!["goblin"])`, which will in turn run a `Lookup("goblin")` three times.  Back up in `roll_strs()` we end things off by calling `run()` on the returned command.  This method returns our `Outcomes`:

```rust
impl Command {
    pub fn run(&self) -> Outcomes {
        match self {
            // a branch for each Command variant
        }
    }
}
```

`Multiplier` isn't terribly interesting - it'll run the `Lookup` command here three times, and the returned `Outcomes` will contain all three results.  Let's instead jump right to (the important parts of) `Lookup`:

```rust
Command::Lookup(ids) => {
                let conn = DB_POOL
                    .get()
                    .expect("Could not get db conn from thread pool");
                let items = get_items(&conn);
                let mut ret = Vec::new();
                for id in ids {
                    // look for each passed in item in the returned db items
                    // if found, get an Outcome from the associated roll and push it to ret
                    // log output
                }
                Outcomes { outcomes: ret } // return an Outcomes struct
```

### Goblin Hunting

Before we can interact with the database, we need to get a connection.  I'm using the [`r2d2`](https://github.com/sfackler/r2d2) crate to maintain a pool of open database connections instead of creating a new one for each request.  Here's the relevant code from [`db.rs`](https://github.com/deciduously/roll/blob/master/src/db.rs):

```rust
lazy_static! {
    pub static ref DB_POOL: Pool = init_pool();
}

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub const DATABASE_URL: &str = dotenv!("DATABASE_URL");

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<SqliteConnection>::new(DATABASE_URL);
    r2d2::Pool::new(manager).expect("failed to create pool")
}
```

This is standard r2d2 boilerplate that sets up a static `DB_POOL` using the database location defined in a `.env` file in the project folder.  To grab a connection, we use `let conn = DB_POOL.get()`.  One nice thing is that when `conn` goes out of scope at the end of this block the connection will be automatically returned to the pool for us.  We don't have to do anything about it ourselves.

Now we can call `get_items(&conn)` using this db connection.  I'm using the [`diesel`](https://http://diesel.rs) ORM:

```rust
pub fn get_items(conn: &SqliteConnection) -> Items {
    use schema::items::dsl::*;
    let results = items
        .limit(5)
        .load::<Item>(conn)
        .expect("Error loading items");

    let mut ret = Vec::new();
    for item in results {
        ret.push(item);
    }
    Items { items: ret }
}
```

The `Items` return type is a wrapper struct for a `Vec<Item>`.  The `Item` looks like this:

```rust
#[derive(Debug, Queryable, Serialize)]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub damage: String,
}
```

This exactly matches our database schema.  Diesel provides the `Queryable` trait, meaning it can marshall entries in our SQLite database to this Rust type for us automatically.  The `items` table was created with the following SQL:

```sql
CREATE TABLE items (
       id INTEGER NOT NULL PRIMARY KEY,
       title VARCHAR NOT NULL,
       damage TEXT NOT NULL
)
```

Diesel creates a DSL for us to compose queries using a Rustic API.  It's quite easy to use.

This particular example is grabbing *all* the items from the database, because the `Lookup` command may have multiple strings to look for.  This is pretty bad design (*cough* *Ben* *cough*).  I could optimize it to use syntax like this if there's only one:

```rust
let results = items
        .filter(title.eq(lookup_title))
        .load::<Item>(conn)
        .expect("Error loading items");
```

This would run a `SELECT * FROM items WHERE title = lookup_title`.

Bringing it all together, our `Lookup` for `goblin` returns something like:

```rust
Item {
    id: 1,
    title: "goblin",
    damage: "3d8",
}
```

The rest of the `Lookup` block in `Command::run()` just grabs that damage field and executes the roll, saving the result for the response.  Here's the example output again:

```json
{"outcomes":
    [
        {"roll":"3d8","rolls":[6,1,3]},
        {"roll":"3d8","rolls":[7,1,8]},
        {"roll":"3d8","rolls":[8,1,5]}
    ]
}
```

### Back Up Front

Whew!  Rust's bit is done - having found our database entry and used our custom `Responder` implementation to send back a JSON response, we've got to display it back the the user.

Recall that back in our Re-Frame event we defined both an effect for `:on-success` and `:on-failure`.  This roll was a *booming* success, so when this response comes back the Re-Frame dispatcher will trigger the `::save-roll` event back in `events.cljs`:

```clojure
(re-frame/reg-event-fx
 ::save-roll
 [(re-frame/inject-cofx :now) (re-frame/inject-cofx :temp-id)]
 (fn-traced [{:keys [db temp-id now]} [_ result]]
            {:db (update db :roll-hx conj {:id temp-id :time now :result result})}))
```

It's our good old friend `reg-event-fx` again, but this time there's a little bit more going on.  Remember when I mentioned and then completely dropped the concept of *co-effects*?  Before we open the lambda, we use `re-frame/inject-cofx` to add a little more data to the context `reg-event-fx` has available to work with than just the application db.  In Clojure, *eveything* is just data.  Kind of like before when the `event` passed in was just the vector we created, which could be destructured, `cofx` is just a Clojure map.  By default it contains our app's `db`, but we have the opportunity to put anything we want on it.  It's a *much* fancier name than concept, but I have to concede its a pretty accurate name.  Let's look at `:now`, the first co-effect we're injecting:

```clojure
(re-frame/reg-cofx
 :now
 (fn-traced [cofx _data]
            (assoc cofx :now (js/Date.))))
```

It looks not altogether unike `reg-event-fx`.  Essentially all it does is add a key to our `cofx` map with the key `:now`, and giving it the current date for a value.

Now, instead of blowing past it with an underscore, we destructure the `cofx` as well as the `event`:

```clojure
[{:keys [db temp-id now]} [_ result]]
```

The second part, `[_ result]`, is exactly what we did earlier with `cmd` - the first element of the vector is the name of the event (`::save-roll`), which we don't need, and `result` will hold the JSON we just generated in the backend.  The first part is our newly augmented `cofx` map.  We're specifically grabbing the values of the keys specified.  `db` is there already for us to use and represents the app state, and `now` is what we just injected - it's the current date.  `temp-id` is the other co-effect I registered - feel free to check it out in `events.cljs`.  It just allows us to assign session-local unique incrementing IDs to each incoming result by bumping an `atom` each time it's injected.

The body of this event just attaches a map containing this result along with the date and tempID our co-effects generated to the `:roll-hx` key in our app db using `conj`: `{:id temp-id :time now :result result}`.

### Bringing it on home

The rest happens automagically.  That's the end of our call chain - I don't have more code to follow. We did, though, change the database.  Re-Frame's got it from here - it will handle to page re-render picking up our newly augmented `:roll-hx` because we have a component *subscribed* to it.

Here's our main panel:

```clojure
(defn main-panel []
(let [result (re-frame/subscribe [::subs/results])
      error (re-frame/subscribe [::subs/error])
      items (re-frame/subscribe [::subs/items])]
  [:div
   [:h1 "ROLL"]
   [usage]
   "Roll history:  " [roll-hx @result] [:br]
   [command-input] [:br]
   "Items: " [all-items @items] [:br]
   [add-item] [:br]
   [view-error @error] [:hr]
   [footer]]))
```

The component in question is `[roll-hx @result]`.  This `result` is created up in the `let` binding using `re-frame/subscribe`.  All of our subscriptions live in [`subs.cljs`](https://github.com/deciduously/roll/blob/master/src/cljs/roll/subs.cljs).  Here's `::subs/results`:

```clojure
(re-frame/reg-sub
 ::results
 (fn [db]
   (:roll-hx db)))
```

Couldn't be simpler - it just returns the value of the `:roll-hx` key from our database.  When the app starts, we initialize this `db` as defined in [`db.cljs`](https://github.com/deciduously/roll/blob/master/src/cljs/roll/db.cljs):

```clojure
(def default-db
  {:name "re-frame"
   :roll-hx []
   :items []})
```

Our `::save-roll` event had the effect of attaching a new map to the `:roll-hx`.  Now it looks something like:

```clojure
{:name "re-frame"
   :roll-hx [{:id 0
              :time (js/Date.)
              :result {:outcomes [
        {:roll "3d8" :rolls [6,1,3]},
        {:roll "3d8" :rolls [7,1,8]},
        {:roll "3d8" :rolls [8,1,5]}
    ]]}
   ]}}]
   :items []})
```

Because the view is subscribed to the `:roll-hx` key of our database, it will automatically redraw to display the new data.  This is nice because the component doesn't need to know about the structure of your database - it's only concerned with that particular key.  If the database structure changes as you develop, you'd change the *subscription* logic - your view doesn't need to care.

There's nothing too surprising in the actual view - it renders this data as a list.  I won't go through the whole tree, it's pretty trivial stuff - there's no state here, it simply reflects the app db.  Here's the outer layer:


```clojure
(defn roll-hx
  "View full roll history"
  [hx]
  [:ul.hx
   (for [os (reverse hx)]
     ^{:key (:id os)}
  [:li [outcomes os]])])
```

Just functions all the way down.  We did the the thing, Re-frame style!  I'll leave killing the goblins as an exercise for the reader.
