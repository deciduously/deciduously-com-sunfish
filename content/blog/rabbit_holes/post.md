---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--Y4hPNjmr--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://thepracticaldev.s3.amazonaws.com/i/secp1t029jvyrsftd5ew.jpg
date: 2019-08-30T12:00:00.000Z
title: In And Out Of Rabbit Holes
tags:
  - devjournal
  - rust
  - webassembly
---

In the past week, I've fallen into the following rabbit holes wholly outside of the necessary requirements while getting my new side project off the ground. Here's how I've gotten out.

## Dynamic dispatch in Rust

Rust binaries can bloat pretty quickly, and one way to mitigate this is to prefer dynamic dispatch (i.e. `Box<dyn Trait>`) to monomorphization (`fn<T: Trait>(obj: T) {}`). In brief, the latter generic function syntax will create a completely separate function implementation in your binary for each type it's used with, and will select the properly typed version at compile time. The former will not bind this function call at compile time, and instead merely ensure that the trait indicated is implemented. It will then _dynamically_ bind the method call at runtime for the type it's ultimately called from.

I rewrote all my generic functions to take trait objects and my bundle got smaller, YMMV.

As it turns out, this is not an insignificant rewrite. One thing to note is that while a `Box<dyn Trait>` is `Sized` (because a `Box` is sized), a _trait object_ (the `dyn Trait` part) by definition cannot be. The type is unknown, all we do know is that it implements this interface, and the method calls will be dispatched dynamically at runtime. This means you'll need to do some manual plumbing if you want to automate trait-to-trait machinery, and be prepared to get up close and personal with the borrow checker. I'm still not quite sure I've got it right, but it _does_ compile.

As an example, I've got two related traits, `Drawable` and `Widget`. A `Drawable` is a type that knows how to paint itself to the canvas, and a `Widget` is an organizational component that contains a 2d grid of child `Widget`s.

Some widgets are also `Drawable`, of course, so that eventually something gets drawn to the screen. The eventual idea is to provide a set of `Widget`s generic enough that games built on top of this library (or whatever) never need to manually implement `Drawable`, they can just compose `Widgets` like `Text` and `Button` and `Area` which handle all the details, and I'm using the game I'm building on top of it to drive what widgets need writing.

The easiest way I could figure out how to make this work is to have implementors of this trait write a function that returns some other type, `MountedWidget`:

```rust
/// Trait representing things that can be drawn to the canvas
pub trait Drawable {
    /// Draw this game element with the given top left corner
    fn draw_at(&self, top_left: Point, w: WindowPtr) -> Result<Point>;
    /// Get the Region of the bounding box of this drawable
    fn get_region(&self, top_left: Point, w: WindowPtr) -> Result<Region>;
}

/// Trait representing sets of 0 or more Drawables
/// Each one can have variable number rows and elements in each row
pub trait Widget {
    /// Make this object into a Widget
    fn mount_widget(&self) -> MountedWidget;
```

The `MountedWidget` provides its own `Drawable` implementation that knows how to methodically draw its way through a grid of children, and optionally contain a raw `Drawable` itself:

```rust
/// A container struct for a widget
pub struct MountedWidget {
    children: Vec<Vec<Box<dyn Widget>>>,
    drawable: Option<Box<dyn Drawable>>,
}

```

Part of me thinks I should be able to streamline this even further and avoid allocating an intermediary struct, but this setup got me something working. The unfortunate part is that as written every widget gets re-created and dropped for every frame - clearly the way to go is to mount it all first and adjust as needed but it's a start, at least.

## Crate-in-a-crate

I've said it before and I'll say it again: `cargo` is the crème de la crème of package managers. Everyone else is missing out.

One way I hoped to leverage it was by pulling out my Canvas mounting and drawing stuff as its own crate, and letting incremental compilation cache the build separately. As it turns out, this was really really easy. Here's what a standard library with three modules might look like, directory-wise:

```
$ tree
.
├── Cargo.toml
├── LICENSE
├── README.md
└── src
    ├── drawing.rs
    ├── game.rs
    └── lib.rs

1 directory, 6 files

```

To turn your "drawing" module into its own crate, make it look like this:

```
.
├── Cargo.toml
├── LICENSE
├── README.md
└── src
    ├── drawing
    │   ├── Cargo.toml
    │   ├── LICENSE
    │   ├── README.md
    │   └── src
    │       └── lib.rs
    ├── game.rs
    └── lib.rs

```

That's all! In `Cargo.toml` for the parent crate, just add the dependency:

```toml
[dependencies.drawing]
path = "src/drawing"
```

It could not be easier, and the efficiency gained makes a real difference especially with these hefty WASM builds. Feel free to grab that directory and plop it anywhere you like (i.e. hosted in a git repo), you can point your `Cargo.toml` where you need.

## Size optimization

