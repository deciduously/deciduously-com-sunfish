---
cover_image: https://res.cloudinahttps://thepracticaldev.s3.amazonaws.com/i/lk886f5xd4t64pa2cw9i.jpg
edited: 2019-07-24T12:00:00.000Z
title: How To Make A Makefile
published: true
description: A reflection on taking my time with Advent of Code problems
tags: beginners, gnu, tools, cpp
---
# What's Make?

This post will explore the basics of [GNU Make](https://www.gnu.org/software/make/) via two small examples.  It's a surprisingly versatile build tool, if a bit archaic, and as it's so ubiquitous it's worth getting at least baseline familiar with how it does its thing.

Caveat: as far as I know this is mostly relevant for Mac and Linux users.  I don't know much about build tooling or development on Windows outside of just booting up an IDE and letting it handle things, or using WSL as a crutch where available.  I do know you can get `make` via [GnuWin32](http://gnuwin32.sourceforge.net/packages/make.htm).  I have no idea how well it works or if anyone uses it.

In brief, `make` is a tool that reads a *Makefile* and turns source files into executable files.  It doesn't care what compilers are used to do so, it's just concerned with build orchestration.

If you've compiled packages from source before, you may be familiar with the following set of commands:

```
$ ./configure
$ make
$ sudo make install
```

A great number of *nix packages are distributed as C or C++ source code, and will be built something like this.  The first line runs a separate program to configure your Makefile for you, which is necessary in big projects which rely on system libraries.  The last line generally assumes admin rights so it can copy the executable(s) it just built onto the system path.  We don't need any of that to get started with `make`, though.  Just the middle line will do us fine.  Aptly named, isn't it?

In this post I'll walk through two different examples with different goals.  The syntax can look opaque (at last, it did to me) if you don't know what you're looking at, but once you know the very basic rules they're very straightforward.

# Example One - Download A File

We'll do the simpler one first.  This Makefile only exists to download `boot`, a build tool for Clojure, to the user's current directory.  This tool exists as a shim that downloads a jarfile to handle the rest, and the shim is very tiny, so it's sometimes convenient to have it live in a project directory itself instead of the system path.

```make
.PHONY: deps help

SHELL        = /bin/bash
export PATH := bin:$(PATH)

deps: bin/boot

bin/boot:
	(mkdir -p bin/                                                                              && \
	curl -fsSLo bin/boot https://github.com/boot-clj/boot-bin/releases/download/latest/boot.sh  && \
	chmod 755 bin/boot)

help:
	@echo "Usage: make {deps|help}" 1>&2 && false
```

We'll take it from the top.

```make
.PHONY deps help

SHELL       = /bin/bash
```

First, we declare the *phony targets*.  To explain this, we need to talk about the core of what `make` is: rules.

Make is for making sources into targets.  To do so, we give it rules for understanding what sources and how to feed them to compilers to get the right targets.  At the end, we should have produced all the targets needed - the compiled sources.

Keeping that in mind, rules are easy to grok.  Each rule starts with the name of the target to be created, followed by a colon.  After the colon are any targets *this* target depend on, and below and indented are a series of commands, or recipes, to build the target from its dependencies.  When you invoke `make` with a target, it will make that target specifically, but when you invoke it on its own it just start evaluating the first rule it sees that doesn't begin with a `.` (like `.PHONY`).

Next, we define the shell executable location,

The `$()` syntax is a Make variable.  Make is neat in that it automatically exposes every variable it finds in the environment as a make variable, so we can just use `$PATH` from `bash` with `$(PATH)`.  To define your own you just assign to the name, omitting the parens, as we do in the first line - that's an assignment to the `$(SHELL)` variable.

Notably, we're using the `:=` assignment syntax for it.  This specifically defines a *simply-expanded* assignment.  This variable will be read once and that's it - any other variables inside it are expanded once immediately at assignment.

The `=` *recursively-expanded* variable instead expands anything inside whenever it's substituted.  This is powerful, but also can lead to problems like infinite loops and slow execution so it's important to be mindful of the difference.

It's important to note that this is only true for this process and any sub-process of it - this isn't permanent, it cannot alter the parent process.  Still useful if you're building inside `make`, though, and doesn't clutter up your global env!

Then we get to our first rule.  In this case, the default rule is called `deps`, one of our phony targets.  No file called "deps" will be created.

```make
deps: bin/boot
```

After the target name, you'll find a colon and then a list of *dependencies*.  These are targets that must be completed before evaluating this rule.  Before executing the block of commands for this target, Make will ensure each of the targets exists, evaluating their rules if it finds them.  In this case, the dependency is target "bin/boot".  There are no commands associated with this rule, all it does is call this other rule.

```make
bin/boot:
	(mkdir -p bin/                                                                              && \
	curl -fsSLo bin/boot https://github.com/boot-clj/boot-bin/releases/download/latest/boot.sh  && \
	chmod 755 bin/boot)

```

This isn't a phony target, and includes a slash, which just means a directory name.  This target, or the result of evaluating this rule, is going to end up in that directory we added to the PATH.

This rule doesn't have any dependencies - they'd all appear on the same line as the target name.  It does have commands though - this rule will create a directory, execute `curl` to downloade the file from GitHub, and execute `chmod` to make the downloaded file executable.

So, running `make` will locate the `make deps` rule, which is empty itself but has `bin/boot` as a dependency.  Make will realize `bin/boot` does not yet exist and execute that rule, which will create the file accordingly.

Try running it, and then running it again:

```
$ make
(mkdir -p bin/                                                                              && \
curl -fsSLo bin/boot https://github.com/boot-clj/boot-bin/releases/download/latest/boot.sh  && \
chmod 755 bin/boot)

$ make
make: Nothing to be done for 'deps'.
```

After evaluating this rule the first time around, a file called `boot` already existed in a directory called `./bin`.  The target was found, so `make` did no extra work.  This handy quality is known as *idempotence*.  Repeated invocations have the same effect as one invocation: `f(x);` and `f(x); f(x);` are equivalent.

Neat!  Let's look at something a little more typical.

## Example Two: Build Some C++

This is more complicated one.  This makefile is what I drop in to a brand new C++ project directory before thinking about it.  It's more indicative of what makefiles in the wild might look like, but still really small in scope.

It expects a `src` directory with a bunch of `.cpp` (and `.h`) files, and will create a directory called `build` with all your `.o` object files and your executable, named whatever you tell it.  You can then run that executable.

```make
.PHONY: all clean help

CXX=clang++ -std=c++11
FLAGS=-Wall -Wextra -Werror -pedantic -c -g

BUILDDIR=build
SOURCEDIR=src
EXEC=YOUR_EXECUTABLE_NAME_HERE
SOURCES:=$(wildcard $(SOURCEDIR)/*.cpp)
OBJ:=$(patsubst $(SOURCEDIR)/%.cpp,$(BUILDDIR)/%.o,$(SOURCES))

all: dir $(BUILDDIR)/$(EXEC)

dir:
	mkdir -p $(BUILDDIR)

$(BUILDDIR)/$(EXEC): $(OBJ)
		$(CXX) $^ -o $@

$(OBJ): $(BUILDDIR)/%.o : $(SOURCEDIR)/%.cpp
		$(CXX) $(FLAGS) $< -o $@

clean:
		rm -rf $(BUILDDIR)/*.o $(BUILDDIR)/$(EXEC)

help:
		@echo "Usage: make {all|clean|help}" 1>&2 && false
```

At the very top we have our phony targets again - these are the targets that aren't creating real files, they're just intended to be invoked as an argument to make.

Next we point it towards our C++ compiler by assigning the variables `$(CXX)` and `$(FLAGS)`:

```make
CXX=clang++ -std=c++11
FLAGS=-Wall -Wextra -Werror -pedantic -c -g
```

These aren't special names - you can call them whatever you like.  We'll refer to them directly in our rules.

C++ compilation happens in two stages.  First, we compile all the separate `*.cpp/*.h` pairs into their own `.o` object files, and in a separate step we'll link them all up into a single executable.  The flags we pass to the compiler are only relevant when building the objects from source - linking together already-compiled objects doesn't need them!  This way we can invoke the compiler with or without this set of flags inside our rule evaluation.  I like to make my compiler as restrictive as possible - these flags turn all warnings into errors that prevent successful compilation, and enable to full suite of checks available.   The `-c` flag instructs it not to go on to the linking phase, finishing with an `.o` file, and the `-g` flag generates source-level debug info.

A fancier makefile will have multiple build configurations.  This, again, is a starter kit.

The next three assignments just configure the names of everything:

```make
BUILDDIR=build
SOURCEDIR=src
EXEC=YOUR_EXECUTABLE_NAME_HERE
```

I think `build` for the output and `src` for the source files make sense, but you can adjust them there, and `$(EXEC)` will be the final compiled binary.

Below that we define where the sources are, and what the objects should   be called:

```make
SOURCES:=$(wildcard $(SOURCEDIR)/*.cpp)
OBJ:=$(patsubst $(SOURCEDIR)/%.cpp,$(BUILDDIR)/%.o,$(SOURCES))
```

The `$(SOURCES)` variable is built with the [`wildcard`](https://www.gnu.org/software/make/manual/html_node/Wildcard-Function.html#Wildcard-Function) function.  This variable collects anything with the `.cpp` extension inside `src/`.

Next we use [`patsubst`](https://www.gnu.org/software/make/manual/html_node/Text-Functions.html#Text-Functions).  The syntax for this is *pattern*, *replacement*, *text*.  The `%` character in the pattern and replacement is the same, and the other part is swapped.  This substitution turns, e.g. "game.cpp" into "game.o".  For the text, we're passing in the `$(SOURCES)` variable we just defined - so the `$(OBJ)` variable will contain a corresponding `build/*.o` filename for each `src/*.cpp` filename that `make` finds.

Check out the [quick reference](https://www.gnu.org/software/make/manual/html_node/Quick-Reference.html) for a complete run-down of what's available.

I've used simply-expanded variable assignment for these.  It's a good idea to do so when you know that will get you the result you need specially when using functions like `wildcard` - recursively expanding these can (but doesn't always) result in significant slowdowns.

With all our variables configured, we can start defining rules.  The first rule is our default behavior, this one is called `all`:

```make
all: dir $(BUILDDIR)/$(EXEC)
```

This is one of our phony targets, so there's no corresponding output file called "all".  Also, like `deps` from the first example, this rule has no commands, only dependencies.  This one has two dependencies, `dir` and `$(BUILDDIR)/$(EXEC)`.  It will execute them in the order they are found, so lets hop over to `dir` first:

```make
dir:
	mkdir -p $(BUILDDIR)
```

This one doesn't have dependencies, so it will immediately execute this command.  This is a simple one - it just makes sure the `build` directory exists.  Once that's complete, we can evaluate `$(BUILDDIR)/$(EXEC)`:

```make
$(BUILDDIR)/$(EXEC): $(OBJ)
		$(CXX) $^ -o $@
```

This rule is starting to look a little funkier.  The target itself is not unlike `bin/boot` from the first example, just using make variables to build it.  If you've set `$(EXEC)` to `my_cool_program`, this target is named `build/my_cool_program`.  It depends on another make variable, `$(OBJ)`, which we just defined as an object file corresponding to each source file.  That will resolve first, so let's look at that rule before looking at the command:

```make
$(OBJ): $(BUILDDIR)/%.o : $(SOURCEDIR)/%.cpp
		$(CXX) $(FLAGS) $< -o $@
```

Whoa, there's *two* sets of dependencies here!  What the heck, Ben.

This is something called a *static pattern rule*.  This is what we use when we have a list of targets.  The overall target, `$(OBJ)`, consists of each one of the object files we'll be creating.  After the first colon, we need to define specifically how each individual object depends on a specific source.  Again we see the `%` used for pattern matching, not unlike up in the `patsubst` call.  Each one will have the same name as the corresponding ".cpp" file, but with the extension flipped to ".o".

The command block for this rule will execute for each source/target pair matched.  We're using the make variables we defined way up at the top to invoke the compiler and pass in all our flags, which includes the `-c` flag signalling to stop before the link phase, just outputting object files.

Then we use some automatic variables to fill in the proper command.  `$<` corresponds to the name of the dependency we're working with, and `$@` corresonds to the name of the target.  Full expanded, this `$(CXX) $(FLAGS) $< -o $@` command will look like `clang++ -std=c++11 -Wall -Wextra -Werror -pedantic -c -g src/someClass.cpp -o build/someClass.o`.

Marvelous!  Once this rule completes, every ".cpp" file has a corresponding ".o" file in the `build/` directory, exactly what we defined as `$(OBJ)`.  With that in place `make` will jump back up to the calling rule and finish off with the `$(CXX) $^ -o $@` command to link our objects together.

This is similar, but we're omitting our flags. We also use a different automatic variable.  `$^` corresponds to the entire list that `$(OBJ)` represents.  You could also use `$+`, which fully includes each list member - `$^` omits any duplicates.  The `$@` part is the same as previously - it stands for the target.  This might run a command something like `clang++ --std=c++11 build/someClassOne.o build/someClassTwo.o build/someClassThree.o build/main.o -o build/my_cool_project`.

Once that's done, you've got your compiled executable ready to go at `build/my_cool_project`.  Thanks, `make`!

This makefile also provides `clean`:

```make
clean:
		rm -rf $(BUILDDIR)/*.o $(BUILDDIR)/$(EXEC)
```

This is another phony target with no dependencies that just runs `rm` to clean out all the object files and the executable.  This way when you run `make` again it will have to build everything again.  Otherwise it will just build any files that have changed since your project was last built.

We've only scratched the surface, but hopefully this helps demystify these files a bit should you come across one.

Challenge: write your own `make install` rule that copies the newly created target out of `build` to a cooler place!

Photo by Jason Briscoe on Unsplash
