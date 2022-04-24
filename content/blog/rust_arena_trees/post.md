---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s---SGQ7aVH--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://thepracticaldev.s3.amazonaws.com/i/0qfm7joaxu4bvp13w618.jpg
date: 2019-12-19T12:00:00.000Z
title: "No More Tears, No More Knots: Arena-Allocated Trees in Rust"
description: An overview of region-based memory management for constructing trees and graphs in Rust
tags:
  - rust
  - beginners
  - tutorial
  - devjournal
---

## Enter The Arena

When programming in Rust, it's not always straightforward to directly translate idioms you know. One such category is tree-like data structures. These are traditionally built out of `Node` structs that refer to other `Node`s in the tree. To traverse through your structure, you'd use these references, and changing the structure of the tree means changing which nodes are referred to inside each one.

Rust _hates_ that. Quickly you'll start running into problems - for instance, when iterating through nodes, you'll generally need to borrow the structure. After doing so, you'll have a bad time doing anything else with the structure inside.

Trees are a fact of life, though, and very much a useful one at that. It doesn't have to hurt! We can use [region-based memory management](https://en.wikipedia.org/wiki/Region-based_memory_management) to pretty much forget about it.

### The Desert

I'll briefly mention a few of the other methods I've bashed my head against before trying this today.

The simplest is to use `unsafe`, which allows you to use raw pointers like you would in C. This forfeits a lot of the benefits we get from using safe Rust, as one use of `unsafe` will infect your whole crate. Now part of the code is only deemed safe because you, the programmer, have deemed it to be, and not the Rust compiler.

To stick to statically safe Rust, you can wrap your pointers in `Rc<RefCell<T>>` types, which are reference-counted smart pointers with interior mutability. When you call `Rc::clone(&ptr)`, you get back a brand new pointer to the same data, that can be owned separately from any existing pointer, and when all such `Rc`s have been dropped the data itself will get dropped. This is a form of static garbage collection. The `RefCell` that allows you to take mutable borrows of things that aren't mutable, and enforces at runtime instead of statically. This lets you cheat, and will `panic!()` if screw up, so, hooray I guess. You need to use methods like `data.borrow_mut()` but then can, for example, change the pointer in a `next` field using an otherwise immutable borrow of the node during your traversal.

Alternatively you can use `Box` smart pointers and clone them around, performing a lot of extra work for no reason - this involves deep-copying whole subtrees to make small edits. You do you, but that's not really my thing.

You can even use plain references and introduce explicit lifetimes:

```rust
struct Node<'a> {
    val: i32,
    next: &'a Node,
}
```

Yippee, you're probably sprinkling `'a` all over the place now, and there's gonna be a part of you that wants to start getting friendly with `b`, and whoa there. That's gross, and you're solving a much simpler problem that requires.

All of these options mean pain, and often compromise. At least in my experience, while you often can get to a successful compile your code gets unreadable and unmaintainable fast, and should you ever need to make a different choice you're pretty much back to square one trying to fit it all together. It's also the only way I've ever managed to actually produce a segfault in Rust. I was pretty impressed with myself for screwing up that hard and I wish I had kept better notes about how I got there, but I know it was some nonsense like the above.

The problem is that Rust is keeping a close eye on who owns your nodes and what lifetime each has, but as you build a structure it's not always easy for the compiler to understand what it is you're trying to do. You end up with inferred lifetimes that are too small or not accurate for your structure and no way to efficiently traverse or edit the map. You end up needing to do manual work to convince the compiler you're right, which sucks.

### The Oasis

What if your nodes could all have the SAME lifetime? I mean, they essentially do, right? Sure, some may get created after one another, but for all intents and purposes within this program you just care that they're all owned by your top-level tree structure.

There's a super easy way - pop 'em in a `Vec<T>`:

```rust
#[derive(Debug, Default)]
struct ArenaTree<T>
where
    T: PartialEq
{
    arena: Vec<Node<T>>,
}
```

Boom. Tree. It's generic for any type that can be compared with `==`, and the lifetime problems are solved. You want a node? Use `self.arena[idx]`. Instead of storing actual references to other nodes, just give 'em each an index:

```rust
#[derive(Debug)]
struct Node<T>
where
    T: PartialEq
{
    idx: usize,
    val: T,
    parent: Option<usize>,
    children: Vec<usize>,
}
```

In this tree, each node has zero or one parents and zero or more children.
New ones will require an ID specified, as well as a value to store, and will not connect to any other nodes:

```rust
impl<T> Node<T>
where
    T: PartialEq
{
    fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: vec![],
        }
    }
}

