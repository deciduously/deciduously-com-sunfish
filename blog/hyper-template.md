---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--WeqCPPqP--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://dev-to-uploads.s3.amazonaws.com/i/sx49v0fgpkipgyfd3n53.jpg
edited: 2020-02-20T12:00:00.000Z
title: Hyper Webapp Template
published: true
description: A Hyper template including Askama, TailwindCSS, and Docker.
tags: beginners, rust, webdev, showdev
---
Like many of us, I'm quite lazy.  When making a wep application, lots of the core functionality will be the same from codebase to codebase.  You need to respond to HTTP requests, generate and return HTML bodies, serve static assets, handle unknown routes.  There's no pressing need to reinvent all of this from the ground up just to render a new webpage.

This is why we have frameworks, we don't like doing this stuff over and over again.

However, also like many of us, I'm quite particular.  This [XKCD](https://xkcd.com/1988/) comes to mind a lot:

![xkcd](https://explainxkcd.com/wiki/images/5/53/containers.png)

Most CLI scaffolding tools  give me this feeling.  I get that all the extra boilerplate is actually stuff I want, but I don't know what it all *is*.

So, I made my own.

If you're like me, you won't use this or any other template.  However, if you ARE me, you will, because you built it!  Otherwise, it may be helpful for doing your own.

Here's the [GitHub repo](https://github.com/deciduously/hyper-template).  You can click the handy "Use this template" button and get going.

Here's the highlights:

* [Hyper](https://hyper.rs/) - No framework, just a small and fast HTTP server library.
* [Askama](https://github.com/djc/askama) - Typesafe, compiled templates.
* [TailwindCSS](https://tailwindcss.com/) - Granular styling for people who don't know how to use actual CSS.
* [Docker](https://www.docker.com/) - Simple, quick deployment.
* [Github Action](https://github.com/features/actions) - Get a fancy green check mark next to your commits.

Let's take a quick tour.


```txt
$ tree
.
├── Cargo.lock                    # Rust package lockfile
├── Cargo.toml                    # Rust package metadata
├── Dockerfile                    # Docker container build instructions
├── LICENSE                       # I use the BSD-3-Clause License
├── README.md                     # Markdown readme file
├── package.json                  # NPM package metadata
├── pnpm-lock.yaml                # NPM package lockfile
├── postcss.config.js             # CSS processing configuration
├── src
│   ├── assets
│   │   ├── config.toml           # Set runtime options (port, address)
│   │   ├── images
│   │   │   └── favicon.ico       # Any static images can live here
│   │   ├── main.css              # Postcss-compiled styelsheet - don't edit this one
│   │   ├── manifest.json         # WebExtension API metadata file
│   │   └── robots.txt            # Robots exclusion protocol
│   ├── config.rs                 # Read config.toml/CLI options, init logging
│   ├── css
│   │   └── app.css               # App-wide stylesheet - DO edit this one
│   ├── handlers.rs               # Rust functions from Requests to Responses
│   ├── main.rs                   # Entry point
│   ├── router.rs                 # Select proper handler from request URI
│   ├── templates.rs              # Type information for templates
│   └── types.rs                  # Blank - use how you like!
├── stylelintrc.json              # CSS style lint options
└── templates
    ├── 404.html                  # Not Found stub
    ├── index.html                # Main page template
    └── skel.html                 # App-wide skeleton markup

5 directories, 24 files
```

One of the oddities (and cool things) is that the `assets/` directory actually lives inside `src/`.  This is because all of these text file assets are included right in the binary as static strings via the `include_str!()` macro.  When you deploy, none of this extra stuff is present.  The deployment directory will look like this, if Docker is not used:

```txt
$ tree
.
├── LICENSE                       # I use the BSD-3-Clause License
├── README.md                     # Markdown readme file
├── images
│    └── favicon.ico              # Favicon
└── hyper-template                # Executable
```

Just run the thing!

I'll briefly unpack a few of these files.  Let's look at `main.rs` first:

```rust
#[tokio::main]
async fn main() {
    init_logging(2); // set INFO level
    let addr = format!("{}:{}", OPT.address, OPT.port)
        .parse()
        .expect("Should parse net::SocketAddr");
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(router)) });

    let server = Server::bind(&addr).serve(make_svc);

    info!("Serving {} on {}", env!("CARGO_PKG_NAME"), addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
```

The only part of this file you might touch is in the `make_service_fn` call.  This stub assumes your handlers cannot fail and uses [`std::convert::Infallible`](https://doc.rust-lang.org/std/convert/enum.Infallible.html).  This means that any errors that do pop up in this call (so, your router and handlers) will need to be handled right there, with `unwrap()` or `expect()`.  You can get yourself a little more flexibility by simply swapping in [`anyhow::Error`](https://github.com/dtolnay/anyhow)!  That way, all those `unwrap()`s can turn into `?`s.  This is what I've done personally when using this template, but I decided not to make that choice for you - that felt like an overreach in a minimal template.

Also, notably, Rust has `async/await` now!  This is very cool syntax for some features (Futures) that have already existed, making the whole thing much much more accessible.  No more crazy 120-char types!  For a primer, [start here](https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html).

You won't really need to touch this.  It just sets up the asynchoronous runtime and converts your actual application to a state machine that can use it.  In this case, our actual application is the function `router()`.  That looks like this:

```rust
pub async fn router(req: Request<Body>) -> HandlerResult {
    let (method, path) = (req.method(), req.uri().path());
    info!("{} {}", method, path);
    match (method, path) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => index().await,
        (&Method::GET, "/main.css") => {
            string_handler(include_str!("assets/main.css"), "text/css", None).await
        }
        (&Method::GET, "/manifest.json") => {
            string_handler(include_str!("assets/manifest.json"), "text/json", None).await
        }

        (&Method::GET, "/robots.txt") => {
            string_handler(include_str!("assets/robots.txt"), "text", None).await
        }
        (&Method::GET, path_str) => {
            // Otherwise...
            // is it an image?
            if let Some(ext) = path_str.split('.').nth(1) {
                match ext {
                    "ico" | "svg" => image(path).await,
                    _ => four_oh_four().await,
                }
            } else {
                four_oh_four().await
            }
        }
        _ => {
            warn!("{}: 404!", path);
            four_oh_four().await
        }
    }
}
```

All of the handlers eventually pass through to this function:

```rust
/// Top-level handler that DEFLATE compresses and responds with from a &[u8] body
/// If None passed to status, 200 OK will be returned
pub async fn bytes_handler(
    body: &[u8],
    content_type: &str,
    status: Option<StatusCode>,
) -> HandlerResult {
    // Compress
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(body).unwrap();
    let compressed = e.finish().unwrap();
    // Return response
    Ok(Response::builder()
        .status(status.unwrap_or_default())
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CONTENT_ENCODING, "deflate")
        .body(Body::from(compressed))
        .unwrap())
}
```

It takes your response body as a [byte slice](https://doc.rust-lang.org/book/ch04-03-slices.html) and compresses it before returning it, adding the proper headers.  Lots of resources are going to be HTML, but we're always using this anyway:

```rust
pub async fn string_handler(
    body: &str,
    content_type: &str,
    status: Option<StatusCode>,
) -> HandlerResult {
    bytes_handler(body.as_bytes(), content_type, status).await
}

pub async fn html_str_handler(body: &str) -> HandlerResult {
    string_handler(body, "text/html", None).await
}
```

These templates all get a specific struct:

```rust
use askama::Template;

#[derive(Default, Template)]
#[template(path = "skel.html")]
pub struct SkelTemplate {}

#[derive(Default, Template)]
#[template(path = "404.html")]
pub struct FourOhFourTemplate {}

#[derive(Default, Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}
```

They're blank for now, there's no data flowing through.  If you wanted to pass a string into the index, it might look like this:

```rust
#[derive(Default, Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub name: &'a str,
}
```

Now you need to instantiate the struct with properly typed data, and you can use `name` inside your template file.

This template comes pre-hooked up with Tailwind - here's `app.css`:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

Again, it's your app, not mine.  When you're ready to style, just start right below these directives - or directly in your templates!  The provided NPM scripts will compile all your CSS into `src/assets/main.css` before compiling the Rust binary, so it too can be included as a static string.

That's pretty much it!  This app is *extremely barebones*, just how I like my templates.  I just successfully used this template to spin up a more complicated application, with a database and some scraping logic, and starting from here instead of from scratch saved me a few hours at the beginning.  YMMV.

Stay tuned for two less-minimal variants of this - one for building a static blog, and one with a database and ORM hooked up!

There is definitely room for improvement, here.  The code could be refactored (middleware, maybe?), there needs to be tests, etc.  I'll get there eventually, but also, I'll take a PR :)

*Photo by Shiro hatori on Unsplash*
