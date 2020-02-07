---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--vaxSJFUh--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://thepracticaldev.s3.amazonaws.com/i/nkecbs070xthq0kes77l.jpg
edited: 2019-07-28T12:00:00.000Z
title: Getting Cozy With C++
published: true
description: A reflection on taking my time with Advent of Code problems
tags: beginners, cpp, devjournal
---
# Swallowing The Pill

## Some Background

C++ is my self-learning white whale.  I've tried many times over the years to make it a bit further though one of the big Stroustrup bibles, but inevitably flame out embarrassingly quickly.  It's just *huge*, and kinda complicated.  I was always in awe of it and *wanted* to be able to leverage its power, but as it was easier to get going with other languages I never got around to taking the time.  I never had quite enough self-discipline.

I'm back in school now, so I took advantage of the opportunity and specifically enrolled in the C++-focused track.  I figured a good jolt of structured academic instruction might be just the ticket to force myself to put in the time and energy.  I'm finally doing a significant month-long final project with it, so it's my first real off-the-rails C++ test drive having now learned more about the language than I'd ever managed before.

It turns out I think I even *like* it, but boy is it an awakening coming from Rust and JavaScript and C and company.

## What This Is

This is an *extremely* beginner-level look at some stuff I've learned to un-stick myself while implementing this homework assignment.  I'm making no claim that I've discovered the *best* solutions to these problems, this is more a journal of what's ended up working for me.  Si vez algo, *di* algo.

The closest language analogue I'm comfortable with now is Rust (or maybe C, I'm not sure - they're similar for different reasons), which also happens to be what I've been using the most of lately, so I'm approaching this project more or less as I would a Rust project for better or for worse.  Quickly I could tell the idioms are pretty different, but you've gotta start from somewhere.

