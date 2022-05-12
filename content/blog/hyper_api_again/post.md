---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--BYmHqqV1--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://dev-to-uploads.s3.amazonaws.com/uploads/articles/njkcjz38dnroakrvb5zj.jpg
date: 2021-11-21T12:00:00.000Z
title: Oops, I Did It Again...I Made A Rust Web API And It Was Not That Difficult 
tags:
  - rust
  - beginners
  - tutorial
  - webdev
---
Over two years ago (oof), I posted [a walkthrough](https://dev.to/deciduously/skip-the-framework-build-a-simple-rust-api-with-hyper-4jf5) of my Rust implementation of [todo-mvp](https://github.com/gypsydave5/todo-mvp) by @gypsydave5 demonstrating how to build a simple Rust API without a framework.  The core functionality was built using [hyper](https://hyper.rs), a lower-level HTTP library instead of a full-blown framework.

It turns out I wrote that post about six months too early.  I published it in May 2019, and in November, Rust released 1.39.0 containing `async/await` syntax.  Womp womp.

My intent here was to simply upgrade the existing application to use the new syntax where applicable and call it a day.  But, you know that thing that happens when you go back and review code you wrote years ago?  Yeah, that thing happened.  We're starting from scratch.

## What's New

Our result functions almost identically to the implementation in the previous post, so some of this code will look very similar. Here's a quick overview of the new stuff you'll find in this post not present in the previous version:

* [anyhow](https://docs.rs/anyhow/1.0.47/anyhow/) - Error handling for humans.
* [async/await](https://rust-lang.github.io/async-book/) - New syntax for expressing asynchronous computation - like a webserver!
* [catch_unwind](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html) - A panicking task shouldn't crash your whole server!  Gracefully catch the panic and keep on servin'.
* [compression](https://docs.rs/flate2/1.0.22/flate2/) - Every response will be DEFLATE compressed, simply because we can.
* [Rust 2021](https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html) - _the future of Rust_.
* state management - Instead of a global variable, we'll use [protocol extensions](https://docs.rs/hyper/0.14.15/hyper/struct.Request.html#method.extensions) to access the app state.
* tracing - The [`log`](https://docs.rs/log/0.4.14/log/) crate is old news. All the cool kids use [`tracing`](https://docs.rs/tracing/0.1.29/tracing/) now.
* unit testing - We didn't have _any_ last time - tsk tsk!  Learn how to write [async unit tests](https://docs.rs/tokio/1.14.0/tokio/attr.test.html) for your handlers.

## Setup

To follow along, you'll need a stable Rust toolchain.  See the [install page](https://www.rust-lang.org/tools/install) for instructions to install `rustup` for your platform.  You should prefer this method to your distribution's package manager.  If you're a NixOS freak, I recommend [fenix](https://github.com/nix-community/fenix).

Once your environment is set up, start a new project and make sure you can build it:

```txt
$ cargo new simple-todo
   Created binary (application) `simple-todo` package
$ cd simple-todo
$ cargo run
   Compiling simple-todo v0.1.0 (/home/deciduously/code/simple-todo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/simple-todo
Hello, world!

$
```

Open up your `Cargo.toml` and make it look like this:

```toml
[package]
authors = ["Cool Person <cool.person@yourcoolsite.neato>"]
edition = "2021"
rust-version = "1.56"
name = "simple-todo"
version = "0.1.0"

[dependencies]
anyhow = "1"
backtrace = "0.3"
clap = {version = "3.0.0-beta.5", features = ["color"] }
flate2 = "1"
futures = "0.3"
hyper = { version = "0.14", features = ["full"] }
lazy_static = "1.4"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
tera = "1"
tokio = { version = "1", features = ["full"] }
uuid = { version = "0.8", features = ["serde", "v4"] }

[dev-dependencies]

pretty_assertions = "0.7"
select = "0.5"
```

There are some new elements here already, up in the package section.  [Rust 2021](https://doc.rust-lang.org/edition-guide/rust-2021/index.html) was recently released, and along with it, the `rust-version` metadata key, allowing you to specify the Minimum Supported Rust Version (MSRV) directly in the package.

This post is only concerned with Rust and will use assets identical to the last post.  Create a folder called `templates` at the project's top-level, and place [this index.html](https://github.com/gypsydave5/todo-mvp/blob/master/todos/rust/templates/index.html) inside.  You will also need to create a directory at `src/resource` and fill it with [these files](https://github.com/gypsydave5/todo-mvp/tree/master/todos/rust/src/resource).  There is a stylesheet and a handful of SVG files.  Your structure should look like this:

```txt
$ tree
.
├── Cargo.lock
├── Cargo.toml
├── src
│  ├── main.rs
│  └── resource
│     ├── check.svg
│     ├── plus.svg
│     ├── tick.png
│     ├── todo.css
│     ├── trashcan.svg
│     └── x.svg
└── templates
   └── index.html
```

Good to go!

## Entrypoint

This whole app will live in `src/main.rs`.  Start off by adding the imports:

```rust
use anyhow::Result;
use backtrace::Backtrace;
use clap::Parser;
use flate2::{write::ZlibEncoder, Compression};
use futures::{future::FutureExt, Future};
use hyper::http;
use lazy_static::lazy_static;
use serde::Serialize;
use std::{
    cell::RefCell,
    convert::Infallible,
    io::Write,
    panic::AssertUnwindSafe,
    path::PathBuf,
    sync::{Arc, RwLock},
};
use tera::Tera;
use uuid::Uuid;
```

It's nice to allow the user to specify where to run their app.  The `clap` library provides a convenient way to specify command-line arguments in a struct.  This feature is still in beta but will stabilize soon.  We can create a struct and use the `Parser` feature to generate the options:

```rust
#[derive(Parser)]
#[clap(version = concat!(env!("CARGO_PKG_VERSION")), about = "Serve a TODO list application.")]
struct Args {
    #[clap(
        short,
        long,
        about = "Address to bind the server to.",
        env,
        default_value = "0.0.0.0"
    )]
    address: String,
    #[clap(short, long, about = "Port to listen on.", env, default_value = "3000")]
    port: u16,
}
```

When run without any arguments, the server will bind to `0.0.0.0:3000`, a reasonable default.  This allows users to either use the `ADDRESS` and `PORT` environment variables or `-a/--address` and `-p/--port` command-line arguments.  It also provides a nice `--help` implementation:

```txt
simple-todo 0.1.0

Serve a TODO list application.

USAGE:
    todo-mvp-rust [OPTIONS]

OPTIONS:
    -a, --address <ADDRESS>    Address to bind the server to. [env: ADDRESS=] [default: 0.0.0.0]
    -h, --help                 Print help information
    -p, --port <PORT>          Port to listen on. [env: PORT=] [default: 3000]
    -V, --version              Print version information
```

Our `main()` function will just parse these arguments and pass them off to our app routine:

```rust
fn main() -> Result<()> {
    let args = Args::parse();
    app(args)?;
    Ok(())
}
```

I've brought [`anyhow::Result`](https://github.com/dtolnay/anyhow) into scope, making error handling super easy to use.  We don't need to specify all our `Error` types. It can automatically convert any errors that implement `std::error::Error`, which should be all of them.  If an error propagates all the way up to `main()`, we'll get all the info it's captured printed to stdout.

The `app()` function is our _real_ entrypoint:

```rust
#[tokio::main]
async fn app(args: Args) -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = std::net::SocketAddr::new(args.address.parse()?, args.port);

    let todos = Todos::new();
    let context = Arc::new(RwLock::new(todos));

    serve(addr, context, handle).await?;

    Ok(())
}
```

This function is tagged with `#[tokio::main]`, meaning it will execute an async runtime.  All the functions we're using now can be marked `async`, meaning we can `await` the result.  Under the hood, Rust converts these functions into constructs called Futures, and in the previous iteration we built these manually.  Futures can either be `Ready` or `Waiting`.  You can call the `poll()` method on a future, which just asks: "are you ready, or are you waiting?" If the future is ready to resolve, it will pass up the response, and if it's waiting, it will just respond with `Pending`.  It can return a `wake()` callback, letting the caller know there's a value.  When `wake` is executed, the caller will know to `poll()` this future again, resolving it to a value.

This all involved a lot of ceremony.  This `async/.await` syntax abstracts these concepts for us.  We just write functions like normal, except we mark them `async`, and when control flow hits a point where we need to wait for a result to resolve, we can use `await`.  If these are used within an executor like `#[tokio::main]`, the runtime will cover all the details.  Instead of blocking control flow, each of these `await` points will yield to other running tasks on this executor until the underlying `Future` returns from a `poll()` request as `Ready<T>`.  This makes our types easier to reason about and our code much more straightforward to write.

This top-level function reads our argument struct to build the `SocketAddr` to bind to, starts up the logging system with [`tracing_subscriber`](https://docs.rs/tracing-subscriber/0.3.2/tracing_subscriber/index.html), and builds our state management.  Then, we call the `serve()` async function and `await` on the result.  This executor will run forever until the user kills the process and can handle multiple concurrent connections for us seamlessly.

For a sneak preview, this is the signature of `serve()`:

```rust
async fn serve<C, H, F>(
    addr: std::net::SocketAddr,
    context: Arc<C>,
    handler: H,
) -> hyper::Result<()>
where
    C: 'static + Send + Sync,
    H: 'static + Fn(Request) -> F + Send + Sync,
    F: Future<Output = Response> + Send,
{
    // ...
}
```

It requires we pass an address, then an `Arc<C>`.  That `C` type is our app state, and for this to work, it must implement the [`Send` and `Sync`](https://doc.rust-lang.org/nomicon/send-and-sync.html) traits.  `Send` means the value can be sent to another thread, and `Sync` means it can be shared between threads simultaneously.

In our case, we don't need to think too hard about this.  We're using an `Arc<RwLock<T>>` to allow mutation to provide a type that can be accessed from multiple tasks and safely mutated as needed.  As long as each task grabs the proper lock, we don't need to worry about concurrent tass clobbering each other.  Only one task will be able to write to this type at a time, so every new reader will always have the correct data.

Finally, we need to add a handler with type `H`.  These types start to peel back a little of what `async` is doing for us.  Stripping out the `Send + Sync` trait bounds, this function satisfies the trait bound `Fn(Request) -> Future<Output = Response>`.  Because we're in an async environment, we can just write `async fn handle(request: Request) -> Response` - it's an asynchronous function from a request to a response.  Sounds like a web server to me!  Using Rust's `async/.await`, we get to simply write what we mean.

We'll come back to the handler shortly - first, we have a little bit of setup to take care of.

## Templates

This application only consists of a single page that will be refreshed whenever the state changes.  We placed markup at `templates/index.html`, an HTML file using [Jinja](https://jinja.palletsprojects.com/en/3.0.x/)-style templating. We'll use [`Tera`](https://tera.netlify.app/) to handle this in Rust.

The templates need to be compiled before use, but this only needs to happen once.  We can use [`lazy_static`](https://github.com/rust-lang-nursery/lazy-static.rs) to ensure this compilation happens the first time the templates are accessed, and then reuse the compiled result for all subsequent access:

```rust
lazy_static! {
    pub static ref TERA: Tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Unable to parse templates: {}", e);
            std::process::exit(1);
        }
    };
}
```

Now we can use the `TERA` global.  If Tera could not compile the templates for any reason, the process will exit here and display the error.  Our server only works if this step completes successfully.

## State

Next, we need to define the app state.  The application revolves around a `Todo` type with a name, an ID, and a boolean tracking whether it's been completed:

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

We just need to provide a string name, like `Todos::new("Task")`, and this type will generate a new unique ID and set it to incomplete.

Storage is pretty simple:

```rust
#[derive(Debug, Default)]
struct Todos(Vec<Todo>);
```

We need methods to add new todos, remove existing todos, and toggle the `done` boolean:

```rust
impl Todos {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, todo: Todo) {
        self.0.push(todo);
    }

    pub fn remove(&mut self, id: Uuid) -> Option<Todo> {
        let mut idx = self.0.len();
        for (i, todo) in self.0.iter().enumerate() {
            if todo.id == id {
                idx = i;
            }
        }
        if idx < self.0.len() {
            let ret = self.0.remove(idx);
            Some(ret)
        } else {
            None
        }
    }

    pub fn todos(&self) -> &[Todo] {
        &self.0
    }

    pub fn toggle(&mut self, id: Uuid) {
        for todo in &mut self.0 {
            if todo.id == id {
                todo.done = !todo.done;
            }
        }
    }
}
```

Notably, we don't have to write any unique code to make this thread safe.  We can write it exactly as we would in a single-threaded, synchronous context and trust that Rust won't let us mutably access this unsafely.  This is what got instantiated up in `app()` with these lines:

```rust
let todos = Todos::new();
let context = Arc::new(RwLock::new(todos));
```

Wrapping this all in an `Arc` means that any task accessing this value can get their own reference to it, and the `RwLock` will allow multiple concurrent readers _or_ exactly one writer at a time.  When the lock is released, the next waiting task will be able to take control.

## Handler

We're finally ready to take a look at the handler.  Per the signature up above, we know we need a function with the following signature: `async fn handle(request: Request) -> Response`.  Each time our web server receives an HTTP request, it will call this function to produce a Response to send back.  First, add type aliases for the request and response:

```rust
type Request = http::Request<hyper::Body>;
type Response = http::Response<hyper::Body>;
```

The `hyper` crate defines all the types we need.  In both cases, the request and response will carry a [`hyper::Body`](https://docs.rs/hyper/0.14.15/hyper/body/struct.Body.html).  Every single handler will use this type signature, so defining these aliases saves us a lot of typing.

The request contains information about how it was sent.  We can use Rust's `match` construct to read both the incoming URI and HTTP method to correctly dispatch a response. Here's the whole body:

```rust
async fn handle(request: Request) -> Response {
    // pattern match for both the method and the path of the request
    match (request.method(), request.uri().path()) {
        // GET handlers
        // Index page handler
        (&hyper::Method::GET, "/") | (&hyper::Method::GET, "/index.html") => index(request).await,
        // Style handler
        (&hyper::Method::GET, "/static/todo.css") => stylesheet().await,
        // Image handler
        (&hyper::Method::GET, path_str) => image(path_str).await,
        // POST handlers
        (&hyper::Method::POST, "/done") => toggle_todo_handler(request).await,
        (&hyper::Method::POST, "/not-done") => toggle_todo_handler(request).await,
        (&hyper::Method::POST, "/delete") => remove_todo_handler(request).await,
        (&hyper::Method::POST, "/") => add_todo_handler(request).await,
        // Anything else handler
        _ => four_oh_four().await,
    }
}
```

Each match arm will match with a specific combination of HTTP verb and path.  For example, `GET /static/todo.css` will properly dispatch the `stylesheet()` handler, but `POST /static/todo.css` is not supported and will fall through to `four_oh_four()`.  Each of these handlers is itself an `async` function, but we don't want to return up to the caller until they've been polled as `ready`, and return an actual `Response`.  Remember, Rust is doing this for us - when we write `async fn() -> Response`, we actually get a `Fn() -> impl Future<Output = Response>`.  We can't use that return type until the future resolves! That's what the `.await` syntax signifies.  Once the future is ready, we'll use the resulting `Response` output but not before.

The most straightforward handler is `four_oh_four()`:

```rust
async fn four_oh_four() -> Response {
    html_str_handler("<h1>NOT FOUND!</h1>", http::StatusCode::NOT_FOUND).await
}
```

This response doesn't depend on the request - the request didn't make any sense to us, after all! There's no input parameter, but like all handlers, it produces a `Response`.  Because all of our routes need to build responses, I pulled this logic out into a series of building-block functions.

## Response Builders

Most of our `Response` building shares a lot of logic. We usually send back some form of string, and we want to attach a content type and a status code.  Because we care about our user's bandwidth usage, we also want to compress our responses, ensuring as little as possible is sent over the wire.  The most common case is a successful response containing HTML:

```rust
async fn ok_html_handler(html: &str) -> Response {
    html_str_handler(html, http::StatusCode::OK).await
}
```

This, in turn, calls the HTML string handler:

```rust
async fn html_str_handler(html: &str, status_code: http::StatusCode) -> Response {
    string_handler(html, "text/html", status_code).await
}
```

Our `four_oh_four()` handler used this directly to include a different status code.  Ultimately, though, it's all just strings:

```rust
async fn string_handler(body: &str, content_type: &str, status_code: http::StatusCode) -> Response {
    bytes_handler(body.as_bytes(), content_type, status_code).await
}

async fn ok_string_handler(body: &str, content_type: &str) -> Response {
    string_handler(body, content_type, hyper::StatusCode::OK).await
}
```

These helpers allow for other content types as long as the body is still passed as a string.  This covers all the body types we need for this application.  At the bottom, we get to `bytes_handler`:

```rust
async fn bytes_handler(body: &[u8], content_type: &str, status_code: http::StatusCode) -> Response {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(body).unwrap();
    let compressed = encoder.finish().unwrap();
    hyper::Response::builder()
        .status(status_code)
        .header(hyper::header::CONTENT_TYPE, content_type)
        .header(hyper::header::CONTENT_ENCODING, "deflate")
        .body(hyper::Body::from(compressed))
        .unwrap()
}
```

This function takes a byte slice (`&[u8]`), and `DEFLATE` compresses it.  It adds the proper `Content-Encoding` header so that any connected client can uncompress the payload before presenting it back to the user.  This is a rapid operation, and the smaller your payloads, the better.  Every response that contains a body will eventually pass through this function before bubbling back up to the top-level `handle()` function and back to the client.

There's one more response type for this application, and this doesn't use a body at all.  Whenever our app state changes, we will use the `301` HTTP status code to trigger the client to redirect back to the index.  Every time the index is rendered, it reads the current app state, meaning any mutation executed in the handler will be automatically reflected on the refresh.  This function calls `Response::builder()` directly:

```rust
async fn redirect_home() -> Response {
    hyper::Response::builder()
        .status(hyper::StatusCode::SEE_OTHER)
        .header(hyper::header::LOCATION, "/")
        .body(hyper::Body::empty())
        .unwrap()
}
```

## Main page

Now, we have everything we need to render the app.  Our index function is the first handler that requires state:

```rust
async fn index(request: Request) -> Response {
    // Set up index page template rendering context
    let mut tera_ctx = tera::Context::new();
    let todos_ctx: Arc<RwLock<Todos>> = Arc::clone(request.extensions().get().unwrap());
    {
        let lock = todos_ctx.read().unwrap();
        let todos = lock.todos();
        let len = todos.len();
        tera_ctx.insert("todos", todos);
        tera_ctx.insert("todosLen", &len);
    }
    let html = TERA.render("index.html", &tera_ctx).unwrap().to_string();
    ok_html_handler(&html).await
}
```

In the previous iteration of this app, the `Todos` struct was instantiated alongside our `TERA` templates in a global static variable.  A better solution is to thread it through to our handler using the `Request` itself. We'll look at the implementation lower down, but by the time we get here, there's a fresh `Arc` containing the context ready to be read.  We can use `request.extensions().get()` and `Arc::clone()` to grab our very own reference to the app state to use for building this response.  The request extensions use the type of what's stored for access, so we need to explicitly add the type of `todos_ctx` to indicate what we're looking for.

Next, we build the index page using the current state of the app.  This handler won't perform any mutation, so we can use `todos_ctx.read()`.  By introducing a child scope, we ensure our read lock gets dropped when we're done with it, allowing any writers waiting for access to grab their own locks.  If we needed to wait, no problem! We're in an `async` function, the caller can poll us any time, and we'll just return `Pending` until we're ready to go.  Nice and neat.

Once we've received our handle to the app state, we can pass it through to Tera.  `TERA.render()` will return an HTML string with all template values resolved using our app state.  Then, we can use our trusty `ok_html_handler()` response builder to tag it with a proper content type and status code and compress the result before returning to the caller.

The `index.html` template will request a style sheet when it loads from `/static/todo.css`. That's a pretty simple handler:

```rust
async fn stylesheet() -> Response {
    let body = include_str!("resource/todo.css");
    ok_string_handler(body, "text/css").await
}
```

The `include_str!()` macro actually bundles the string content directly in your compiled binary.  These files need to be present at compile-time but do not need to be distributed in production.  The compiled binary already includes everything it needs.

## SVG

All the image assets in this application are SVG, which is represented as XML.  This means we just need to read these strings and pass the proper content-type:

```rust
async fn image(path_str: &str) -> Response {
    let path_buf = PathBuf::from(path_str);
    let file_name = path_buf.file_name().unwrap().to_str().unwrap();
    let ext = match path_buf.extension() {
        Some(e) => e.to_str().unwrap(),
        None => return four_oh_four().await,
    };

    match ext {
        "svg" => {
            // build the response
            let body = match file_name {
                "check.svg" => include_str!("resource/check.svg"),
                "plus.svg" => include_str!("resource/plus.svg"),
                "trashcan.svg" => include_str!("resource/trashcan.svg"),
                "x.svg" => include_str!("resource/x.svg"),
                _ => "",
            };
            ok_string_handler(body, "image/svg+xml").await
        }
        _ => four_oh_four().await,
    }
}
```

I just used a catch-all - any GET request that's not for the index or the stylesheet is assumed to be for an image.  At the top, there's a little extra logic to make sure we're looking for an image file. If there's no extension, i.e. `/nonsense`, this handler will dispatch a `four_oh_four()`. Otherwise, we press forward and try to find the actual SVG file.  If we succeed, we just pass the string back, and if not, we also `four_oh_four()`.

## State Handlers

The remaining handlers are involved with mutating the state.  All of them will pass a request body with an item.  For a new todo, it will be `item=Task`, and for toggling or removing, it will hold the id: `item=e2104f6a-624d-498f-a553-29e559e78d33`.  In either case, we just need to extract the value after the equals sign:

```rust
async fn extract_payload(request: Request) -> String {
    let body = request.into_body();
    let bytes_buf = hyper::body::to_bytes(body).await.unwrap();
    let str_body = String::from_utf8(bytes_buf.to_vec()).unwrap();
    let words: Vec<&str> = str_body.split('=').collect();
    words[1].to_owned()
}
```

The body may come in chunks, so we use `hyper::body::to_bytes()` to produce a single `Bytes` value with everything concatenated.  We can then convert the bytes to a UTF-8 string and split on the `=` to grab the actual payload.  All of our state mutation handlers call this function on the incoming request:

```rust
async fn add_todo_handler(request: Request) -> Response {
    let todos_ctx: Arc<RwLock<Todos>> = Arc::clone(request.extensions().get().unwrap());
    let payload = extract_payload(request).await;
    {
        let mut lock = todos_ctx.write().unwrap();
        (*lock).push(Todo::new(&payload));
    }
    redirect_home().await
}

async fn remove_todo_handler(request: Request) -> Response {
    let todos_ctx: Arc<RwLock<Todos>> = Arc::clone(request.extensions().get().unwrap());
    let payload = extract_payload(request).await;
    {
        let mut lock = todos_ctx.write().unwrap();
        (*lock).remove(Uuid::parse_str(&payload).unwrap());
    }
    redirect_home().await
}

async fn toggle_todo_handler(request: Request) -> Response {
    let todos_ctx: Arc<RwLock<Todos>> = Arc::clone(request.extensions().get().unwrap());
    let payload = extract_payload(request).await;
    {
        let mut lock = todos_ctx.write().unwrap();
        (*lock).toggle(Uuid::parse_str(&payload).unwrap());
    }
    redirect_home().await
}
```

Each handler grabs its own unique reference to the app state, then extracts the payload.  Like we did in `index()`, we open a new scope to interact with the `RwLock` mutex, and in this case, we use `todos_ctx.write()` to request a mutable lock.  This blocks all other tasks until the mutation is complete.  Then, we just `redirect_home()`.  This prompts the client to send a `GET /` request, which leads our to-level handler to call `index()`, which reads the newly-mutated app state to build the page.

Groovy! That's a full-fledged TODO app.

## Serve

There's one missing piece.  We defined our `handle()` function, but we haven't talked about `serve()` beyond the type signature.  This one is pretty beefy:

```rust
async fn serve<C, H, F>(
    addr: std::net::SocketAddr,
    context: Arc<C>,
    handler: H,
) -> hyper::Result<()>
where
    C: 'static + Send + Sync,
    H: 'static + Fn(Request) -> F + Send + Sync,
    F: Future<Output = Response> + Send,
{
    // Create a task local that will store the panic message and backtrace if a panic occurs.
    tokio::task_local! {
        static PANIC_MESSAGE_AND_BACKTRACE: RefCell<Option<(String, Backtrace)>>;
    }
    async fn service<C, H, F>(
        handler: Arc<H>,
        context: Arc<C>,
        mut request: http::Request<hyper::Body>,
    ) -> Result<http::Response<hyper::Body>, Infallible>
    where
        C: Send + Sync + 'static,
        H: Fn(http::Request<hyper::Body>) -> F + Send + Sync + 'static,
        F: Future<Output = http::Response<hyper::Body>> + Send,
    {
        let method = request.method().clone();
        let path = request.uri().path_and_query().unwrap().path().to_owned();
        tracing::info!(path = %path, method = %method, "request");
        request.extensions_mut().insert(context);
        let result = AssertUnwindSafe(handler(request)).catch_unwind().await;
        let start = std::time::SystemTime::now();
        let response = result.unwrap_or_else(|_| {
            let body = PANIC_MESSAGE_AND_BACKTRACE.with(|panic_message_and_backtrace| {
                let panic_message_and_backtrace = panic_message_and_backtrace.borrow();
                let (message, backtrace) = panic_message_and_backtrace.as_ref().unwrap();
                tracing::error!(
                    method = %method,
                    path = %path,
                    backtrace = ?backtrace,
                    "500"
                );
                format!("{}\n{:?}", message, backtrace)
            });
            http::Response::builder()
                .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                .body(hyper::Body::from(body))
                .unwrap()
        });
        tracing::info!(
            "Response generated in {}μs",
            start.elapsed().unwrap_or_default().as_micros()
        );
        Ok(response)
    }
    // Install a panic hook that will record the panic message and backtrace if a panic occurs.
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|panic_info| {
        let value = (panic_info.to_string(), Backtrace::new());
        PANIC_MESSAGE_AND_BACKTRACE.with(|panic_message_and_backtrace| {
            panic_message_and_backtrace.borrow_mut().replace(value);
        })
    }));
    // Wrap the request handler and context with Arc to allow sharing a reference to it with each task.
    let handler = Arc::new(handler);
    let service = hyper::service::make_service_fn(|_| {
        let handler = handler.clone();
        let context = context.clone();
        async move {
            Ok::<_, Infallible>(hyper::service::service_fn(move |request| {
                let handler = handler.clone();
                let context = context.clone();
                PANIC_MESSAGE_AND_BACKTRACE.scope(RefCell::new(None), async move {
                    service(handler, context, request).await
                })
            }))
        }
    });
    let server = hyper::server::Server::try_bind(&addr)?;
    tracing::info!("🚀 serving at {}", addr);
    server.serve(service).await?;
    std::panic::set_hook(hook);
    Ok(())
}
```

I know, I know. There's a lot here.  The meat of this function happens right at the end:

```rust
let server = hyper::server::Server::try_bind(&addr)?;
tracing::info!("🚀 serving at {}", addr);
server.serve(service).await?;
```

We build a `hyper::Server`, bind it to the address we constructed from the `Args` struct, and serve this `service` thing. The `service` is built right above that:

```rust
let handler = Arc::new(handler);
let service = hyper::service::make_service_fn(|_| {
    let handler = handler.clone();
    let context = context.clone();
    async move {
        Ok::<_, Infallible>(hyper::service::service_fn(move |request| {
            let handler = handler.clone();
            let context = context.clone();
            PANIC_MESSAGE_AND_BACKTRACE.scope(RefCell::new(None), async move {
                service(handler, context, request).await
            })
        }))
    }
});
```

We also wrap the handler function in an `Arc`.  Our context is already wrapped, so we clone both to get a local reference within this closure.  This allows the executor to spin up multiple concurrent versions of our handler service that all have access to the same state and logic.

The beefy stuff happens in this `service()` closure above.  This is where we take an incoming request and match it up with our handler and context.  A new instance is executed for each incoming request, and all this ceremony allows this to happen without interfering with other simultaneous requests.

For one, this is where we add our context to the request:

```rust
request.extensions_mut().insert(context);
```

When we call `request.extensions().get()` in our mutating handlers, we're pulling out the context added at this stage.

There's also some logging added.  We trace the request's specifics and start a timer that reports how long the request took.  To see this logging in action, set the environment variable `RUST_LOG=info` when executing the server process.

### Catching Panics

The most exciting part (to me, at least) is the panic handler.  We always want our requests to succeed.  However, there are situations where we may encounter a `panic`.  This will cause the entire Rust program to crash and print out a stack trace in normal usage.  However, this is a web service.  A panic situation in one request handling process shouldn't prevent other requests from executing.  We don't want the whole server to crash; we still want to handle these situations gracefully.  We can intercept the normal panic behavior and instead simply produce a different response containing the details.

At the top, we create a task-local storage location:

```rust
tokio::task_local! {
    static PANIC_MESSAGE_AND_BACKTRACE: RefCell<Option<(String, Backtrace)>>;
}
```

This is local to just the currently executing Tokio task, not the whole program.  Then, we replace the default panic logic with our own:

```rust
let hook = std::panic::take_hook();
std::panic::set_hook(Box::new(|panic_info| {
    let value = (panic_info.to_string(), Backtrace::new());
    PANIC_MESSAGE_AND_BACKTRACE.with(|panic_message_and_backtrace| {
        panic_message_and_backtrace.borrow_mut().replace(value);
    })
}));

// ...

std::panic::set_hook(hook);
```

First, we grab the existing hook and store it to the `hook` variable.  Then, we overwrite our own.  At the end of the function, we make sure to reset the global panic hook back to what it was.  If the task panics inside this function - for example, one of our `unwrap()` statements encounter a non-success, we'll store the panic message and backtrace to this task-local location.  However, we will _not_ abort the process.

Up above, in the `service` location, we can catch this happening:

```rust
let result = AssertUnwindSafe(handler(request)).catch_unwind().await;
```

We attempt to build the response, but this result will not be a success if anything happens.  If it was a success, great, we pass it back up.  However, if we find an error value here, we can dispatch different logic:

```rust
let response = result.unwrap_or_else(|_| {
    let body = PANIC_MESSAGE_AND_BACKTRACE.with(|panic_message_and_backtrace| {
        let panic_message_and_backtrace = panic_message_and_backtrace.borrow();
        let (message, backtrace) = panic_message_and_backtrace.as_ref().unwrap();
        tracing::error!(
            method = %method,
            path = %path,
            backtrace = ?backtrace,
            "500"
        );
        format!("{}\n{:?}", message, backtrace)
    });
    http::Response::builder()
        .status(http::StatusCode::INTERNAL_SERVER_ERROR)
        .body(hyper::Body::from(body))
        .unwrap()
});
```

For most requests, that `result.unwrap()` went fine, and we just stored our response to `Response`.  However, if it was an error, we can read the result of the panic from this task-local area.  We emit an error trace on the server-side and then build a _new_ response with status code `INTERNAL_SERVER_EROR`.  This response includes the full backtrace as the body.  This means our server can keep handling other requests without interruption, but the specific client that caused the panic gets a complete log of the problem, and our server has logged the backtrace as well.  We can diagnose the issue without losing uptime for any other client.

Now, no matter what happened while processing the request, we've stored a valid `hyper::Response` to this response value, and we can pass that back to the caller even if something catastrophic occurs.  We can safely use `Ok::<_, Infallible>`, signifying that there is no possible way for control to fail to hit this point.  Our server will _always_ generate a response and continue, even if something terrible happens.  Good stuff.

## Tests

Finally, we want to ensure we can automate tests. I'll just demonstrate a test of our 404 handler, which includes all the pieces needed to build a robust test suite:

```rust
#[cfg(test)]
mod test {
    use super::*;
    use flate2::write::ZlibDecoder;
    use pretty_assertions::assert_eq;
    use select::{document::Document, predicate::Name};

    #[tokio::test]
    async fn test_four_oh_four() {
        let mut request = hyper::Request::builder()
            .method(http::Method::GET)
            .uri("/nonsense")
            .body(hyper::Body::empty())
            .unwrap();
        let context = Arc::new(RwLock::new(Todos::new()));
        request.extensions_mut().insert(Arc::clone(&context));
        let response = handle(request).await;

        assert_eq!(response.status(), http::status::StatusCode::NOT_FOUND);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let mut decoder = ZlibDecoder::new(Vec::new());
        decoder.write_all(&body).unwrap();
        let uncompressed = decoder.finish().unwrap();
        let result = String::from_utf8(uncompressed).unwrap();

        let document = Document::from(result.as_str());
        let message = document.find(Name("h1")).next().unwrap().text();
        assert_eq!(message, "NOT FOUND!".to_owned());
    }
}
```

Tokio provides a `#[tokio::test]` macro for building async tests.  We can use `hyper::Request` to construct a request and build our context the same way we did in the server.  Because our handler is just a function from a `Request` to a `Response`, we can test it very simply: `let response = handle(request).await;`.

We first assert the status code matches what we expect, then we have to decode the body.  We use the `ZlibDecoder` to read the response body and decompress it back to a string.

Once we have our string response, we can use the [`select.rs`](https://github.com/utkarshkukreti/select.rs) library to ensure the structure matches our intent.  In this case, we are asserting we've received an `h1` element with a text body matching the string `NOT FOUND!`.

## Fin

This implementation is an improvement over the previous in several fundamental ways.  The `async/.await` syntax allows us to write code closely matching our intent without getting bogged down with `Box`es and `Future`s.  We avoid polluting the global scope and use the `Request` itself to handle concurrent access to the app state and even catastrophic request handling errors gracefully without affecting other clients.  Our handler is straightforward to test.  This application provides a solid, performant base for building more complex applications and keeps the dependencies and therefore compile times and bundle sizes to a minimum.

While there are many different frameworks for building web applications in Rust, it's worth asking yourself whether you actually need that level of abstraction.  For many web service needs, putting the building blocks together yourself isn't much more complicated, and you retain control over how you build your application.  If we want to deconstruct the request URI, we can do that already.  If we return JSON, we just need to create a struct that implements `serde::Serialize`.

The conclusion here is the same as before: when the end goal is simple enough, why not use simple tools?

_Cover photo by [Danny Howe](https://unsplash.com/@dannyhowe?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText) on [Unsplash](https://unsplash.com/s/photos/festival?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText)_
  