```

You could go on and store as many indices as you want - it's your graph. This is just the example tree I used for [Day 6 of AoC](https://adventofcode.com/2019/day/6) (and why we're here).

This is pretty easy to use. When you want a value, you can just ask for its index:

```rust
impl<T> ArenaTree<T>
where
    T: PartialEq
{
    fn node(&mut self, val: T) -> usize {
        //first see if it exists
        for node in &self.arena {
            if node.val == val {
                return node.idx;
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, name));
        idx
    }
}
```

Whether or not it was there previously, you now have an index for that value in your tree. If it wasn't already there, a new node was allocated with no connections to any existing nodes. It will automatically drop when the `ArenaTree` goes out of scope, so all your nodes will always live as long as any other and all will clean up at the same time.

This snippet shows how easy traversal becomes - you just walk the vector with, e.g., `for node in &self.arena`. Certain operations become trivial - want the number of nodes? Ask for it:

```rust
fn size(&self) -> usize {
    self.arena.len()
}
```

What about counting how many edges are there? Nothing fancy here either, count them:

```rust
fn edges(&self) -> usize {
    self.arena.iter().fold(0, |acc, node| acc + node.children.len())
}
```

It's still pretty easy to do your standard recursive data structure stuff, though. You can see how deep a node is:

```rust
fn depth(&self, idx: usize) -> usize {
    match self.arena[idx].parent {
        Some(id) => 1 + self.depth(id),
        None => 0,
    }
}
```

Search for a value from the root, returning its depth:

```rust
fn depth_to_target(&self, idx: usize, target: &T) -> Option<usize> {
    // are we here?  If so, Some(0)
    if target == &self.arena[idx].val {
        return Some(0);
    }

    // If not, try all children
    for p in &self.arena[idx].children {
        if let Some(x) = self.depth_to_target(*p, &target) {
            return Some(1 + x);
        }
    }
    // If it cant be found, return None
    None
}
```

You can of course traverse iteratively as well. This method finds the distance between the parents of two nodes using both iterative and recursive traversal to perform a series of depth-first searches:

```rust
fn distance_between(&mut self, from: T, target: T) -> usize {
    // If it's not in the tree, this will add a new unconnected node
    // the final function will still return None
    let start_node = self.node(from);
    let mut ret = 0;
    // Start traversal
    let mut trav = &self.arena[start_node];
    // Explore all children, then hop up one
    while let Some(inner) = trav.parent {
        if let Some(x) =  self.depth_to_target(inner, &target) {
            ret += x;
            break;
        }
        trav = &self.arena[inner];
        ret += 1;
    }
    // don't go all the way to target, just orbit
    ret - 1
}
```

This repeats a little work on each backtrack, but at even puzzle scale computes nearly instantly. It's quite concise and readable, not words I'm used to using for Rust trees!

Inserting will depend on the domain, but this application received input as `PARENT)CHILD`, so my `insert` looked like this:

```rust
fn insert(&mut self, orbit: &str) {
    // Init nodes
    let split = orbit.split(')').collect::<Vec<&str>>();
    let inner = self.node(split[0]);
    let outer = self.node(split[1]);
    // set orbit
    match self.object_arena[outer].parent {
        Some(_) => panic!("Attempt to overwrite existing orbit"),
        None => self.object_arena[outer].parent = Some(inner),
    }
    // set parents
    self.object_arena[inner].children.push(outer);
}
```

To recap, whenever you want to manipulate a given node you just need its index to do so. These are handily `Copy`, so don't worry too much about manipulating them. To get a node's index, call `tree.node(val)`. It will always succeed by performing a lookup first and then allocating it to your tree's arena if it wasn't already there. Then it's up to you to manipulate the node's fields to the indices where it belongs: `self.arena[idx].children.push(outer);`. You never need to worry about the memory again, your `Vec` will drop itself when it can. You define the structure of the tree yourself by what indices are stored in each node and what happens when you insert a new one.

Basically, it's a tree like you want it to be, but it's in Rust and you don't even have to fight about it, and it's great.

Here's a [playground link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=7cccabd269fd1ee8f61ff23fd79117e7) to poke and prod at.

_cover image by Amanda Flavell on Unsplash_
