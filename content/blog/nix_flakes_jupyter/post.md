---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--V5i6n7SZ--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://dev-to-uploads.s3.amazonaws.com/uploads/articles/jtkguing4h1wsk4xh7ms.jpg
date: 2021-10-30T12:00:00.000Z
title: "Workstation Management With Nix Flakes: Jupyter Notebook Example"
tags:
  - tutorial
  - jupyter
  - beginners
  - nix
---
## What?

If you're reading DEV, your computer is probably your primary productivity tool.  Whether it's a job, a hobby, or just a point of interest, your ability to install and use different types of software with ease is integral to your ability to keep learning and working.

This means that many of us end up with *complicated* workstation needs, with an interconnected web of dependency interactions.  If you need to wipe your hard drive and start over, it would likely take a non-trivial amount of work to set everything back up *exactly* the same way, and each piece needs to be managed separately.

[Nix](https://nixos.org/) is a tool designed to take the pain out of workspace management.  Nix is [declarative](https://en.wikipedia.org/wiki/Declarative_programming).  You're not telling the computer how to build your environment step-by-step; you're simply describing the end state you need.  Nix learns how to make it and then stores everything you need in a reproducible way so that subsequent invocations are near-instant.  It is also unified.  Every tool you use can be managed this way, with lots of built-in functionality for quickly setting up complex environments with minimal effort on your part.

This post will show you how I'm using Nix to manage my [Python](https://www.python.org/) environment and dependencies for using [Jupyter Notebooks](https://jupyter.org/) for school assignments.  However, this same strategy can be used to build environments for whatever tool you need.

If you just want to see the config file, jump down to [The Flake](#the-flake).

## Who?

While Nix can control your entire operating system, you don't need to go all-in to reap the benefits.  You can install the Nix package manager on Linux (i686, x86_64, or aarch64) or macOS (currently just x86_64, but ARM support will show up eventually).  See the [Quick Start](https://nixos.org/manual/nix/stable/#chap-quick-start) guide to get up and running.

This post uses a new feature of Nix called Nix Flakes, which you need to enable after you install.  See the [Flakes wiki page](https://nixos.wiki/wiki/Flakes) and look for the "Non-NixOS" section.  I promise it's not too hard.

However, if you end up liking what you see, I think NixOS is the best way to use Nix.  Just recently, I ended up wiping my hard drive.  I re-partitioned and then ran *one single command*.  After letting it build, I could boot into an operating system identical to how I had left it, with all my tools installed and configuration settings set.  That's pretty hard to beat.  You can see [my personal config on Github](https://github.com/deciduously/nixos-config).

## Why?

Python is a great programming language.  It has both a low barrier of entry for people learning the craft and a bustling ecosystem for scientific/numerical computing and web development (and much, much more).

However, one sticking point for me has always been dependency management.  For most Python uses, you will need to install some packages, and keeping this organized is not always straightforward.

I'm using this example specifically because I recently started taking a class that uses Jupyter Notebooks for assignments and labs.  I love Jupyter. It's an excellent tool for this sort of thing and is pretty widely used.  However, when I opened up my first assignment, the first interactive cell contained these lines:

```python
import numpy as np
import pandas as pd
```

While this is super standard and to be expected, I still felt myself thinking, "Oh boy, here we go again."

There is a tool for installing dependencies. It's not like you're just manually downloading Python files, putting them in a sensible place yourself, and somehow telling your Python interpreter where to find them (looking at you, C and C++).  We have `pip`!  With a properly configured Python installation, you can just type `pip install numpy`, and the tool will automatically take care of the details.

However, this installs to a *global* location.  Every Python program in any project on your computer will refer to the same place.  No good - what if you want to run some code that depends on a different version of some package?

The `numpy` dependency isn't a global dependency of your python installation. The need is local to whatever you're currently doing.  There's an answer for that as well, of course.  It involves, well, a `virtualenv`, a requirements file, and a substantial local folder with a full copy of your environment.  This environment is pretty specific to exactly how it was installed too.  If anything happens to it, or you change computers, you have to start over.

It, you know, works, but it doesn't exactly "spark joy" for me.  With each new Python project, the tangled mess that comprises "my workstation" grows, and I don't need that kind of background anxiety in my life.

## How?

With Nix, we have a *declarative* solution.  We can create one text file and describe what we need, and Nix will get it done.  This file will work the same way anywhere we want to use it and doesn't leave brittle artifacts littered in our project codebase.  As long as this single text file is present in your project, you can produce the environment you need the same exact way every time and pretty much just forget about the whole thing.  It will even work if we drop it into a different computer running on a different platform and OS, as long as it's supported by Nix.

### The Flake

I'll start by showing you the whole file, and we'll dig through it in pieces below.  This gets saved as `flake.nix` in the top level of your project directory.

```nix
{
  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-unstable";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
  };
  outputs = { nixpkgs, flake-utils, ... }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
      };
    in rec {
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          (python3.withPackages(ps: with ps; [
            ipython
            jupyter
            numpy
            pandas
          ]))
        ];
        shellHook = "jupyter notebook";
      };
    }
  );
}
```

Not so bad, right?  Let's take a closer look from the outside in.

### The Walkthrough

A Nix Flake is just an object - check out those surrounding curly braces.  This object has two keys, `inputs` and `outputs`.  The inputs are where we define the flake's dependencies and where to find all the tools we use.  This one has two, [`nixpkgs`](https://github.com/NixOS/nixpkgs) and [`flake-utils`](https://github.com/numtide/flake-utils).  Each of these just points to a GitHub URL, and if you follow those links, you'll see each repo provides its own `flake.nix`.  The `outputs` of each remote flake get piped into the `inputs` of my flake, so we can use what they provide.

Next, we define our own output object:

```nix
outputs = { nixpkgs, flake-utils, ... }: flake-utils.lib.eachDefaultSystem (system:
  # ...
);
```

We specifically bring each name we need from the inputs with `{{ nixpkgs, flake-utils, ... }}` so we can refer to them inside, then use a tool from `flakeUtils` called `eachDefaultSystem`.  This is a convenience helper.  It provides a special variable called `system` (see, right at the end: `(system:`) which refers to the specific platform you're on.  On my computer, this resolves to `x86_64-linux`.  This means we can use the exact same flake on any supported system, and Nix will know how to find the correct versions of anything inside.

First, we create a special `pkgs` variable to refer to the Nix package repository:

```nix
let
  pkgs = import nixpkgs {
    inherit system;
  };
in rec {
  #...
};
```

If you're a functional programmer, you might recognize the `let...in` pattern.  We're just defining variables to be used below.  This flake just needs the one, and using `inherit system` - the `system` from just above - we can expressly point to the subset of `nixpkgs` that applies to our platform.  We add `rec` to allow Nix to fully resolve any Nix variables in the following object recursively until thoroughly evaluated.

To discover what's available, you can use the [NixOS Search](https://search.nixos.org/packages) page - yes, this applies to Nix users on other operating systems as well.

Finally, we're ready to tell Nix what we need.  This flake doesn't build a package; it just defines a development shell.  We can activate it by using the command `nix develop` from anywhere in this folder.  This is called a `devShell`:

```nix
devShell = pkgs.mkShell {
  #...
};
```

To build it, we can use the built-in [`mkShell`](https://nixos.wiki/wiki/Development_environment_with_nix-shell) feature.  This is contained in `pkgs`, which we've already appropriately configured above.

We want this shell to have stuff available for use, using the `buildInputs` key of `mkShell`:

```nix
buildInputs = with pkgs; [
          (python3.withPackages(ps: with ps; [
            ipython
            jupyter
            numpy
            pandas
          ]))
        ];
```

We use `with pkgs;` first, so that we don't have to fully qualify `pkgs.python3` for each package inside, but that's what we mean for each item in this array.

This array only contains one input, `python3`.  However, we don't want just a plain Python environment. We know some packages we'll need access to already.  This is the part that replaces the `virtualenv` setup entirely.  We can use the `withPackages` option to define the list: this environment will grab `ipython`, `jupyter`, `numpy`, and `pandas` for us.  This will install Jupyter for us and the packages used inside, so the import statements from above will just work without any further action on your part.  If you need a new package at any point, you can just add it to this list and re-run `nix develop`.

Additionally, you can pin to a minor version of Python by simply replacing `python3` in the snippet with, for example, `python39`.

Finally, to make this as easy as possible to use for this purpose, we can have it immediately launch Jupyter:

```nix
shellHook = "jupyter notebook";
```

You can run any arbitrary code here. This will execute immediately once the environment is ready.  You can use multiple lines:

```nix
shellHook = ''
  mkdir cool_dir
  echo "cool file contents" > cool_dir/cool_file.txt
  jupyter notebook
'';
```

I don't go anywhere without my `cool_file.txt`, and Nix definitely can deliver.

With my particular `shellHook` in place, running `nix develop` in this folder will automatically launch the Jupyter server and open the landing page in my browser.  I just store all my `.ipynb` files in the same directory, and they're all ready to use.  I am *thrilled* that I could use Nix to escape all the Python package management problems I've experienced in the past with under 30 lines of config.  This solution is portable, reproducible, and once you understand the building blocks, easy to use and adapt.

The first time you run `nix develop` will be the slowest because Nix needs to download everything from scratch.  However, once it's in your nix store, subsequent runs will be rapid.  If nothing has changed, it will be instant, and if you tweak something, it will only recalculate the difference.

The inputs are hashed, so you always use the same revision upstream.  You can run `nix flake update` to grab updated hashes if you want to pull in new changes.  This will update the `flake.lock` file that Nix creates and manages.

Your Nix store will grow over time because it will keep fully specified and hashed versions of whatever you use.  To clean up unneeded files, you can periodically run `nix-collect-garbage`.

## Okay, And?

This example is just a taste.  You can use Nix to completely manage your entire workstation from the ground up, as well as declaratively package any type of application you would like to distribute to others.

In future posts, we'll look at how to package your code for others to use, how to use Nix Flakes to define your entire operating system, and take a deeper look at what Nix is doing under the hood to turn these files into development environments.

Nix is a vast topic, but it can significantly streamline how you manage your computer and the tools you need every day.  If there's something specific you'd like to see in this series, please let me know!

*Cover photo by [CHUTTERSNAP](https://unsplash.com/@chuttersnap?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText) on [Unsplash](https://unsplash.com/s/photos/boxes?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText).*
  