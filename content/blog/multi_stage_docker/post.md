---
title: Use Multi-Stage Docker Builds For Statically-Linked Rust Binaries
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--jd9si7y1--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://dev-to-uploads.s3.amazonaws.com/i/c8jy6kr09r0zhjmlj3pr.jpg
tags:
  - beginners
  - rust
  - docker
  - todayilearned
date: 2020-02-05T12:00:00.000Z
---

I'm making a [static website](https://en.wikipedia.org/wiki/Static_web_page) in [Rust](https://www.rust-lang.org/). Last time I did this, I used [Docker](https://www.docker.com/) to automate the deployment. I was frustrated at how much bandwidth I was using shuffling around these massive build images, but the convenience was too hard to pass up and I wasn't rebuilding the image often, so just left it.

With this new method, my final production Docker image for the whole application is 6.85MB. _I can live with that_.

I'm using [Askama](https://github.com/djc/askama) for templating, which actually compiles your typechecked templates into your binary. The image assets I have are all [SVG](https://www.w3.org/Graphics/SVG/), which is really [XML](https://en.wikipedia.org/wiki/XML), so I can use [`include_str!()`](https://doc.rust-lang.org/std/macro.include_str.html) for those along with things like `manifest.json` and `robots.txt` and all CSS, which includes their entire file contents directly in my compiled binary as a `&'static str`. As a result, I don't really _need_ a full Rust build environment or even any asset files present to run the compiled output.

This time around, I did my homework and found [this blog post](https://alexbrand.dev/post/how-to-package-rust-applications-into-minimal-docker-containers/) by [@alexbrand](https://twitter.com/alexbrand), which demonstrates this technique. Instead of just bundling up with all the build dependencies in place, you can use a [multi-stage build](https://docs.docker.com/develop/develop-images/multistage-build/) to generate the compiled output first and then copy it into a minimal container for distribution. Here's my adaptation for this project:

```dockerfile
# Build Stage
FROM rust:1.40.0 AS builder
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new deciduously-com
WORKDIR /usr/src/deciduously-com
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
COPY templates ./templates
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Bundle Stage
FROM scratch
COPY --from=builder /usr/local/cargo/bin/deciduously-com .
USER 1000
CMD ["./deciduously-com", "-a", "0.0.0.0", "-p", "8080"]
```

That's it! The top section labelled `builder` uses the `rust:1.40.0` base image, which has everything needed to build my binary with rust. It targets `x86_64-unknown-linux-musl`. The [musl](https://www.musl-libc.org/) library is an alternative `libc` designed for [static linking](https://en.wikipedia.org/wiki/Static_library) as opposed to [dynamic](https://en.wikipedia.org/wiki/Dynamic_linker). Rust has top-notch support for this (apparently). This means the resulting binary is entirely self-contained - it has no environment requirements at all.

The second section, which defines the actual distribution, just starts from [`scratch`](https://hub.docker.com/_/scratch/), not even [`alpine`](https://www.alpinelinux.org/) or whatever other minimal Docker base image I'd otherwise use. You can use `COPY --from=builder` to reference the previous Docker stage. This docker image has _nothing at all_ in it. This means my image really just contains my binary, no Linux userland to be found! All with one invocation of `docker build`.

The middle part, with `cargo new`, makes a dummy application leveraging the docker cache for dependencies. This means that while you're developing, subsequent runs of `docker build` won't need to rebuild every dependency in your Rust application every time, it will only rebuild what's changed just like building locally. Marvelous!

I'm deploying on the DigitalOcean [One-Click Docker](https://marketplace.digitalocean.com/apps/docker) app, which is an Ubuntu LTS image with docker pre-installed and some [UFW](https://en.wikipedia.org/wiki/Uncomplicated_Firewall) settings preset. This was my whole deploy process:

```
$ docker build -t deciduously-com .
$ docker tag SOMETAG83979287 deciduously0/deciduously-com:latest
$ docker push deciduously0/deciduously-com:latest
$ ssh root@SOME.IP.ADDR
root@SOME.IP.ADDR# docker pull deciduously0/deciduously-com:latest
root@SOME.IP.ADDR# docker run -dit -p 80:8080 deciduously0/deciduously-com:latest
root@SOME.IP.ADDR# exit
$
```

The remote server pulls down my whopping 6.85MB image and spins it up. I was immediately able to connect. This minuscule image just sips at disk space, memory, and CPU, so I'm going to be able to stretch my $5/month lowest-possible-tier DigitalOcean droplet as far as it can possibly go. The flashbacks I'm having from trying to do something similar with Clojure are terrifying...

Add in some scripts so you don't have to remember those commands, and my whole build and deploy process is distilled to a few keystrokes.

Why would I use anything else?

For those keeping score, yes, I've already scrapped [Stencil](https://dev.to/deciduously/stencil-i-think-i-found-my-frontend-home-46bf) in favor of [Askama](https://github.com/djc/askama)/[Hyper](https://hyper.rs/). Within a day I had re-implemented all previous work in about a half of the code and a small fraction of the bundle size. Yes, there's a bigger post (and GitHub template) about it brewing, and no, I'm not even sorry. [KISS](https://en.wikipedia.org/wiki/KISS_principle) and all...

_Photo by Richard Sagredo on Unsplash_
