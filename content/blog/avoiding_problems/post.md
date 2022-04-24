---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--6vU7yfM1--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://thepracticaldev.s3.amazonaws.com/i/5p3ouxfw3jiepppsrosu.jpg
date: 2019-05-07T12:00:00.000Z
title: Solving Problems By Avoiding Them
tags:
  - rust
  - beginners
  - lisp
  - help
---

I just saved myself a _ton_ of heartache by doing something a lot easier instead.

I think. Either that, or I introduced a weird brittle workaround that I'll come to regret down the line. Is there some sort of gut check you use to tell which is which?

I'm working on translating (another) [orangeduck](http://theorangeduck.com/page/about) thing into Rust, this time his [Build Your Own Lisp](http://www.buildyourownlisp.com/) book which also functions as a great crash course in C. Of course, C and Rust are different in some important ways, so the translation is sometimes straightforward and other times very much not. I was following along with the book more or less without a hitch until we got to [Scoped Environments](http://www.buildyourownlisp.com/chapter12_functions#parent_environment).

This post is not a tutorial or walk-through of my translation, but the full code can be found [here](https://github.com/deciduously/blispr) for context.

## The Task

It's helpful to understand the high-level overview of the program we need to write. This program will take a string as input and attempt to evaluate the result. We need to _parse_ the string into a tree of semantically tagged lexical tokens, _read_ this parse tree of tokens into a structure called an [Abstract Syntax Tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree), and then _evaluate_ that AST. To do this, we'll need to semantically tag each element so that our program can methodically work its way through, understanding what each part is.

If you're not familiar with those data structures, it's not as complicated as it may sound (and I shelled parsing out to [a library](https://pest.rs/)). For a small concrete example, let's look at the input string `+ 2 (* 3 4) 5`. To work with this input, we need to build a an AST structure like the following:

```rust
S-Expression(
    Symbol("+"),
    Number(2),
    S-Expression(
        Symbol("*"),
        Number(3),
        Number(4),
    ),
    Number(5),
)
```

The whole program is represented as an [S-Expression](https://en.wikipedia.org/wiki/S-expression). When our program sees one of these with multiple elements, it's going to try to execute it as a function call, looking up the function from the symbol in the first position. First, though, it's going to recursively evaluate all of its children - so if any of them are themselves s-expressions, we'll get them into values we can work with first. In this example, the inner S-Expression `(* 3 4)` will be evaluated first:

```rust
S-Expression(
    Symbol("*"),
    Number(3),
    Number(4),
)
```

This will be interpreted as a function call, and evaluates to:

```rust
S-Expression(
    Symbol("+"),
    Number(2),
    Number(12),
    Number(5),
)
```

Now we have an operation as the first element of the S-expression and some numbers with which to apply it. When this evaluates, we're left with just `Number(19)`, which can be displayed to the user as the result of the computation.

But wait! There's a missing step. That `Symbol("+")` doesn't mean a lot on its own, it's just a string. We need to associate that with the addition function somehow. Thus, we add in the concept of an _environment_, which is just a data structure that associates names with values. These values can be built in functions like `"+"` or user-defined values and functions, they're all stored in the same manner and looked up during evaluation.

## The Issue

Now, this is trivial if there's only one environment. You just make a big `HashMap`. The book I'm translating uses two arrays with matching indices - the point is, it's not a complicated problem. And one big global environment is sufficient to allow variable declaration:

```
lisp> def {x} 100
()
lisp> def {y} 200
()
lisp> def {a b} 5 6
()
lisp> + a b x y
311
```

It gets tricky when we start having _user-defined lambdas_:

```
lisp> def {embiggen} (\ {x y} {^ (* x y) (+ x y)})
()
lisp> embiggen 2 3
7776
lisp> def {real-big} (embiggen 4)
()
lisp> real-big 2
262144
```

There are two distinct things going on here. For one, we now know we need scoped environments because `x` and `y` are only supposed to make sense inside `embiggen`. The lookup of `x` in the environment should fail if we're outside in the global scope, and if we're called inside an `embiggen` call it should find whatever was bound during the call, in the case of `embiggen 2 3` this would be `2`.

This is fine, we can handle this by building the local environment when evaluate this call, adding in these values and using this new temporary environment for this particular evaluation. What if we want to be able to have a value like `real-big`, though? This lambda has `x` in the environment as part of its definition, but after evaluating it it's not stored in either its arguments or its body:

```
lisp>real-big
(\ {y} {^ (* x y) (+ x y)})
```

This is a function of only one argument, but it's gotta be able to look up that `x` we defined too. It's no longer sufficient to just build the environment with what we've got at evaluation, `real-big` needed to retain this information somehow with it when you look it up in the environment.

## The Solution

In C, this solution is a pointer away. Each function value in the AST carries a pointer to an environment, and each environment carries a pointer to a parent. You just de-reference where appropriate and everything's groovy. Rust is not quite so forgiving. You can store that pointer to a parent just fine:

```rust
#[derive(Debug, PartialEq)]
pub struct Lenv<'a> {
    lookup: HashMap<String, Box<Lval>>,
    pub parent: Option<&'a Lenv<'a>>,
}
```

But uh-oh - we've got an explicit lifetime. As it turns out, you can't just casually toss that into something like `Lval`:

```rust
type LvalChildren = Vec<Box<Lval>>;
pub type LBuiltin = fn(&mut Lval) -> BlisprResult;

#[derive(Clone)]
pub enum LvalFun {
    Builtin(String, LBuiltin), // (name, function pointer)
    Lambda(ENVIRONMENT TYPE?!, Box<Lval>, Box<Lval>), // (env, formals, body)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lval {
    Fun(LvalFun),
    Num(i64),
    Sym(String),
    Sexpr(LvalChildren),
    Qexpr(LvalChildren),
}
```

This type is recursive as heck and stuff, and there's mutable references to these babies all over this thousand-line codebase. I went though _hell_ trying to emulate the exact C code. There were `Arc<RwLock<T>>`s, there was an arena-type allocator for a while, there were two explicit lifetimes, it was a mess.

Then, I just...didn't. I filled in that frantic `ENVIRONMENT TYPE?!` with `HashMap<String, Box<Lval>>` - the same as the innards of the environment struct but without that pesky reference and all its lifetime baggage.

Instead of carrying an environment, parent reference and all, it will just carry specifically any partially applied forms. In the `real-big` example, this `HashMap` would have just a single entry: `x: 2`. Then it's up to the calling code to do a little extra work:

```rust
for (k, v) in env {
    local_env.put(k, v);
}
```

In most usage, this won't realistically have more than a handful of values in it. So, yes, it's a slowdown, but it's really not that big a deal of a slowdown? And it solves the problem, this feature works as promised. I didn't necessarily learn anything deep about anything with it - on the contrary I probably avoided getting a better understanding of how to solve this sort of problem in the general case in Rust.

But _the thing does the thing_. For now, at least, just as well as I need it to. Hack or nah?