This [link](https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code-size) in the RustWasm book has some good tips. You need to install the [Binaryen](https://github.com/WebAssembly/binaryen) toolkit to get the full benefit - it can run further speed and size optimizations on your compiled WASM output beyond what LLVM will do via `rustc`. You'll need to have [`cmake`](https://cmake.org/) installed, which is available in all major repositories (`apt`,`homebrew`,`chocolatey`, etc.)

```
$ git clone https://github.com/webassembly/binaryen
$ cmake . && make
```

It will take a little while. There's several frontends we won't be using, see the readme for usage. I just symlinked `wasm-opt` to my user's path:

```
$ ln -s /home/ben/code/extern/binaryen/bin/wasm-opt /home/ben/.local/bin/
```

I then wrote a script to handle the `wasm-opt` call:

```sh
#!/bin/bash
PKGDIR='pkg'
BINARY='fivedice_bg'
WASM="$PKGDIR/$BINARY.wasm"

function wasm_size {
    wc -c $1
}

function echo_size {
    echo "$(eval wasm_size $1)"
}

function extract_size {
    wasm_size $1 | sed 's/^\([0-9]\+\).*/\1/'
}

# $1 = target $2 = focus $3 = level
function shrink {
    ARG='-O'
    if [ "$2" = "size" ]; then
        if [ "$3" = "aggro" ]; then
            ARG="${ARG}z"
        else
            ARG="${ARG}s"
        fi
    else
        if [ "$3" = "aggro" ]; then
            ARG="${ARG}3"
        fi
    fi
    COMMAND="wasm-opt $ARG -o $1 $WASM"
    echo $COMMAND
    eval $COMMAND
}

function choose_smaller {
    NORMAL='_normal'
    AGGRO='_aggressve'
    NORMAL_TARGET="${PKGDIR}/${BINARY}${NORMAL}.wasm"
    AGGRO_TARGET="${PKGDIR}/${BINARY}${AGGRO}.wasm"
    shrink $NORMAL_TARGET $2 $3
    NORMAL_SIZE="$(eval extract_size $NORMAL_TARGET)"
    shrink $AGGRO_TARGET $2 $3
    AGGRO_SIZE="$(eval extract_size $AGGRO_TARGET)"
    if [ $NORMAL_SIZE -lt $AGGRO_SIZE ]; then
        echo "Normal settings smaller, saving..."; mv $NORMAL_TARGET $WASM; rm $AGGRO_TARGET;
    else
        echo "Aggressive settings smaller, saving..."; mv $AGGRO_TARGET $WASM; rm $NORMAL_TARGET;
    fi
}

# parse args
for i in "$@"
do
case $i in
    -f=*|--focus=*)
    FOCUS="${i#*=}"
    shift
    ;;
    -l=*|--level=*)
    LEVEL="${i#*=}"
    shift
    ;;
    *)
    # unknown option
    ;;
esac
done
# last line is target, non-opt, no equals sign
if [ -n $1 ]; then
    TARGET=$1
fi

echo_size $WASM
if [ -z $FOCUS ]; then
    FOCUS_STR='speed'
else
    FOCUS_STR=$FOCUS
fi
echo "Shrinking, optimizing for ${FOCUS_STR}."
if [ "$LEVEL" = "aggro" ]; then
    echo "Using aggressive optimizations."
fi
if [ "$FOCUS" = "size" ]; then
    choose_smaller $1
else
    shrink $WASM $FOCUS $LEVEL
fi
echo_size $WASM

exit
```

There's also a Makefile to call it for me:

```makefile
.PHONY: all clean help

RUSTCLEAN=cargo clean
RUST=wasm-pack build
PKGDIR=pkg
EXEC=fivedice_bg.wasm
OPT=./shrink-wasm.sh -f=speed -l=aggro

all: $(PKGDIR)/$(EXEC)
    $(OPT)

$(PKGDIR)/$(EXEC):
    $(RUST)

clean:
    $(RUSTCLEAN)

help:
    @echo "Usage: make {all|clean|help}" 1>&2 && false
```

The script passes the proper args to `wasm-opt` for either size or speed, and aggressive or normal. If you choose size, it runs it both with aggressive or not and saves the smaller of the two. To tweak it, set the options in the OPT line of the makefile. Seems like I should be able to get some mileage out of this setup for now.

This project had a rabbit-hole-in-a-rabbit-hole writing the `extract_size` function in the bash script. The `sed` call was my first solution. Then I decided for some odd reason I wanted to try to do it without a call or subshell, using string substitutions like the argument matching or something. What a waste of a morning. I'm sure there's a simple solution staring at me in the face, but I didn't find it and even if I had it would have made _no difference_. Why do we do this to ourselves?

## Debugging

This is less of a rabbit hole so much as a way around them.

Use [console_error_panic_hook](https://github.com/rustwasm/console_error_panic_hook). With it, when your module panics you get actual useful error output to the browser console instead of just "Unreachable executed". This is obviously an improvement.

Also, `wasm-pack build` runs a release build by default, with no debug symbols. When debugging, use `wasm-pack build --debug` or add `debug = true` to your `Cargo.toml`. Now your errors will actually have the name of the Rust function that tripped instead of `webassembly[37]` or some nonsense. I didn't realize this for too long and thought debugging WASM apps was just like that. It _doesn't have to be like that_.

## Stay tuned

Even with all this time....alternatively spent, the _thing does the thing_ and my little grid of test widgets is accurately painted to the canvas, so I chalk it up as a successful week. Up next, Ben attempts to provide a procedural macro-style DSL! This should be a _mess_, you won't want to miss it.

_Photo by Gary Bendig on Unsplash_
