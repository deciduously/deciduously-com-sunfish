---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--ivG7iUTZ--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://thepracticaldev.s3.amazonaws.com/i/rrbhyqlr8zanb3hpd3uh.JPG
date: 2019-05-28T12:00:00.000Z
title: "Skip the Framework: Build A Simple Rust API with Hyper"
description: A tutorial for building a Rust API with Hyper
tags:
  - rust
  - beginners
  - tutorial
  - webdev
---

## Intro

In this post I will walk through creating a small web API using the [hyper](https://github.com/hyperium/hyper) HTTP library. The app is an implementation of [todo-mvp](https://github.com/gypsydave5/todo-mvp), as introduced by David Wickes in his post: {# {% link gypsydave5/todo-mvp-or-why-you-shouldnt-use-a-web-framework---the-revenge-261l %} #}

One of the stipulations of the `todo-mvp` project is that each implementation should avoid "frameworks" and stick to libraries only. Framework is a nebulous term, and not necessarily always easy to delineate, so I went with the rule of thumb that if the crate documentation refers to itself as a framework, it's not appropriate for use. This greatly narrows down the available tooling, but as it turns out `hyper` is all you need to build an application like this without much incidental complexity.

Hyper is a lower level HTTP implementation. It provides Client and Server types and exposes the underlying [Tokio](https://tokio.rs/) asynchronous runtime it's built on top of. We'll also bring in a few other crates, but still nothing resembling a full-featured framework.

## Setup

You'll need to obtain a stable Rust toolchain. If you need one, see [rustup](https://rustup.rs/). Once installed, spin up a new executable project:

```
$ cargo new simple-todo
$ cd simple-todo
$ cargo run
   Compiling simple-todo v0.1.0 (/home/ben/code/simple-todo)
    Finished dev [unoptimized + debuginfo] target(s) in 1.30s
     Running `target/debug/simple-todo`
Hello, world!
```

Open your new `simple-todo` directory in your favorite editor. Before diving into code, let's define our dependencies. Make your `Cargo.toml` look like this:

```toml
[package]
name = "simple-todo"
version = "0.1.0"
authors = ["You <you@yourcoolsite.com>"]
edition = "2018"

[dependencies]
futures = "0.1"
hyper = "0.12"
lazy_static = "1.3"
log = "0.4"
pretty_env_logger = "0.3"
serde = "1.0"
serde_derive = "1.0"
tera = "0.11"

[dependencies.uuid]
features = ["serde", "v4"]
version = "0.7"
```

In addition to `hyper`, we're using a couple extra helper crates. In brief, `futures` provides zero-cost asynchronous programming primitives, `lazy_static` will let us define `static`s that require runtime initialization (like `Vec::new()`), `log` and `pretty_env_logger` provide logging, `serde` and `serde_derive` are for serialization, `tera` performs HTML templating from Jinja-like template files, and `Uuid` provides, well, uuids! These crates provide our basic building blocks.

This is a small program which will be defined entirely in `main.rs`. Open that file and remove the `println!` statement from the `cargo new` template and spin up the logging instead:

## Entrypoint

```rust
fn main() {
    pretty_env_logger::init();
}
```

Note that in Rust 2018 we can omit `extern crate` declarations unless we need to import a macro.

Before we can set up the server, we need an address to bind to. We'll just hardcode it for this demo. Add this line right below the init:

```rust
let addr = "127.0.0.1:3000".parse().unwrap();
```

The `parse()` method will return a [`std::net::SocketAddr`](https://doc.rust-lang.org/std/net/enum.SocketAddr.html).

Next, we'll need to pull in a few imports at the top of the file:

```rust
use futures::{future, Future, Stream};
use hyper::{
    client::HttpConnector, rt, service::service_fn, Body, Client, Request,
    Response, Server
};
```

Now we can finish off `main()`:

```rust
    rt::run(future::lazy(move || {
        // create a Client for all Services
        let client = Client::new();

        // define a service containing the router function
        let new_service = move || {
            // Move a clone of Client into the service_fn
            let client = client.clone();
            service_fn(move |req| router(req, &client))
        };

        // Define the server - this is what the future_lazy() we're building will resolve to
        let server = Server::bind(&addr)
            .serve(new_service)
            .map_err(|e| eprintln!("Server error: {}", e));

        println!("Listening on http://{}", addr);
        server
    }));
```

This won't quite typecheck - to get it to compile, you can add the following stub above for the `router` function we reference in the `service_fn` call:

```rust
fn router(req: Request<Body>, _client: &Client<HttpConnector>) -> Box<Future<Item = Response<Body>, Error = Box<dyn std::error::Error + Send + Sync>> + Send> {
    unimplemented!()
}
```

This is all a little beefier, let's unpack it. This whole tidbit lives inside a call to `rt:run()`. Here `rt` stands for runtime, and refers to the default Tokio runtime. Immediately our program is going to spin up and enter this async environment.

Inside, we call `future::lazy`, which accepts a closure and returns a `Future` that will resolve to it. The rest of the definition is in this closure, and has a few steps. We build a hyper `Client`, capable of making outgoing HTTP requests.

The next order of business is to create a `Service`. This is a trait representing an asynchronous function of a request to a response - exactly what our web server needs to handle! Instead of implementing this trait by hand, we're just going to define this function oursleves (in this case, it's `router()`), and use the `service_fn` helper to convert the function to a `Service`. Then all we need to do is create the `Server` itself, which binds to the address we provided, and have it serve this service.

That's _pretty much it_. Now our job is just defining the responses, which is your job anyway, framework or no!

## Router

First, though, take a look at that `router()` signature. Gross, right? Make a few type aliases under your imports:

```rust
type GenericError = Box<dyn std::error::Error + Send + Sync>;
type ResponseFuture = Box<Future<Item = Response<Body>, Error = GenericError> + Send>;


fn router(req: Request<Body>, _client: &Client<HttpConnector>) -> ResponseFuture {
    unimplemented!()
}
```

Any time we want to give a response back to a connection, it's gotta be given as a `Response` wrapped up in a `Future` wrapped up in a `Box` - it's definitely a good idea to make that easier to type! Now we can start defining routes. Before getting started, add `Body`, `Method`, and `StatusCode` to the list of `hyper` imports.

We can leverage Rust pattern matching to correctly dispatch responses:

```rust
match (req.method(), req.uri().path()) {
    (&Method::GET, "/") | (&Method::GET, "index.html") => unimplemented!(),
    _ => four_oh_four(),
    }
```

We're matching on both the method and the path at once - a POST request to "/" would not match this branch. We can add as many match arms as the app requires here, and any incoming request that doesn't have a corresponding arm will get the `four_oh_four()` response:

```rust
static NOTFOUND: &[u8] = b"Oops! Not Found";

fn four_oh_four() -> ResponseFuture {
    let body = Body::from(NOTFOUND);
    Box::new(future::ok(
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body)
            .unwrap(),
    ))
}
```

As expected, this function returns a `ResponseFuture`. For the 404 page, we'll just use this static value as the body. The `future::ok` returns a future which immediately resolves, and we use the builder pattern to build a `Response`. There are `hyper` enums set up for things like `StatusCode` for maximum correctness!

## HTML

To build an index page, we'll use [tera](https://github.com/Keats/tera) which provides Jinja2-like HTML templates. We are going to need a macro, and this will be set up as a static, so we need a few declarations:

```rust
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate tera;

// ...

use tera::{Context, Tera};
```

The `todo-mvp` project requires each implementation use the same template. This post isn't about Jinja2 or HTML, so I'm just gonna direct you to download it [here](https://github.com/gypsydave5/todo-mvp/blob/master/todos/rust/templates/index.html) and save it to `simple-todo/templates/index.html`. You'll also want to save [`todo.css`](https://github.com/gypsydave5/todo-mvp/blob/master/todos/rust/src/resource/todo.css) to `simple-todo/src/resource/todo.css`.

Tera is incredibly easy to use. Add the following snippet:

```rust
lazy_static! {
    pub static ref TERA: Tera = compile_templates!("templates/**/*");
}
```

Voila, templates. Now we can write `index()`:

```rust
fn index() -> ResponseFuture {
    let mut ctx = Context::new();
    let body = Body::from(TERA.render("index.html", &ctx).unwrap().to_string());
    Box::new(future::ok(Response::new(body)))
}
```

To inject data into a Tera template, you put it in a `tera::Context` and pass both the template path and this context to `render()`. Then we just wrap up the resulting string in a `ResponseFuture`! Don't forget to update the match arm in `router()` to call this function instead of `unimplemented!()`.

## State

There's a problem, though - we haven't actually put any data in the context! If you ran this program it'd crash when loading this template, complaining that `todos` and `todosLen` are not found in the context. It's an incredibly valid complaint, they're not there.

Keeping track of state in an asynchronous application like this one _could_ be a complicated problem, but this is Rust. We've got [`std::sync`](https://doc.rust-lang.org/std/sync/) to play with! Specifically, we're going to use the combination of [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html) and [`RwLock`](https://doc.rust-lang.org/std/sync/struct.RwLock.html) to store our todos safely across threads without really even thinking about it.

First, the import additions:

```rust
#[macro_use]
extern crate serde_derive;

// ...

use std::sync::{Arc, RwLock};
use uuid::Uuid;
```

Now, the Todo type:

```rust
#[derive(Debug, Serialize)]
pub struct Todo {
    done: bool,
    name: String,
    id: Uuid,
}

impl Todo {
    fn new(name: &str) -> Self {
        Self {
            done: false,
            name: String::from(name),
            id: Uuid::new_v4(),
        }
    }
}
```

The `new_v4()` method will randomly generate a unique identifier for any new `Todo`. Also add a new type alias for the list of all todos:

```rust
type Todos = Arc<RwLock<Vec<Todo>>>;
```

Now we can instantiate it in the `lazy_static!` block:

```rust
lazy_static! {
    pub static ref TERA: Tera = compile_templates!("templates/**/*");
    pub static ref TODOS: Todos = Arc::new(RwLock::new(Vec::new()));
}
```

We'll need a few helper functions to manipulate the list:

```rust
fn add_todo(t: Todo) {
    let todos = Arc::clone(&TODOS);
    let mut lock = todos.write().unwrap();
    lock.push(t);
}

fn remove_todo(id: Uuid) {
    let todos = Arc::clone(&TODOS);
    let mut lock = todos.write().unwrap();
    // find the index
    let mut idx = lock.len();
    for (i, todo) in lock.iter().enumerate() {
        if todo.id == id {
            idx = i;
        }
    }
    // remove that element if found
    if idx < lock.len() {
        lock.remove(idx);
    }
}

fn toggle_todo(id: Uuid) {
    let todos = Arc::clone(&TODOS);
    let mut lock = todos.write().unwrap();
    for todo in &mut *lock {
        if todo.id == id {
            todo.done = !todo.done;
        }
    }
}
```

When you call `Arc::clone()`, it creates a new pointer to the same data, increasing its reference count. Then we grab a write lock on the underlying `RwLock`, after which we can safely manipulate the `Vec` inside. Using these helpers, our route handlers can manipulate the state safely in a manner that's guaranteed to be thread-safe. Finally we can build the context, back in `index()` right after you define `ctx`:

```rust
let todos = Arc::clone(&TODOS);
let lock = todos.read().unwrap();
ctx.insert("todos", &*lock);
ctx.insert("todosLen", &(*lock).len());
```

## Handlers

Now running the app and pointing your browser to `localhost:3000` should display the given HTML (sans stylesheet).

The rest of the app is easy. We simply need to fill out the the rest of the handlers. For instance, to load the missing stylesheet, you need a new match arm:

```rust
(&Method::GET, "/static/todo.css") => stylesheet(),
```

As well as a function to build the response:

```rust
fn stylesheet() -> ResponseFuture {
    let body = Body::from(include_str!("resource/todo.css"));
    Box::new(future::ok(
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/css")
            .body(body)
            .unwrap(),
    ))
}
```

Nothing surprising in there! Each todo list manipulation also has an endpoint:

```rust
(&Method::POST, "/done") => toggle_todo_handler(req),
(&Method::POST, "/not-done") => toggle_todo_handler(req),
(&Method::POST, "/delete") => remove_todo_handler(req),
(&Method::POST, "/") => add_todo_handler(req),
```

These handlers all take the same format:

```rust
fn add_todo_handler(req: Request<Body>) -> ResponseFuture {
    Box::new(
        req.into_body()
            .concat2() // concatenate all the chunks in the body
            .from_err() // like try! for Result, but for Futures
            .and_then(|whole_body| {
                let str_body = String::from_utf8(whole_body.to_vec()).unwrap();
                let words: Vec<&str> = str_body.split('=').collect();
                add_todo(Todo::new(words[1]));
                redirect_home()
            }),
    )
}
```

This is a little more complicated. We need to read the request and then act on it. In this case, the request body stored in `str_body` will look something like `item=TodoName`. There are more robust solutions, but I'm just splitting on the `=` and calling the `add_todo` function on the result to add it to the list. Then we redirect to home, and every time we go back home `index()` is called, which rebuilds the HTML from whatever the current app state is! The `toggle` and `remove` handlers are nearly equivalent, just calling the proper function.

The redirect is also not surprising:

```rust
fn redirect_home() -> ResponseFuture {
    Box::new(future::ok(
        Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(header::LOCATION, "/")
            .body(Body::from(""))
            .unwrap(),
    ))
}
```

This looks like what you'd write in any toolkit. The app also includes some SVG:

```rust
 (&Method::GET, path_str) => image(path_str),
```

```rust
fn image(path_str: &str) -> ResponseFuture {
    let path_buf = PathBuf::from(path_str);
    let file_name = path_buf.file_name().unwrap().to_str().unwrap();
    let ext = path_buf.extension().unwrap().to_str().unwrap();

    match ext {
        "svg" => {
            // build the response
            let body = {
                let xml = match file_name {
                    "check.svg" => include_str!("resource/check.svg"),
                    "plus.svg" => include_str!("resource/plus.svg"),
                    "trashcan.svg" => include_str!("resource/trashcan.svg"),
                    "x.svg" => include_str!("resource/x.svg"),
                    _ => "",
                };
                Body::from(xml)
            };
            Box::new(future::ok(
                Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "image/svg+xml")
                    .body(body)
                    .unwrap(),
            ))
        }
        _ => four_oh_four(),
    }
}
```

That's the whole enchilada. To add more routes, you just add a new match arm to `router()` and write a function that returns a `ResponseFuture`. This is a solid , performant base that's easily extensible in myriad ways, because you're not beholden to any specific predetermined pattern. All in all, writing a server using plain `hyper` instead of a higher-level framework isn't really that much less ergonomic for simple use cases, and cuts a serious amount of overhead from your app. My current favorite framework is `actix-web` but it's almost absurd how much bigger the binaries are and how much longer a cold compile takes. When the end goal is simple enough, why not use simple tools?

The full implementation can be found [here](https://github.com/gypsydave5/todo-mvp/tree/master/todos/rust/src).
