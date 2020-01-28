---
title: I Scrapped My Stencil Project And Wrote A Static Site In Rust Instead And I'm Not Even Sorry
description: Despite everything, I wrote another DIY static site in Rust.
cover_image: crab_medium.jpg
tags: hooray
published: false
date: 2020-01-31T11:00:00.000Z
---

## Simplicity wins

// XKCD gluing together stuff

This gives me the developer experience I wanted and nearly got with Stencil while also delivering a very simple, reliable, and fast set o' bits down the wire.

Who knew?

### Askama > Components

Askama is the secret sauce, here.  For a static site, most of what I want components for is dicing up markup.  I also liked the ability to use TypeScript to help make the structure of data flow between them well defined and rigid, to avoid runtime issues.

### async

This isn't a game changer, but it is nice:

```rust
// TODO use your eventual router() instead
pub async fn index(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let markup = CvTemplate::default();
    Ok(Response::new(Body::from(markup.render().unwrap())))
}

#[tokio::main]
async fn main() {
    init_logging(2); // For now just INFO
    let addr = format!("{}:{}", OPT.address, OPT.port).parse().expect("Should parse net::SocketAddr");
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(index)) });

    let server = Server::bind(&addr).serve(make_svc);

    info!("Serving deciduously-com on {}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
```

**Oooh, aah**.  Compare to my version from before the new stuff:

```rust
fn router(req: Request<Body>, _client: &Client<HttpConnector>) -> Box<Future<Item = Response<Body>, Error = Box<dyn std::error::Error + Send + Sync>> + Send> {
    // pattern match for both the method and the path of the request
    match (req.method(), req.uri().path()) {
        // GET handlers
        // Index page handler
        (&Method::GET, "/") | (&Method::GET, "/index.html") => index(),
        // Style handler
        (&Method::GET, "/static/todo.css") => stylesheet(),
        // Image handler
        (&Method::GET, path_str) => image(path_str),
        // POST handlers
        (&Method::POST, "/done") => toggle_todo_handler(req),
        (&Method::POST, "/not-done") => toggle_todo_handler(req),
        (&Method::POST, "/delete") => remove_todo_handler(req),
        (&Method::POST, "/") => add_todo_handler(req),
        // Anything else handler
        _ => four_oh_four(),
    }
}

fn main() {
    pretty_env_logger::init();

    // .parse() parses to a std::net::SocketAddr
    let addr = "127.0.0.1:3000".parse().unwrap();

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
}
```

So much cleaner now!  I mean, look at those types.  We don't even need to bring in a separate crate, I'm finally using the standard library!  It doesn't seem like a huge deal, but I think `async` makes tools like `hyper` much more approachable for beginners.

Stabilized in LOOK IT UP on WHAT DATE

### Deployment

Everything except images are compiled into a native binary.  Run `cargo build --release` using whatever target you need.  Run the resulting executable with the desired options!

I THINK IT WILL NEED IMAGE ASSETS

### jq

### Tailwind

### Docker

I deployed on the DigitalOcean [One-Click Docker](https://marketplace.digitalocean.com/apps/docker) app.  The cheapest tier is $5/month.  My docker image is only 6.82 megabytes!  Updating this thing is super light on bandwidth.  Here's the dockerfile, doing a mutli-stage thing to build a statically-linked application locally then only ship the executable:

```dockerfile
FROM rust:1.40.0 AS builder
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl

# Create a dummy project and build the app's dependencies.
# If the Cargo.toml or Cargo.lock files have not changed,
# we can use the docker build cache and skip these (typically slow) steps.
RUN USER=root cargo new deciduously-com
WORKDIR /usr/src/deciduously-com
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Copy the source and build the application.
COPY src ./src
COPY templates ./templates
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Copy the statically-linked binary into a scratch container.
FROM scratch
COPY --from=builder /usr/local/cargo/bin/deciduously-com .
COPY --from=builder /usr/src/deciduously-com/templates .
USER 1000
CMD ["./deciduously-com -p 80 -a 0.0.0.0"]
```

Many thanks to [this blog post](https://alexbrand.dev/post/how-to-package-rust-applications-into-minimal-docker-containers/) by [@alexbrand](https://twitter.com/alexbrand).

## Conclusion

It's just a lot more 'me' now.
