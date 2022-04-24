---
title: Getting Cozy With Debian Buster
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--ZBRSmA_v--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://thepracticaldev.s3.amazonaws.com/i/g3gxkpx0h3crwhl1ex7t.jpeg
tags:
  - linux
  - debian
  - devjournal
  - beginners
date: 2020-01-01T12:00:00.000Z
---

## Yes, Really, Even After All That

This was an unexpectedly phenomenal discussion:

{# {% post deciduously/au-revoir-gentoo-sell-me-a-new-linux-distro-4d3e %} #}

Thank you, DEV, seriously, I'm blown away by how many of you were compelled to add your take. The discussion there turned into a highly useful resource with a wide array of perspectives, experiences, and opinions from people who really know what they're talking about, and I'll be returning to it as a reference time and again.

I carefully read every single comment, and took all of your well-informed perspectives into consideration, but ultimately went with my initial instinct and installed [Debian Buster](https://www.debian.org/releases/buster/).

The [first half](#trusting-instinct) of this post is a somewhat defensive run-down of why I picked [Debian](https://www.debian.org/) instead of, oh, pretty much anything else, and the [second half](#installation-log) is a log of the installation process and post-install configuration I've done so far, more for my own reference than anything else.

## Trusting Instinct

I'm sure many of you are shaking your heads at me, wondering how I could have made such a poor choice after all that discussion. It's actually happening to me too, don't worry. I'm looking at this as an experiment.

The two runners up after reading through the responses were [KDE Neon](https://neon.kde.org/) and [Pop!\_OS](https://system76.com/pop). I hadn't even considered Pop!\_OS initially, and am quite glad so many DEV users spoke so highly of it - it's [considerably cooler](https://pop.system76.com/docs/difference-between-pop-ubuntu/) than I thought. I think there's a good chance y'all are correct about Debian and its shortcomings as a main work distribution, and if so I'm headed straight for Pop!\_OS. I guess I just need to see it for myself first.

At the end of the day, while I think [Manjaro](https://manjaro.org/) or [Solus](https://getsol.us/home/) sound fun, or sticking with Gentoo or a derivative like [Calculate](https://www.calculate-linux.org/en/), Debian compatibility was the primary driving factor. Once that decision had been made, it came down to whether I wanted to pin to Debian Stable or [Ubuntu LTS](https://wiki.ubuntu.com/BionicBeaver/ReleaseNotes) as a base.

Between those two, most (but not all) of you recommended Ubuntu (or an Ubuntu derivative like both Neon and Pop!\_OS) because it more squarely fits my "just forget about it" criterion. Debian's philosophy and user experience is great for servers, but perhaps not designed for desktop end-users. I think that's generally true, but wasn't completely sold that it wasn't right for me specifically anyway.

The primary shortcoming mentioned for Debian was that all of my software will always be out of date. I gave this some thought and looked at my current software usage, and concluded that I don't actually care. I'd much rather have a system that's thoroughly vetted and tested than get all the cool new stuff that comes out. I often never use it anyway, or at least am happy to wait for some nifty KDE trick until it does trickle its way into Debian in a few years. I kind of can't believe how long I spent on bleeding edge rolling-release distributions for - if I'm being honest with myself - very little reason. It makes sense for a lot of people who do care about and use that stuff, but I really just don't. So what if the version of `cairo` I'm on is years old? It's doing the thing I need it to do. More specifically, what I need is for it and all of my system components to play well together, regardless of what specific versions of each I'm running. That's Debian's entire raison d'être: stable doesn't break.

I also don't need everything to "just work" without intervention, which is Ubuntu's sales pitch. Thinking about it more, I don't think "just forget about it" is really my goal. I want to keep learning from my system, I just don't want to waste time. I don't think Gentoo itself is necessarily the culprit (it's definitely me, I'm the culprit), but it's not helping in that regard either. Debian is boring in a way that I need it to be, and Gentoo isn't.

That said, I am capable of and generally don't mind configuring my system and do like retaining that control. I had to install non-free drivers for my wireless and graphics cards, so now I _know_ what my system needs in the Debian universe. Now that I've done it once, it's just as much a non-issue ongoing maintenance-wise as Ubuntu would have been. The problems come for me when that extra control leads to time wasted, or creating extra work for work's sake. I don't mind a little real work to get everything the way I want it to be, though, especially if it results in a highly stable system.

There is still a bit of hesitation, because I know inevitably I will want a more updated version of something than what Buster includes, and I did get a bit of a mixed review when it comes to selectively updating specific packages. Debian packaging itself is mythically horrific, a far cry from what I'm used to with [ebuilds](https://wiki.gentoo.org/wiki/Basic_guide_to_write_Gentoo_Ebuilds).

Ultimately, I chose Debian because it's an industry standard, and the base from which all these other operating systems are derived. I think it's important to become well versed in what's actually widely used in the real world as well as what I find elegant and fun. This philosophy guides a lot of my tool choices - my two main programming languages right now are [C++](https://en.cppreference.com/w/cpp/language) and [Rust](https://www.rust-lang.org/), I like experimenting with alternate shells like [`zsh`](https://en.wikipedia.org/wiki/Z_shell) and [`fish`](https://fishshell.com/) and scripting tools like [ClojureScript](https://clojurescript.org/)/[`planck`](https://planck-repl.org/) but I still make sure to learn plain [`bash`](https://www.gnu.org/software/bash/), I try to make a point of practicing functional programming concepts in both [Haskell](https://www.haskell.org/) and [JavaScript](https://www.ecma-international.org/ecma-262/10.0/index.html#Title), et cetera. I get a lot out of comparing and contrasting the tools I learn by learning two sides of a particular coin concurrently, and end up with (I think) a deeper understanding of the pros and cons of both as well as a working familiarity with the one more likely to be useful in the long term. It makes sense to me to apply this thinking to my OS as well.

For this reason, I've decided to see Debian through for a while and then potentially re-assess down the road if it turns out I am spending too much time making it work instead of working. At that point, a more polished Ubuntu LTS-based experience will likely be the remedy. I know it's far too early to really tell, but for now I'm completely satisfied with Debian Stable.

## Installation Log

The above was a mild fib. The very first action I took was trying to install KDE Neon, but the live disc didn't boot on my hardware in either graphical or OEM mode. It got through GRUB and then crashed, every time. I wrote it to different media, tried multiple times, nothing. While I still kinda would have liked to try this distro, and know there absolutely would have been a way to get it running - I'm pretty sure it's just a graphics card issue - that's a poor first impression, and pretty much why I ran straight to Debian next, and in retrospect I am glad I did.

### Getting to First Boot

As expected, Buster installed without a hitch on the first try. New in Buster is [UEFI Secure Boot](https://wiki.debian.org/SecureBoot) support, so I didn't have to do a thing first. I just cleared the way in my partition table and let the installer do it's thang. It correctly found my other operating systems ([Windows 10](https://www.microsoft.com/en-us/windows/get-windows-10), [NixOS](https://nixos.org/)) and installed GRUB the way I would have done it myself with minimal hand-holding.

The migration from Gentoo was painless. I tarballed all my PDFs and popped them on a flash drive, then reformatted my Gentoo partition. Seven years of configuration, obliterated in a `mkfs`. Everything else I need is hosted in an offsite git repo, including dotfiles. No use getting sentimental!

Okay, I lied again - It actually wasn't _quite_ without a hitch but it was my fault, not Debian's. I was a little stupid and didn't bother customizing the install, so initially I had a similar problem to the KDE Neon disc - it would start the boot process and then die trying to load the graphics driver. Luckily, the Debian live disc has a "recover" mode, and I was able to enter a rescue shell on my brand new install. All I needed to do was edit `/etc/apt/sources.list` to include `contrib` and `non-free`:

```bash
# https://unix.stackexchange.com/questions/449794/installing-nvidia-driver-for-debian-stretch
sudo sed -i.bak 's/buster[^ ]* main$/& contrib non-free/g' /etc/apt/sources.list
```

Then I ran `apt update && apt install nvidia-drivers`. This actually ended up crashing the live CD once the driver was installed, but then it booted up from the hard drive perfectly. Solved!

I used a wired connection to run the installation from the `netinst` disc image. The default installation didn't install the proprietary driver I needed for my Realtek WNIC, but the above step enabled the `non-free` repository I needed. I installed the firmware with `apt install firmware-realtek` and was immediately able to connect to my home LAN, no other configuration was needed. The base KDE installation came with NetworkManager and `plasma-nm`.

### Post-Install

I decided instead of trying to replicate any experience I had in the past from a tools-first perspective off the bat based on a preconceived notion of what I'd need, I'd just try to go about my business normally. This would force me to install and configure stuff as I go organically in hopes of cutting to the core of what I actually care about. This is what I've done so far.

#### Preloaded Software

My first order of business was a web browser. My currently preferred browser is [Firefox](https://www.mozilla.org/en-US/firefox/) and the options I chose at install came preloaded with Firefox ESR v68.3.0. Eventually this might be one that I look in to getting more recent updates for especially if I end up working more with web development than I currently do but it's fine with me for now.

I did a quick webcam test call, both audio and video worked out of the box on my external USB device with no additional installation or configuration.

I also use [LibreOffice](https://www.libreoffice.org/) for schoolwork and spreadsheets. This installation came preloaded with version 6.1.5.2 - also fine with me. I also frequently use both [GIMP](https://www.gimp.org/) and [Imagemagick](https://imagemagick.org/index.php), which were both preloaded as well. Core components like `gpg` and `ssh` were also preloaded and I was able to configure both as needed with no surprises. A comprehensive suite of KDE software is included, unlike Gentoo which gives you the basic desktop and lets you pick and choose. There's a bunch of these I don't use but some I do almost daily like [Spectacle](https://kde.org/applications/utilities/org.kde.spectacle), [Dolphin](https://kde.org/applications/system/org.kde.dolphin), [Kate](https://kde.org/applications/utilities/org.kde.kate), [KNotes](https://kde.org/applications/utilities/org.kde.knotes), [KCalc](https://utils.kde.org/projects/kcalc/), and [Okular](https://okular.kde.org/). It was honestly great to not have to install anything extra to click right back into my familiar, comfortable workflow.

All in all, the default set of packages is not excessively bloated at all and largely consists of either things I actively want and use or little extra KDE components that I don't mind having around and may even want to try someday. The extra stuff I installed was pretty much entirely confined to development environment tools. That's an A+ experience in my book.

#### Acquiring My Development Environment

Before getting started with the process of building out the system I need, I needed to add my user to the `sudo` group. Weirdly `usermod` lives in `/usr/sbin` but that's not in `$PATH` by default, so I added `export PATH="$PATH:/usr/sbin/"` to `.bashrc` for both root and my non-admin account. Then I was able to run `usermod -a -G sudo ben`, narrowly avoiding starting off my Debian career by having an unauthorized `sudo` incident ominously reported.

Now able to run `apt` safely from my user account, I decided to pull down a few Rust and C++ repositories I've worked on recently and compile them.

First, though, I installed my preferred development editors. [Emacs](https://www.gnu.org/software/emacs/) went in with `apt install emacs`, but lately I use [Visual Studio Code](https://code.visualstudio.com/) for pretty much everything except a few specific cases. Buster doesn't package this one, so I downloaded the `.deb` directly and invoked `sudo apt install ./code_1.41.1-1576681836_amd64.deb` - easy enough. I also needed to install [`rustup`](https://rustup.rs) directly using their `curl` one-liner which doesn't come by default. At that point I grabbed a few other extras I knew I'd need immediately: `apt install curl git htop tmux`. At this point I've always been in the habit of installing `neovim`, but I skipped it for now - there's nothing wrong with `nano` for tasks I used to use `nvim` for, and it's usually available everywhere.

The Rust toolchain installed fine, but `cargo install cargo-update` failed when building `openssl_sys`. I needed to install both `pkg-config` and the `ssl` development headers: `apt install pkg-config libssl-dev`. To build my [`music`](https://github.com/deciduously/music) project that interfaces with the system audio output device, I needed the ALSA development headers: `apt install libasound2-dev`.

Getting my [`nannou_dots`](https://github.com/deciduously/nannou_dots) project running was a little more complicated. I needed the following tools from Buster repos: `apt install cmake python3-distutils libxcb-render0-dev libxcb-xfixes0-dev libxcb-shape0-dev`. It also depends on the [Vulkan SDK](https://www.lunarg.com/vulkan-sdk/), and I was pleasantly surprised to find I could follow the instructions to [add the Ubuntu 18.04 PPA](https://vulkan.lunarg.com/doc/sdk/latest/linux/getting_started_ubuntu.html) exactly and have it work fine: `apt update && apt install vulkan-sdk`. Afterwards the `vkvia` test program executed without errors and I could build and run my demo app.

I've heard that's a pretty bad idea. How bad, exactly, is it? What Ubuntu<->Debian incompatibilities should I be aware of?

For C++, I installed a few tools: `apt install clang cppcheck gdb valgrind`.

I also use [Haskell](https://www.haskell.org/) sometimes: `apt install ghc`.

I set up [Node](https://nodejs.org/en/) and [`pnpm`](https://pnpm.js.org/) with `sudo apt install npm && sudo npm install -g pnpm`.

I also have a soft spot for [`bc`](https://www.geeksforgeeks.org/bc-command-linux-examples/) for doing arithmetic in a terminal: `apt install bc`.

At this point, I feel I've more or less met all the needs I have after only a few hours, and feel generally confident both that I will be able to meet new ones as they arise and that I shouldn't need to touch it much at all. Being Debian-compatible does feel like a breath of fresh air even though I did genuinely enjoy Gentoo and found it easy to use, and being Debian _itself_ hasn't thus far proven to be a roadblock.

#### Notes

You can search for a package by keyword with `apt-cache search`. You can get the current Debian version in `/etc/debian_version`: `10.2`.