For context, the project is a CLI game of Battleship.  The code can be found on [GitHub](https://github.com/deciduously/volley).

## Using statements

My first confusion came from namespace etiquette.  I knew I didn't like `using namespace std`, so I decided to go with scope-level using statements:

```cpp
std::string someFunc()
{
    using std::string;

    string myString = "";
}
```

This keeps it explicit in the global scope but allows me to pull specific things into a function without sacrificing clarity - you can see where it's coming from right there.

Then I got confused about `#include` statements - sometimes a header is included through multiple layers of other includes, because the preprocessor is literally just pasting code into other code resolving these.  It can be tough to see where a specific function is actually included.

I was pointed to [this article](http://www.cplusplus.com/forum/articles/10627/), which is worth a read.  The biggest takeaway for me was that if you just use a pointer to a specific object, you don't need to actually include it, you can (and probably should) just forward-declare it.

## Debugging

Until now, I've mostly been a `println` debugger.  I've known how to use [`gdb`](https://www.gnu.org/software/gdb/) but never saw that as easier than just adding a `debug!()` output somewhere.

I have now come to *very much* appreciate `gdb`.  My programs had simply never gotten large enough.  A quick rundown of literally everything I need:

1. Compile with the `-g` flag.

2. Invoke `gdb my_executable`

```
→ gdb build/volley
GNU gdb (Gentoo 8.3 vanilla) 8.3
Copyright (C) 2019 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
// blah blah blah
Reading symbols from build/volley...
(gdb)
```

You enter commands at the `(gdb)` prompt.

3. Break on the function you want to inspect:

```
(gdb) break runFiring
Breakpoint 1 at 0x40e913: file src/game.cpp, line 38.
```

4. Use commands to navigate through your program:

a. `run`/`r`: run the loaded program for debugging until next breakpoint
b. `next`/`n`: step through execution one line at a time, **not** stepping into functions.
c. `step`/`s`: step through execution one line at a time, stepping into all functions
d. `print`/`p`: print the value of a variable
e. `examine`/`x`: examine memory of a variable
f. `continue`/`c`: stop stepping line by line and resume execution to the next breakpoint (or program completion)
g. `kill`/`k`: kill the program being debugged without exiting `gdb` to take it again from the top (use `quit`/`q` to get back to your shell)

You can add breakpoints any time, and remove them with `delete`.  Use the up and down arrows to access command history, and leaving it empty and hitting `Enter` will just repeat the last command - useful for stepping line by line.

There's lots and lots more, there's a great PDF cheatsheet [here](https://darkdust.net/files/GDB%20Cheat%20Sheet.pdf).  I find myself using `info locals` a lot, which shows you all the variables in the current stack frame.

It is *so much better* than adding and removing println statements and recompiling.  It's much more exploratory and interactive, and a million times more efficient.  I still only just barely know how to use it, too.


## Clean Up After Yourself

There's a super quick way to check if you've done your job memory-leak wise: [valgrind](http://www.valgrind.org/).

This is another tool I *do not* know how to use but already get immense benefit from:

```
± |master U:13 ✗| → valgrind build/volley
==7744== Memcheck, a memory error detector
==7744== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==7744== Using Valgrind-3.15.0 and LibVEX; rerun with -h for copyright info
==7744== Command: build/volley
==7744==

            Battleship!!!

// ... etc - play a game

Game over!
==7744==
==7744== HEAP SUMMARY:
==7744==     in use at exit: 672 bytes in 8 blocks
==7744==   total heap usage: 3,970 allocs, 3,962 frees, 177,854 bytes allocated
==7744==
==7744== LEAK SUMMARY:
==7744==    definitely lost: 0 bytes in 0 blocks
==7744==    indirectly lost: 0 bytes in 0 blocks
==7744==      possibly lost: 0 bytes in 0 blocks
==7744==    still reachable: 672 bytes in 8 blocks
==7744==         suppressed: 0 bytes in 0 blocks
==7744== Rerun with --leak-check=full to see details of leaked memory
==7744==
==7744== For lists of detected and suppressed errors, rerun with: -s
==7744== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
```

Hold up, heap in use at exit?  Ah, of course - I'd written my destructors, but never actually call `delete` on the top-level instance anywhere!  After a quick edit:

```
$ valgrind build/volley
==8122== Memcheck, a memory error detector
==8122== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==8122== Using Valgrind-3.15.0 and LibVEX; rerun with -h for copyright info
==8122== Command: build/volley
==8122==

            Battleship!!!

// ... etc - play a game

Game over!
==8122==
==8122== HEAP SUMMARY:
==8122==     in use at exit: 0 bytes in 0 blocks
==8122==   total heap usage: 3,993 allocs, 3,993 frees, 178,686 bytes allocated
==8122==
==8122== All heap blocks were freed -- no leaks are possible
==8122==
==8122== For lists of detected and suppressed errors, rerun with: -s
==8122== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
```

That's 672 whole bytes now present and accounted for.  Boo-yah.  All I needed was the nudge to go double-check from just running it with no options, there's also a lot more this tool can do for you.

## Struct Equality

Off the bat one of my problems was `std::find()`.  This is used to locate an element in a vector.  Clearly, such a function will be comparing elements for equality.  In Rust, you'd derive or hand-implement the `PartialEq` trait on a struct in order to enable that behavior.  C++ doesn't have that, but you still need to be able to define equality for structs.

Structs are basically equivalent to classes, but their members are public by default.  This is something I knew from textbook-reading, but had never needed to use.

Without providing a definition, you get this somewhat opaque error from `clang`:

```
usr/lib/gcc/x86_64-pc-linux-gnu/9.1.0/include/g++-v9/bits/predefined_ops.h:241:17: error: invalid operands to binary expression ('Cell' and 'const Cell')
        { return *__it == _M_value; }
```

This happens because `std::find()` tried to use `==` on two structs, but we hadn't defined how to do that.  I think the problem is that it was expecting it to be passed by reference, and instead it got passed by value.

You can allow equality checks to work on structs you define by overloading the `==` operator and specifically passing a `const` reference:

```cpp
// A single cell on the board
typedef struct Cell
{
    int row;
    char col;
    bool operator==(const Cell &other) const
    {
        return row == other.col && col == other.col;
    }
} Cell;
```

This looks a lot like a handwritten `impl PartialEq block` (from [the docs](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)), which also uses what's essentially the Rust-y `const &`:

```rust
struct Book {
    isbn: i32,
    format: BookFormat,
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.isbn == other.isbn
    }
}

```

## Constantly Const

This leads into the next point - sprinkle `const` *everywhere*.  This is something Rust has actually prepared me well for.  I essentially use it like the opposite of `mut`.  Here's one of my class headers:

```cpp
class Board
{
    int dimension;
    std::vector<Cell> receivedShots;
    std::vector<Ship> ships;

public:
    Board(int boardSize = BOARD_SIZE);
    bool doesFit(ShipPlacement sp) const;
    char getCharAt(Cell c, bool showShips) const;
    Cell getRandomCell() const;
    Cell promptCell(const std::string &promptStr) const;
    void pushShip(Ship s);
    std::vector<Cell> getAllShots() const;
    bool receiveFire(Cell target);
    int size() const;
    lines toLineStrings(bool showShips) const;
};
```

This thing is so full of `const` it's ridiculous.  When I started coding I didn't realize quite how much it would be applicable and was hesitant to use it for fear of not understanding it.  Now my rule of thumb is to add it by default to any method, and only take it away if I'm sure I cannot have it.

## My God, It's Full Of Streams

C++ leans hard into the stream abstraction.  I ran into this relatively quickly when I wanted to pretty-print some data.  In Rust I'd reach for `impl Display`, in something more OOP i'd override `toString()` or something.

In C++, you actually overload the `<<` stream insertion operator.  For a simple example:

```cpp
enum Direction
{
    Left,
    Down
};

std::ostream &operator<<(std::ostream &stream, const Direction &d)
{
    if (d == Direction::Left)
        return stream << "Left";
    else
        return stream << "Down";
}
```

Now you can pop it right in a stream, no need to call anything:

```cpp
std::cout << "Direction: " << direction << "!" << std::endl;
```

This pattern was not obvious to me at first but feels a lot more natural after a few days.

## Overloaded Constructors

I've never worked with a language that does this, so it's still novel and neat to me.  In Rust, you use traits and it's a little more unwieldy.  In C++ I can just literally define three constructors:

```cpp
class ShipClass
{
  // ..
public:
    ShipClass();
    ShipClass(char c);
    ShipClass(ShipClassType sc);
 // ..
}
```

That's a pretty easy way to get yourself a flexible API.

## Frustrations

Not everything has been happy.  I've generally expected everything C++ has had to throw at me given prior knowledge, but there are a few outstanding things I'm still not sure how to learn to like.

### Build tooling / Package Management

C++ is, to me, the wild friggin' west.

I haven't gotten started with things like [CMake](https://cmake.org/) and [Autotools](https://www.gnu.org/software/automake/manual/html_node/Autotools-Introduction.html), but the very fact these tools exist says a lot.  It's really hard to just use external libraries, so often projects will simply just not.  There's a lot of reinventing the wheel because package management is such a complete mess.  That's not a healthy ecosystem  to the untrained eye, but the language itself is powerful enough that maybe it makes up for it.  It also is a gigantic ecosystem despite this shortcoming, so I want to be able to explore and use it, but if it's so complicated I won't bother.

Then there's things like [`boost`](https://www.boost.org/) which are just their own beasts in and of themselves.  I think it will quite literally be years until I'm able to make a reasonable and informed statement about the power and quality of the C++ ecosystem.  Until then, it's a newbie turn-off.

I've already written [a post](https://dev.to/deciduously/how-to-make-a-makefile-1dep) about `make`, which I won't recreate here.  The second example I walk through in that post is the exact Makefile I'm using to build this project.

This is the very first C++ course in the curriculum, and I imagine it will be covered later on, but for this one the professor basically said "I do not care how you build your code, just make sure I can recreate it, if you don't know what to do, here's a link to download Visual Studio".

I should probably learn Visual Studio sometime, but I think it's easier to learn one thing at a time, so I just stuck with my usual text editor and compilation via CLI.  I already knew how to use make from years of tinkering with Linux.  I don't know what the best way to go about this is.  It seems like in a professional setting CI/CD would run all the compilers anyway.

### Exceptions

This wasn't difficult to pick up, being not dissimilar from exceptions in JS or Python:

```cpp
try
{
    row = stoi(originStr.substr(1, originStr.size() - 1));
}
catch (const std::invalid_argument &ia)
{
    std::cerr << "Please enter a number as your second term." << endl;
    // ..
}
```

I guess I'm just spoiled with type-level stuff.  I don't like this, it seems like it seems to spaghetti-type code and lots of needless verbosity to catch errors for a whole app.  I also haven't really started mucking about with templates much, so it's likely this is a familiarity issue.  It does seem to work more or less the same as I expect from Python or JS.

### Class vs Struct

This isn't a problem, but still doesn't really make sense to me.  In Rust, everything is a `struct`, and you can provide an `impl Struct` block to define a constructor and/or methods if needed, or just not for plain data.

C++ has structs and classes, but they're almost identical.  The only difference is the default visibility: structs are public, classes are private, but through explicit annotations they're functionally equivalent otherwise.  I try to use structs for plain data and classes for anything else, but the line is blurry.  If I have a struct that just wraps an enum but has a bunch of different getters, like a `char` and a `string` and an `int`, is that a class or a struct?  Right now I have a struct which just holds a row and a column, also has some constructors and an equality method defined.  That's not that different from a class.  I don't know which, if either, is correct, or if it matters at all.  I'm just kind making a gut decision when I define a new one and not thinking about it again, it doesn't seem to make a difference.

Follow-up: is that the kind of thing you'd use a `union` for?  I still don't quite know when I'd want one of those unless I'm specifically space constrained.

## Conclusion

I'm glad I'm finally ripping the band-aid and using C++ for something a little more substantial, but never before has the vastness of the mountain been so apparent at the outset.

Photo by Matthew Henry on Unsplash
