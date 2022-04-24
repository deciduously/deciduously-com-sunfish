---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--q5Ws5y01--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://thepracticaldev.s3.amazonaws.com/i/9c5r1t5vumxukim530px.jpg
date: 2019-04-03T12:00:00.000Z
title: Build You A Markov Chain In Rust (Or Whatever)
tags:
  - beginners
  - rust
  - tutorial
---

I've found a great way to ensure I've grokked a thing is to write it up in Rust. In that spirit, this post covers a translation of the program in [this post](http://theorangeduck.com/page/17-line-markov-chain) by [orangeduck](http://theorangeduck.com/page/about) into Rust, with a minor difference and some extra explanation including about why writing Rust is the way it is. Depending on your comfort level it may be skimmable, especially if you already got some Rust in you. It will only take us 70 extra lines!

A [Markov chain](https://en.wikipedia.org/wiki/Markov_chain) can be used to generate realistic(ish) sounding random text based on a sample input. The Wikipedia article is somewhat opaque, as Wikipedia can tend to be, but at its heart it's a very simple concept in which the next word is chosen based entirely on the current two words. It's surprisingly simple (at least, I was surprised at how easy it was) and yet generates some real-sounding(ish) text with minimal effort. For a fun example of this in action, check out the subreddit [/r/SubredditSimulator](https://www.reddit.com/r/SubredditSimulator/). All of the posts and comments found there are generated using Markov chains using their respective subreddits as input data.

# On Your Marks

If you're just here for the Markov Chain algorithm and not the Rust, skip down to "The Algorithm" in the **Markov!** section.

This project requires stable [Rust](https://rustup.rs/). Go there to get it if you need, and then spin up a project:

```
$ cargo new markov
$ cd markov/
```

# Get Set

Before hopping in, a quick 'n' dirty CLI would be nice for playing around with different inputs. Luckily, Rust has a great option in [structopt](https://github.com/TeXitoi/structopt). From the project root:

I like to use `cargo-add` from rapid experimentation: `$ cargo install cargo-add` from any dir to add it to cargo.

```
$ cargo add structopt
```

As the name implies this crate makes it easy to define an interface by simply defining a struct. It uses macros to handle all the code generation required. Add the following to the top of `src/main.rs`:

```rust
use std::{error::Error, path::PathBuf, str::FromStr};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "markov")]
struct Opt {
    /// Input text file
    #[structopt(short = "i", long = "input")]
    input: Option<PathBuf>,
    /// Output length
    #[structopt(short = "l", long = "length")]
    length: Option<u32>,
}
```

We're auto-deriving two [traits](https://doc.rust-lang.org/book/ch10-02-traits.html), `StructOpt` and `Debug`. The latter is like `toString()` from Java, it creates a string representation of the struct, and the StructOpt one is going to give us methods like `from_args()` to instantiate it from the command line arguments automatically. It also leverages a special custom tag `#[structopt]` which is used to configure the behavior of this macro.

The doc comments with the three slashes end up in the help string this crate will generate for us. An example format for this struct would be something like `./markov -i poetry.txt -l 500`. You can use cargo directly with `cargo run -- -i poetry.txt -l 500`. The long names are used with two dashes, like `--length`.

Each field has a type like `Option<T>`, which means if either is omitted when the program is invoked this struct will just hold a `None`. If you're not used to that syntax, any time you see a single capital letter it stands for a generic type. A real value you use would be specifically typed, such as `Option<String>`.

A `PathBuf` is a fancy `String` with [cross-platform path abstractions](https://doc.rust-lang.org/std/path/index.html) built in. You can `push()` to them and traverse them the same way in Rust code on whichever platform it runs.

Otherwise, though, this is just a regular old Rust struct, it's just got a fancy custom `impl StructOpt {}` block generated for us.

Now, replace the template `println!` call given in `main()` with:

```rust
fn main() {
    let opt = Opt::from_args();
    let filename = opt
        .input
        .unwrap_or_else(|| PathBuf::from_str("poetry.txt").unwrap());
    let length = opt.length.unwrap_or(350);

    if let Err(e) = run(filename, length) {
        eprintln!("Error: {}", e);
        ::std::process::exit(1);
    };
}
```

If you're not new to Rust, that's probably fine and dandy. If you are, let's unpack it a little.

First, we generate the struct itself from whatever was passed on the command line. In the line `let opt = Opt::from_args()`, `Opt` is the struct we defined just above, flexing its fancy code-gen'd `from_args()` method. If this program were invoked as `cargo run -- -i poetry.txt -l 350` example from above, we now have stored in the variable `opt`:

```rust
Opt(
    input: Some(PathBuf(inner: "poetry.txt")),
    length: Some(350u32),
)
```

All in-memory data structures will be presented in [RON](https://github.com/ron-rs/ron).

Note that the guts of `PathBuf` are omitted - it's an [`OsString`](https://doc.rust-lang.org/std/ffi/struct.OsString.html) if you're curious but we just care it's a `PathBuf`.

The first thing to do is get something more concrete from those options to pass in to the program. Using `unwrap_or_else()` is a great way to do this. If the value is a `Some(thing)` it returns `thing`, and if it's `None` it calls the passed closure, and it's gotta be one of those two. If you just need a default value and not a function call, you can just use `unwrap_or()`.

That `from_str` call we do to get our default `"poetry.txt"` `&str` value into a `PathBuf` is part of the `FromStr` trait and only works when that trait is in scope. It's an operation that can fail - for example, with a malformed path - so it returns a `Result<T, E>`. This type acts like `Either` from Haskell if you're familiar, it either contains an `Ok(something: T)` or an `Err(error: E)` value. You can get at the `T` of those with `unwrap()` if you're sure you'll have a successful `Ok` return value. We know this one won't fail because we just made the input ourselves and it's not a malformed path, just a filename with an extension. If you don't have something valid this will panic and crash. It's almost always better to use something like `unwrap_or()` or [pattern matching](https://doc.rust-lang.org/book/ch06-02-match.html) to deal with the alternative cleanly!

Next we pass both in to an error-checked function. It's good practice to take advantage of Rust's error handling for as much of your program as possible - this is a good way to force it! The `if let` syntax is a way of capturing any error. Our `run()` function here is also going to return a `Result<T, E>` - they're a bit of a theme in Rust. It's sort of like a big try/catch embedded in the type system. When called in an `if let`, if it ends up returning an `Ok(_)` nothing will happen. If anything inside returns an error of any type (more on that in a moment) at any point, we'll execute the code path in this if block. It uses destructuring - this line is saying that if the return value of `run()` can be destructured into an `Err(e)`, run this code. The only alternative variant is `Ok(val)` - in which case we know everything went fine, and there's no action to take. If we wanted to do something else, we could have included an `else {}` block as well. This error catch will use `eprintln!` to display the specific error information returned on `stderr` and end the program with an error code of 1, indicating it was not successful.

Of course, now we need a properly typed `run()` function. Here's a stub, just to get us to compile:

```rust
fn run(input: PathBuf, length: u32) -> Result<(), Box<dyn Error>> {
    Ok(())
}
```

The meat of our program expects concrete values, not `Option<T>`, and like good responsible Rustaceans we return a `Result<T, E>`, specifically a `Result<(), Box<dyn Error>`. Our success type, `()` stands for `unit` which is the empty tuple, akin to `void`. This demo will just be outputting our random text to `stdout`, there's no value to return. If you wanted to store the generated text and pass it to another part of your program, this might look like `Result<String, Box<dynError>>`. The `Box<dyn Error>` type we're using for the Error type merits a little more explaining.

A [`Box<T>`](https://doc.rust-lang.org/std/boxed/index.html) is a boxed value - a basic heap-allocated value of type `T`. Specifically the `Box` is a pointer to it, but a Rust-y smart pointer that knows about ownership and borrowing. It's got a big name but it's just a pointer, nothing else. This is useful because the `Box` has a size known at compile time, even if the value it points to may not. The thing in the box with the `dyn Trait` syntax is a [_trait object_](https://doc.rust-lang.org/book/ch17-02-trait-objects.html). `Error` from `std::error` is a trait that many different types of more specific errors types implement. Using `dyn Error` we cover any type that implements the `Error` trait. This allows us to pass and catch all the different types of errors in one function easily.

If you're brand new to Rust and that was a little too breezy, you're in for a real treat outside the scope of this post but don't worry - this part isn't necessary to understand the Markov bits below! It's just some Rust boilerplate for clean and happy error handling without much setup.

Let's fire it up! See if the help string is working with:

```
$ cargo run -- -h
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/markov -h`
markov 0.1.0
you <you@you.cool>

USAGE:
    markov [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>      Input text file
    -l, --length <length>    Output length
```

Groovy! Thanks, structopt. Don't forget:

```
$ git init
$ git add .
$ git commit -m "Initial commit"
```

# Markov!

## The Algorithm

The idea of this method of text generation is to choose the next word based entirely on the current two words, using only words that appear after them in the source text. This algorithm never cares about more than two words at a time - it just knows all the possible options that come after that particular word duo. Before we can start spewing out beautiful nonsense, then, we need to catalog the input.

We can implement this in Rust with a [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html) as a lookup table. One word doesn't quite give us enough context for realistic generation, though, so the keys of this hashmap will actually be combinations of two words. These keys can be any type that implement the [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html) and [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html) traits, and a tuple of two [string slices](https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices) `(&str, &str)` works just fine. We'll then store for the corresponding value every word in our source text that ever follows the combination of those two. This way we can look up words likely to come next based on the current two words we have in our text.

Here's a concrete example what our hashmap might look like if built from the source text "bears are big and bears are furry and bears are strong":

```rust
{
    ("bears", "are"): ["big", "furry", "strong"],
    ("and", "bears"): ["are"],
    ("are", "big"): ["and"],
    ("are", "furry"): ["and"],
    ("big", "and"): ["bears"],
    ("furry", "and"): ["bears"],
}
```

This source text would not provide terribly interesting output, but it demonstrates how this will work on a larger scale. First, you pick a random spot in the source text. Let's go for "bears are" (randomly, I promise). Stepping through a few iterations:

1. Output: "bears are". Look up `("bears", "are")`. Randomly select "furry" from the three stored options and append it to the output.

2. Output: "bears are furry". Look up `("are", "furry")`. The only option stored is "and". Append to output.

3. Output: "bears are furry and". Look up `("big", "and")`, append the only option "bears".

4. Output: "bears are furry and bears". Look up `("and, bears")`, append "are".

5. Output: "bears are furry and bears are". Look up `("bears", "are")`. Randomly select "strong" from the three stored options.

6. Output: "bears are furry and bears are strong". Look up `("are", "strong")`, append "and".

And so forth. We generate a random output string of arbitrary length that resembles the source text. The words will always sort of seem to make sense after one another as long as your input text did, and will actually sort of emulate the style. Take a moment now and go back to the [orangeduck python post](http://theorangeduck.com/page/17-line-markov-chain) to grab the poetry set he created. It's quite large (over 1.8 million lines!) and distributed as a zip file. Unzip it into your project root as `poetry.txt`. It's a great one because it's got a few different languages and several styles of poetry so successive runs will usually give you something pretty unique.

Our next word of the randomly generated text will always be pulled from this lookup table of words that do follow our current two words in the real text, which will (often) result in real-sounding sentences getting strung together even though each run through the loop is only ever aware of exactly where it is and nothing else. On each iteration we perform a lookup of the proper tuple and select one of the options stored there at random. Rinse and repeat for the length of the desired text! Boom, nonsense. The bigger the source text, the more interesting the output.

## Read Them In

The first step in building this is to read in the source text. First, tweak your `std` imports:

```rust
use std::{
    collections::HashMap, error::Error, fs::OpenOptions, io::Read, path::PathBuf, str::FromStr,
};
```

This function will accept a `PathBuf` (which we've collected from the user already) and attempt to return the file's contents as a string:

```rust
fn read_file(filename: PathBuf) -> Result<String, Box<dyn Error>> {
    let mut file = OpenOptions::new().read(true).open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

That return type is familiar from up above - this operation can fail if, for instance, there is no file at the path specified, so we're wrapping it in a `Result<T, E>`. The `T` here is the successfully read `String`, and our `E` is that nifty trait object to catch any and all error types that may get thrown along the way.

Any variable that we need to mutate has to be explicitly marked as such with `mut`. By default any attempt to mutate the value stored in a `let` binding will fail to compile.

In the first line it attempts to open the file at the path passed in with read permissions only. It's got a question mark at the end, which is a shorthand way of saying automatically unwrapping the return value if it's an `Ok(val)` and early-returning this function with `Err(the error returned)`. Quite handy! The expansion would look something like:

```rust
let file = match OpenOptions::new().read(true).open(filename) {
    Ok(f) => f,
    Err(e) => return Err(e),
}
```

This is pretty reasonable behavior as syntactic sugar goes. It only works inside a function that returns a `Result<T, E>`, though, which justifies all this hullabaloo. You can't use `?` in `main()`, for instance.

The `read_to_string()` method from the [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) trait also uses `?` to catch any errors that may happen. If everything has succeeded, our source text is sitting snugly inside this massive string, so we can wrap it in an `Ok()` and get going.

As an aside, I often reach for [`BufReader`](https://doc.rust-lang.org/std/io/struct.BufReader.html) by instinct. This is a use case in which it won't help us, and actually might slow us down. We're just reading this very large file in once to a single `String`, so we'd rather avoid the extra allocations doing a buffered read would add.

Go ahead and pop it in `run()`:

```rust
fn run(input: PathBuf, length: u32) -> Result<(), Box<dyn Error>> {
    let file_str = read_file(input)?;
    println!("{}", file_str);
    Ok(())
}
```

If you've got `poetry.txt` in place, `cargo run` should now display the entire contents, at least until you get bored and interrupt it.

## Split Them Up

We can't work with just a massive `String`, though, we've got to split it in to individual words. We want to preserve things like newlines for this operation. As orangeduck points out, in poetry especially line endings are part of the structure the output should resemble. To do this we'll use a regular expression via the regex crate: `$ cargo add regex`.

Here's a function that will carry out this operation:

```rust
use regex::Regex;
//..
fn split_words(w: &str) -> Vec<&str> {
    let spaces_re = Regex::new(r" +").unwrap();
    spaces_re.split(w).collect::<Vec<&str>>()
}
```

This function is going to allocate a new `Vec`, but inside we're only going to store references to our original file string. We don't need to change the input, just look at it in order to build this vector. By just taking a reference to the string in the argument, we don't move ownership of the input away from the original binding. Building this list causes no new copying or allocation involved beyond the `Vec` structure itself.

The first line defines the regular expression - instead of wrapping this whole function in a `Result`, I'm just promising the compiler (and you) that `r" +"` constitutes a valid `Regex` and using a plain old `unwrap()` on the `Result` that `Regex::new()` returns. Creating a new `Regex` would return an error if passed an invalid regular expression, in which case our `unwrap()` call would panic. We know this won't panic, though, it will just match one or more spaces ignoring anything else like tabs and newlines. Different inputs may require different regexes for optimal output. Then we return the result of calling the `split()` method using this regex and then `collect()` to return the resulting [`Iterator`](https://doc.rust-lang.org/std/iter/index.html) as a `Vec<&str>`.

## Get Organized

To build the lookup table, we want to look at three words at a time. The first two will be used for the key, and the third will be appended to the list of possible options. That is, we're going to want to look at the first, second, and third word, then the second, third, and fourth word, then the third, fourth, and fifth word, and so on. The most concise way to build a nice handy iterator for this is the `izip!()` macro found in the [`itertools`](https://docs.rs/itertools/0.8.0/itertools/) crate: `$ cargo add itertools`.

```rust
#[macro_use]
extern crate itertools;
//..
fn build_table(words: Vec<&str>) -> HashMap<(&str, &str), Vec<&str>> {
    let mut ret = HashMap::new();
    for (w0, w1, w2) in izip!(&words, &words[1..], &words[2..]) {
        // add w2 to the key (w0, w1)
        let current = ret.entry((*w0, *w1)).or_insert_with(Vec::new);
        current.push(*w2);
    }
    ret
}
```

We need to pull in the `izip!()` macro with the `#[macro_use]` tag, first. We then use slices to build sublists with the proper offsets. This `for` loop will end up iterating through each three word triple in the source text.

Inside the loop we use the [`Entry API`](https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html#method.entry) to look up the key from the first two words of the triple - `("bears", "are")`, for example. If no such key is found the `or_insert_with()` call will create it for us with an empty `Vec` ready to go, so that no matter wht we can `push` the third word to it in the next line. Once this loop completes, we've built the data structure described in the step-through above.

It's possible to skip the itertools dependency if you like, but the code comes out a little clunkier - the built-in `zip` method can only zip two iterators, so you've got to call it twice and then combine everything yourself:

```rust
fn build_table_no_itertools(words: Vec<&str>) -> HashMap<(&str, &str), Vec<&str>> {
    let mut ret = HashMap::new();
    let all_words = &words[..];
    let offset_1 = &words[1..];
    let offset_2 = &words[2..];
    for (w0, w1, w2) in all_words
        .iter()
        .zip(offset_1.iter())
        .zip(offset_2.iter())
        .map(|((a, b), c)| (a, b, c))
    {
        // add w2 to the key (w0, w1)
        let current = ret.entry((*w0, *w1)).or_insert_with(Vec::new);
        current.push(*w2);
    }
    ret
}
```

This function works as a drop-in replacement with no external dependency required but you're sacrificing readiblity - this takes much longer to stare at before you understand it's just zipping together three iterators. I'd much rather add the dependency, `izip!()` is much nicer.

## Spit Them Out

Now that everything's set up, we can just perform as many lookups as specified and string everything together. We need something to start, with though, so first we'll just select a random starting point from the source text. We need one more crate to accomplish this: `$ cargo add rand`.

```rust
use rand::{seq::SliceRandom, thread_rng, Rng};
//..
fn run(input: PathBuf, length: u32) -> Result<(), Box<dyn Error>> {
    let file_str = read_file(input)?;
    let words = split_words(&file_str);

    let mut rng = thread_rng();
    let i = rng.gen_range(0, words.len() - 3);

    let mut w0 = words[i];
    let mut w1 = words[i + 1];
    let mut w2 = words[i + 2];
}
```

We'll reuse these variables inside the loop.

There's a gotcha in this implementation, though. As it's written, `build_table` is taking ownership of our `words` vector. That means that after we call it, we can't use `words` again. Luckily, there is nothing stopping us from simply picking our starting location _before_ we build the table. We'll need to actually call the function directly below the above setup:

```rust
let lookup = build_table(words);
```

Now everything's in place for the generation loop:

```rust
    // each iteration, print current word and then a space, and update our words
    for _ in 0..length {
        // append to output
        print!("{} ", w2);

        // choose the next word
        w2 = &lookup[&(w0, w1)].choose(&mut rng).unwrap();
        w0 = w1;
        w1 = w2;
    }
```

We just print out whatever we've got stored in `w2`, add a space, and then use `w0` and `w1` to look up the next word. Once we've selected in, we need to update `w0` and `w1`, advancing our cursor to the next triple.

The `unwrap()` call here is also safe because we'll never have a key corresponding to a zero-length list. Every time we create a new key, we immediately push the following word to it. The last iteration of the loop covers the last three words, so we'll always be able to do so.

That's the whole program - fire it up with `cargo run --release`. We provided sane default argument values, so you don't need to use the command line options we defined unless you want to.

This is my favorite run so far on the poetry set:

```
$ cargo run --release
   Compiling markov v0.1.0 (/home/ben/code/markov)
    Finished release [optimized] target(s) in 0.58s
     Running `target/release/markov`
An actor experiences
Other peoples lives
Through a metamorphosis of mind

Words sifted through a Forest, beneath the blowing gale,
The waves have now the year of 1897, and on like that.
I can't abear it. I killed last night.

I wonder, 'struth, I wonder if the listener please,
A most important thing;
But Fortune to a thousand times, but I
 Would have him with his prophetic bill.
The great Colosse, erect to Memory;
And what the royal feast!
See here the blue night, railway stations.

The water and fire his courage on despair
And utter dissolution, as the love of slaughter;
Many indeed are the men
With spears gathering at his feet: and my evening hours.

Last evening when it rests,
Leaves to be
Of work may be shared by not crossing the line,
Though that same morning officers and men.

Continues yet the dream
```

Continues, yet, the dream...

See [this repo](https://github.com/deciduously/markov) for the full code.
