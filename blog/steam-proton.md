---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--xVx2wib8--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://dev-to-uploads.s3.amazonaws.com/i/64ox1zbuydq1va8n6bfj.jpg
edited: 2020-02-08T12:00:00.000Z
title: Steam Proton Is Great
published: true
description: My experience with Steam Proton.
tags: offtopic, games, linux
---
My (awesome) fiancé picked me up a copy of [Disco Elysium](https://zaumstudio.com/) for my birthday recently (she's *really awesome*).

I've always kept a [Windows 10](https://www.microsoft.com/en-us/windows/get-windows-10) installation around specifically for games.  Cross platform support has increased dramatically since I started using Linux, but there have always been a handful of (usually AAA) titles that I want to play which require Windows.  I know, I know, I'm a terrible Linux zealot.

I've used [Wine](https://www.winehq.org/) before, as well as the wrapper [PlayOnLinux](https://www.playonlinux.com/en/), but had generally experienced that as an activity unto itself.  Some games did work, but there was a lot of configuration and tweaking involved.  While there's a part of me that enjoys that process for the sake of it, there's an even bigger part of me that just wants to play my games.  It gets old.

Steam recently released [Proton](https://www.protondb.com/).  Proton is also a wrapper around Wine, but (apparently) there's a lot more engineering involved here too.  I had initially sort of written it off as "just another Wine tool" but now having used it, I'm completely sold.

Y'all, it works *perfectly*.

I was originally disappointed when I enabled the Proton option on the Linux client because most of my games weren't showing up.  They have a curated set of games that Valve has specifically given a seal of approval.

However, the tool *does* run on anything you've got.  You just need to specifically opt in:

![settings screenshot](https://dev-to-uploads.s3.amazonaws.com/i/5hps8b6dpjwd7uecg734.png)

With that checked, you can attempt to run even non-Valve-ordained titles through Proton.

Disco Elysium worked fine out of the box with the default audio and graphics settings.  The only problem is that it would crash every 45-ish minutes.  The fix was adding an option to the launch configuration:

![launch options](https://dev-to-uploads.s3.amazonaws.com/i/1f1b0y4qn4ixi1eeyqvj.png)

Esync is a [Proton tool](https://www.protondb.com/help/improving-performance) that attempts to reduce the CPU overhead of running games in Wine, but apparently it doesn't play nice with this game.  I haven't had a single crash since, and do not experience performance problems.  I can play this brand new Windows-exclusive game exactly as intended by the developer without having to leave Linux, ever, and my configuration phase consisted of a single web search and a single config option to set.

Welcome to the future, I guess!

Also, Disco Elysium is fantastic.  It's even better than I thought it would be.  If you're into role-playing games and looking for a completely fresh take on the genre, look no further.  It's like a pen-and-paper RPG that takes place inside your wacky messed-up character's head - combat is replaced by dialogue, and it's hilarious, philosophical, depressing and beautiful all at once.  Cannot recommend enough.

*Photo by Raphaël Biscaldi on Unsplash*
