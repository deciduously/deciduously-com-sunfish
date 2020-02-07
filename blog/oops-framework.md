---
cover_image: https://res.cloudinahttps://thepracticaldev.s3.amazonaws.com/i/lk886f5xd4t64pa2cw9i.jpg
edited: 2019-08-21T12:00:00.000Z
title: Oops, I'm Making A Framework
published: true
description: Discussion about abstraction
tags: beginners, help, discuss
---
In a [previous post](https://dev.to/deciduously/towards-complexity-341i), I discussed a project I'm working on to level up the complexity present in my portfolio.  After a few days of deliberation and prototyping I have decided to implement this application in Rust, rendering to an HTML5 canvas via WebAssembly.

I got a prototype working with some clickable buttons using a [previous experiment](https://github.com/deciduously/dots) as an example, but it was brittle.  Every individual region had its own separate detection and handling logic - a mess to tweak or modify.  I decided I wanted a better abstraction over the canvas before going any further, for blood pressure purposes.

Fast forward another two days, and I've essentially outlined a UI framework.  I've decoupled the rendering from the game logic to the point where it could be separated out into its own external standalone library.   It provides a set of traits you can implement for your game's types which fully handle creating the canvas element, mounting your app to it, and handling clicks.  All you need to do is plug in the drawable widgets and it organizes your layout for you.

How does this not always happen?  Or, rather, *does* this always happen?  Does any sufficiently large project end up with its own highly decoupled bits of logic like this?  Does the fact that I'm writing one from scratch mean I should be using a "real" framework instead?  For this project I don't care, it's specifically an educational experience so I don't mind the extra labor, but would this sort of thing be a flag for you in a "real" project?  How much of this would you let yourself implement from scratch before exploring the ecosystem?  The extra control is nice, but it's time consuming, and this problem can be crowd-sourced.

Additionally, if you've found yourself with a DIY framework in the course of writing an application, what drives you to decide whether or not to actually separate and generalize it for public consumption?  I don't know if the world needs another homebrew canvas UI framework, but if I've gone and written one anyway, maybe it's worthwhile to throw it out there.  That said, it's also a whole other set of time-consuming tasks unrelated to this project.

*Photo by Chris Abney on Unsplash*
