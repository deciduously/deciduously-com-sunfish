---
title: I Scrapped My Stencil Project And Wrote A Static Site Instead
description: Despite everything, I wrote another DIY static site in Rust.
cover_image: crab_medium.jpg
tags: hooray, works
published: true
edited: 2020-02-01T12:05:00.000Z
---

TODO MAKE A TEMPLATE APP ON GITHUB

## Simplicity wins

In a previous post, I dramatically announced that I had *figured out my tool* for frontend:

POST HERE

I then promptly hopped ship and built a second static site generator in Rust from scratch, just like I said the whole problem was the first time.  Oops.

Now, in my defense, I still stand by the previous post.  That was an ankle-deep survey of the landscape, and things were rosy.  It's not accurate to say that I hopped ship immediately - I did get, like, ankle deep.  I had a 3k line codebase on my hands before hopping ship.

But, like, I had a 3k line codebase on my hands.

So, my Stencil conclusion is that it's still by a lot mny favorite tool for that kind of thing.  However, it is imperative that you fit the tool to the job.  This job just ain't it.

Also, `async` stabilized:

```rust
pub async fn router(req: Request<Body>) -> Result<Response<Body>, std::convert::Infallible> {
    let (method, path) = (req.method(), req.uri().path());
    info!("{} {}", method, path);
    match (method, path) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => index().await,
        (&Method::GET, "/cv") => cv().await,
        (&Method::GET, "/main.css") => {
            string_handler(include_str!("assets/main.css"), "text/css", None).await
        }
        (&Method::GET, "/manifest.json") => {
            string_handler(include_str!("assets/manifest.json"), "text/json", None).await
        }
        (&Method::GET, "/robots.txt") => {
            string_handler(include_str!("assets/robots.txt"), "text", None).await
        }
        (&Method::GET, path_str) => image(path_str).await,
        _ => {
            warn!("{}: 404!", path);
            four_oh_four().await
        }
    }
}
```

I mean, come on.  Look at it.  Before the drop, you're working with these crazy types, that even required external crates to work at all:

```rust
fn router(req: Request<Body>, _client: &Client<HttpConnector>) -> Box<Future<Item = Response<Body>, Error = Box<dyn std::error::Error + Send + Sync>> + Send> {
    match (req.method(), req.uri().path()) {

        (&Method::GET, "/") | (&Method::GET, "/index.html") => index(),
        (&Method::GET, "/static/todo.css") => stylesheet(),
        (&Method::GET, path_str) => image(path_str),
        (&Method::POST, "/done") => toggle_todo_handler(req),
        (&Method::POST, "/not-done") => toggle_todo_handler(req),
        (&Method::POST, "/delete") => remove_todo_handler(req),
        (&Method::POST, "/") => add_todo_handler(req),
        _ => four_oh_four(),
    }
}
```

No more `futures` - just use the standard language feature `await`.  This kinda changes everything for me about writing this sort of code in Rust, even thous it's really just some super convenient syntax sugar.

As I got deeper into my over-engineered Stencil mess and looked at what sorts of stuff was being shipped to my browser and run just to render the very simple markup and style I needed, Rust just kept looking sweeter and sweeter.

// XKCD gluing together stuff

This gives me the developer experience I wanted and nearly got with Stencil while also delivering a very simple, reliable, and fast set o' bits down the wire.

Who knew?

### Askama > Components

Askama is the secret sauce, here.  For a static site, most of what I want components for is dicing up markup.  I also liked the ability to use TypeScript to help make the structure of data flow between them well defined and rigid, to avoid runtime issues.

### Deployment

Everything except images are compiled into a native binary.  Run `cargo build --release` using whatever target you need.  Run the resulting executable with the desired options!

I THINK IT WILL NEED IMAGE ASSETS

### jq

### Tailwind

### Config

Toml scrape + Structopt  to same struct

## Conclusion

It's just a lot more 'me' now.
