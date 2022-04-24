---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--H5b_3Qbt--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://dev-to-uploads.s3.amazonaws.com/i/vz50vjb1ufs2snvbb0pd.jpg
date: 2020-02-07T12:00:00.000Z
title: Automatically Generate Rust Modules With Cargo Build Scripts
description: Learn how to use build.rs to autogenerate Rust code.
tags:
  - beginners
  - rust
  - devjournal
  - todayilearned
---

I just learned how to use [Cargo build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html). They're pretty cool.

## The Context

If you don't care about the context, here's the [build script part](#the-build-script-part).

I'm rebuilding my personal website from scratch and plan to re-host my DEV blog posts there. I've selected the [`askama`](https://github.com/djc/askama) library to generate HTML for my webpages. This tool is kind of like [Jinja](https://jinja.palletsprojects.com/en/2.11.x/) (or [`tera`](https://github.com/Keats/tera) in the Rust world) but with one significant difference - it typechecks your templates, and actually compiles them directly in to your application's exectuable.

As an example, here's my toplevel `skel.html` template:

```html
<!DOCTYPE html>
<html dir="ltr" lang="en">

<head>
  <meta charset="utf-8" />
  <title>{% block title %}{% endblock %} - deciduously.com</title>
  <meta name="Description" content="Ben Lovy's personal website" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0, minimum-scale=1.0, maximum-scale=5.0" />
  <link rel="icon" type="image/x-icon" href="/favicon.svg" />
  <link rel="stylesheet" href="/main.css" />
  <link rel="manifest" href="/manifest.json" />
</head>

<body>
  <header>
    <nav>
      {% for link in links %}
      <a class="{% if link.target == "/" %}font-extrabold text-lg{% else %}italic{% endif %} px-10"
        href={{ link.target }}>{{ link.name }}</a>
      {% endfor %}
    </nav>
  </header>
  <main>
    {% block content %}{% endblock %}
  </main>
  <footer class="text-xs italic">
    © 2020 Ben Lovy - <a href="https://github.com/deciduously/deciduously-com" target="_blank"
      rel="noreferrer">source</a>
  </footer>
</body>

</html>
```

You can create subpages using `extends`, and then add your own content to fill in the `block`s defined in the base:

```html
{% extends "skel.html" %} {% block title %}404{% endblock %} {% block content %}
<h1>NOT FOUND!</h1>
{% endblock %}
```

On the Rust side, to render this markup you create a struct and pass it the file directly in a tag:

```rust
#[derive(Template)]
#[template(path = "skel.html")]
pub struct SkelTemplate {
    links: &'static [Hyperlink],
}

impl Default for SkelTemplate {
    fn default() -> Self {
        Self { links: &NAV }
    }
}
```

When the template says `{% for link in links %}` it means specifically what Rust has stored in this struct field. To finally pull out your rendered markup, you instantiate the struct and call `render()`, which `askama` auto-generates for us:

```rust
pub async fn four_oh_four() -> HandlerResult {
    let template = FourOhFourTemplate::default();
    let html = template.render().expect("Should render markup");
    string_handler(&html, "text/html", Some(StatusCode::NOT_FOUND)).await
}
```

If you needed to inject any data, you'd have to store it in the struct and define a constructor (or other method) to add the data. It works just like you expect any other Rust to work. All the data flowing into this template is defined in this struct, and verified by the compiler well before it ever hits your markup.

This is great for all the reasons Rust typechecking is usually great. It's also highly performant, because your template gets slurped up right into your binary and precompiled - no file IO happens at runtime, and all templating operations like loops and conditions are already turned into actual Rust loops and conditions by the time they're called. It's _totally sweet_.

The magic happens in the tag:

```rust
#[derive(Template)]
#[template(path = "skel.html")]
```

This is a [procedural macro](https://blog.rust-lang.org/2018/12/21/Procedural-Macros-in-Rust-2018.html). When your code is compiled, these expand before anything else happens. In this case, it parses your template and inserts the resulting Rust code in your module before compilation begins as an `impl MyTemplate {}` block that includes a `render(&self)` method you can then call. It's during this macro expansion stage, not compilation, that your actual template files like `skel.html` are opened up from the filesystem - it assumes they're all in `<crate root>/templates` - and after that your code does not read those files again.

## The Problem

I want to author my posts in Markdown, not HTML. This means I'm going to need to transform my Markdown to HTML before serving them up. Okay, that's fine - I'm driving a programming language. This is a three-line problem with [`pulldown-cmark`](https://github.com/raphlinus/pulldown-cmark):

```rust
let parser = pulldown_cmark::Parser::new("# THE BEST HEADING");
let mut html = String::new();
html::push_html(&mut html, parser);
println!("{}", html); // <h1>THE BEST HEADING</h1>
```

This generated markup, though, also needs to inherit the `skel.html` boilerplate to make it look like it's part of the same website. Easy enough, I just need to make a new template for each file.

Scaling up juuust slightly, if this is my markdown:

```md
---
title: COOL POST
---

# THE BEST HEADING

But _nothing_ compared to this intro!
```

This is my markup:

```html
{% extends "skel.html" %} {% block title %}COOL POST{% endblock %} {% block
content %}
<h1>THE BEST HEADING</h1>
<p>But <em>nothing</em> compared to this intro!</p>
{% endblock %}
```

That's a string manipulation problem - again, we're driving a programming language, so I'm okay with that:

```rust
fn write_template(title: &str, html: &str, file: &mut std::fs::File) -> Result<(), std::io::Error> {
    writeln!(file, "{{% extends \"skel.html\" %}}")?;
    writeln!(file, "{{% block title %}}{}{{% endblock %}}", title)?;
    writeln!(file, "{{% block content %}}{}{{% endblock %}}", html)?;
    Ok(())
}
```

You might have already guessed the snag, here. To get these Askama templates out of our Markdown and write them to disk, we need to execute some code. However, all of our template macros have _already_ expanded by the time we have the chance to run this process.

In order for this to work, we need to somehow auto-generate these template files and corresponding structs _before_ the macro expansion phase - which, as we've gone over, happens before anything else. Ruh-roh.

## The Fix

When I first tackled this problem, I...well, I didn't tackle it at all. I instead created a separate built-in CLI command for my executable to handle this, so I had a `publish` mode and a `serve` mode. You needed to invoke `publish` before building your production binary. It worked, but I hated it.

Another option would be to ditch Askama and just use the aforementioned [`tera`](https://github.com/Keats/tera) instead, which does do its work at runtime. It's quick and easy and gets the job done more than adequately, and you probably should just do that. You lose out on the typechecking and the self-contained binary, though. I'm also stubborn.

Luckily, there's build scripts!

### The Build Script Part

A [build.rs](https://doc.rust-lang.org/cargo/reference/build-scripts.html) file can be placed in the root of your crate, outside of `src`. It's not a part of your crate. If present, `cargo` will compile and run it _before_ getting to your crate.

The example given in the documentation link is for FFI:

```rust
// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/hello.c");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("src/hello.c")
        .compile("hello");
}
```

This script checks to see if `hello.c` has changed, and will rebuild it if necessary before compiling your crate.

One annoying thing is that you communicate with `cargo` from within the script by writing to `stdout`: `println!("cargo:rerun-if-changed=src/hello.c");`. This path does not recurse through directories, so if you want to watch for changes for, say, every template in `templates/`, you're gonna need to write a separate line to `stdout` for each file therein.

Being a regular old Rust program, that's not really an issue - we can read the directory and generate a `println!()` statement for each line found:

```rust
#[derive(Debug, Default)]
pub struct Blog {
    pub posts: Vec<BlogPost>,
}

impl Blog {
    fn new() -> Self {
        let mut ret = Blog::default();
        // scrape posts
        let paths = std::fs::read_dir("blog").expect("Should locate blog directory");
        for path in paths {
            let path = path.expect("Could not open blog post").path();
            let post = BlogPost::new(ret.total(), path);
            ret.posts.push(post);
        }
        ret
    }
    fn total(&self) -> usize {
        self.posts.len()
    }
}

fn main() {
    let blog = Blog::new();
    println!("cargo:rerun-if-changed=blog");
    for p in &blog.posts {
        println!("cargo:rerun-if-changed=blog/{}.md", p.url_name);
    }
}
```

That'll do. So, if we can use Rust, we can use `std::fs::File` and `writeln!()` like we did for generating Askama templates above. Why not write some Rust instead:

```rust
fn write_link_info_type(file: &mut std::fs::File) -> Result<(), std::io::Error> {
    writeln!(file, "#[derive(Debug, Clone, Copy)]")?;
    writeln!(file, "pub struct LinkInfo {{")?;
    writeln!(file, "    pub id: usize,")?;
    writeln!(file, "    pub url_name: &'static str,")?;
    writeln!(file, "    pub title: &'static str,")?;
    writeln!(file, "}}\n")?;
    Ok(())
}

fn generate_module() -> Result<(), std::io::Error> {
    let mut module = std::fs::File::create(&format!("src/{}.rs", "blog"))?;
    write_link_info_type(&mut module)?;
    Ok(())
}

fn main() {
    if let Err(e) = generate_module() {
        eprintln!("Error: {}", e);
    }
}
```

This build script will plop a file in your crate at `src/blog.rs` that looks like this:

```rust
#[derive(Debug, Clone, Copy)]
pub struct LinkInfo {
    pub id: usize,
    pub url_name: &'static str,
    pub title: &'static str,
}
```

That looks like runnable Rust! All you need to do is ensure you add it to `main.rs` or `lib.rs`:

```rust
mod blog;
```

Boom, brand new module. It gets better, though. Not only can you use the Rust standard library, you can actually use anything `cargo` can find. You can add dependencies to `Cargo.toml` for the build phase specifically:

```toml
[build-dependencies]
pest = "2.1"
pest_derive = "2.1"

[build-dependencies.pulldown-cmark]
default-features = false
version = "0.6"
```

Anything defined here are NOT available to your crate, only to `build.rs`. If you want to use something in both, you need to add it to both sections of this file. The only thing you can't use here is your crate specifically, because it by definition has not yet been built. Beyond that you're good to go.

I decided I wanted a little more fine-grained control over the Markdown-header-to-Rust-handler-and-template pipeline, so I used [`pest`](https://pest.rs) to throw together my own blog post parser to crawl through the header:

```pest
header = { header_guard ~ attribute{3,6} ~ header_guard }
    header_guard = _{ "-"{3} ~ NEWLINE }
    attribute = { key ~ ": " ~ value ~ NEWLINE }
        key = { (ASCII_ALPHANUMERIC | "_")+ }
        value = { (ASCII_ALPHANUMERIC | PUNCTUATION | " " | ":" | "/" | "+")* }

body = { ANY* }

draft = { SOI ~ header ~ body? ~ EOI }
```

This means that _right in the build script_ I can parse and generate a structure for my blog posts:

```rust
// Compiles drafts to templates and generates struct
#[derive(Parser)]
#[grammar = "draft.pest"]
struct Draft;

#[derive(Debug, Default, Clone)]
pub struct BlogPost {
    pub cover_image: Option<String>,
    pub description: Option<String>,
    pub edited: Option<String>, // only if published
    pub id: usize,
    pub published: bool,
    pub markdown: String,
    pub url_name: String,
    pub title: String,
}
```

I can use the Pest parser to work with the markdown files right here:

```rust
impl BlogPost {
    fn new(id: usize, path: PathBuf) -> Self {
        // Init empty post
        let mut ret = Self::default();
        ret.id = id;
        ret.url_name = path.file_stem().unwrap().to_str().unwrap().to_string();

        // fill in struct from draft
        let md_file = fs::read_to_string(path.to_str().unwrap()).expect("Could not read draft");
        let parse_tree = Draft::parse(Rule::draft, &md_file)
            .expect("Could not parse draft")
            .next()
            .unwrap();
        // cycle through each attribute
        // unwrap is safe - if it parsed, there are between 3 and 6
        let mut parse_tree_inner = parse_tree.into_inner();

        // set header
        let header = parse_tree_inner.next().unwrap();
        let attributes = header.into_inner();
        for attr in attributes {
            let mut name: &str = "";
            let mut value: &str = "";
            for attr_part in attr.into_inner() {
                match attr_part.as_rule() {
                    Rule::key => name = attr_part.as_str(),
                    Rule::value => value = attr_part.as_str(),
                    _ => unreachable!(),
                }
            }
            match name {
                "cover_image" => ret.cover_image = Some(value.to_string()),
                "description" => ret.description = Some(value.to_string()),
                "edited" => ret.edited = Some(value.to_string()),
                "published" => {
                    ret.published = match value {
                        "true" => true,
                        _ => false,
                    }
                }
                "title" => ret.title = value.to_string(),
                _ => {}
            }
        }

        // set body
        let body = parse_tree_inner.next().unwrap();
        ret.markdown = body.as_str().to_string();

        // done
        ret
    }
}
```

Now that the build script has each blogpost with properly organized metadata in memory, we can tell it how to fill in the template we need:

```rust
    fn write_template(&self) -> Result<(), std::io::Error> {
        let mut file = std::fs::File::create(&format!("templates/post_{}.html", self.url_name))?;
        let parser = pulldown_cmark::Parser::new(&self.markdown);
        let mut html = String::new();
        html::push_html(&mut html, parser);
        writeln!(file, "{{#  This file was auto-generated by build.rs #}}")?;
        writeln!(file, "{{% extends \"skel.html\" %}}")?;
        writeln!(file, "{{% block title %}}{}{{% endblock %}}", self.title)?;
        writeln!(file, "{{% block content %}}{}{{% endblock %}}", html)?;
        Ok(())
    }
```

The driver code just has to loop through all the scraped posts and call this method. We also need a struct for Askama to render, too, though - as long as we can generate a Rust module, we can generate those too:

```rust
    fn struct_name(&self) -> String {
        format!("Blog{}Template", self.id)
    }
    fn write_template_struct(&self, file: &mut std::fs::File) -> Result<(), std::io::Error> {
        writeln!(file, "#[derive(Template)]")?;
        writeln!(file, "#[template(path = \"post_{}.html\")]", self.url_name)?;
        writeln!(file, "pub struct {} {{", &self.struct_name())?;
        writeln!(file, "    links: &'static [Hyperlink],")?;
        writeln!(file, "}}")?;
        writeln!(file, "impl Default for {} {{", &self.struct_name())?;
        writeln!(file, "    fn default() -> Self {{")?;
        writeln!(file, "        Self {{ links: &NAV }}")?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}\n")?;
        Ok(())
    }
```

This will plop something like this is `src/blog.rs`:

```rust
#[derive(Template)]
#[template(path = "post_cool-post.html")]
pub struct Blog0Template {
    links: &'static [Hyperlink],
}
impl Default for Blog0Template {
    fn default() -> Self {
        Self { links: &NAV }
    }
}
```

I used the same `writeln!()` strategy to autogenerate a handler with a bunch of match arms, one per struct:

```rust
pub async fn blog_handler(path_str: &str) -> HandlerResult {
    match path_str {
        "/cool-post" => {
            string_handler(
                &Blog0Template::default()
                    .render()
                    .expect("Should render markup"),
                "text/html",
                None,
            )
            .await
        }
        // etc ...
        _ => four_oh_four().await,
    }
}
```

As well as scrape some of the metadata to build a static value holding information to create the post listing page:

```rust
lazy_static! {
    pub static ref LINKINFO: BlogLinkInfo = {
        let mut ret = BlogLinkInfo::default();
        ret.posts.push(LinkInfo {
            id: 0,
            title: "Cool Post",
            url_name: "cool-post",
        });
        // etc...
}
```

Pulling it all together just looks like a bunch of Rust, which, you know, it is and all - here's a partial snippet:

```rust
fn generate_handler(blog: &Blog, file: &mut std::fs::File) -> Result<(), std::io::Error> {
    writeln!(file, "pub async fn blog_handler(path_str: &str) -> HandlerResult {{")?;
    writeln!(file, "    match path_str {{")?;
    for p in &blog.posts {
        p.write_handler_match_arm(file)?;
    }
    writeln!(file, "        _ => four_oh_four().await,")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;
    Ok(())
}

fn generate_module(blog: &Blog) -> Result<(), std::io::Error> {
    let mut module = fs::File::create(&format!("src/{}.rs", MODULE_NAME))?;

    write_imports(&mut module)?;

    write_link_info_type(&mut module)?;
    write_blog_link_info_type(&mut module)?;

    generate_blog_link_info(blog, &mut module)?;
    generate_template_structs(blog, &mut module)?;
    generate_posts(blog)?;
    generate_handler(blog, &mut module)?;

    Ok(())
}
```

Now when askama's procedural macros wake up at the beginning of compiling your actual crate, all of the template files in `templates/` and Rust code you need to use each `*.md` file in your project has been generated, ready to be called from the rest of your crate:

```rust
// src/blog.rs
// this module was auto-generated by build.rs
use crate::{
    config::NAV,
    handlers::{four_oh_four, string_handler, HandlerResult},
    types::Hyperlink,
};
use askama::Template;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy)]
pub struct LinkInfo {
    pub id: usize,
    pub url_name: &'static str,
    pub title: &'static str,
}

#[derive(Debug, Default)]
pub struct BlogLinkInfo {
    pub posts: Vec<LinkInfo>,
}

lazy_static! {
    pub static ref LINKINFO: BlogLinkInfo = {
        let mut ret = BlogLinkInfo::default();
        ret.posts.push(LinkInfo {
            id: 0,
            title: "Cool Post",
            url_name: "cool-post",
        });
        ret.posts.push(LinkInfo {
            id: 1,
            title: "Kind Of Alright Post",
            url_name: "honestly-meh",
        });
        ret
    };
}

#[derive(Template)]
#[template(path = "post_cool-post.html")]
pub struct Blog0Template {
    links: &'static [Hyperlink],
}
impl Default for Blog0Template {
    fn default() -> Self {
        Self { links: &NAV }
    }
}

#[derive(Template)]
#[template(path = "post_honestly-meh.html")]
pub struct Blog1Template {
    links: &'static [Hyperlink],
}
impl Default for Blog1Template {
    fn default() -> Self {
        Self { links: &NAV }
    }
}

pub async fn blog_handler(path_str: &str) -> HandlerResult {
    match path_str {
        "/cool-post" => {
            string_handler(
                &Blog0Template::default()
                    .render()
                    .expect("Should render markup"),
                "text/html",
                None,
            )
            .await
        }
        "/honestly-meh" => {
            string_handler(
                &Blog1Template::default()
                    .render()
                    .expect("Should render markup"),
                "text/html",
                None,
            )
            .await
        }
        _ => four_oh_four().await,
    }
}
```

The build script will re-make this file to match every time you change the files in this directory, so you only ever have to worry about the markdown files to manage your blog.

...You know, like a static site thingamajigger or something. Crazy.

Build scripts are pretty powerful - what have _you_ used them for?

_Photo by Scott Blake on Unsplash_
