---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--yRY2WEdr--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://dev-to-uploads.s3.amazonaws.com/uploads/articles/rkq7m254287fybp5ku2o.jpg
date: 2021-10-25T12:00:00.000Z
title: Pretend You're Using A Different Linux Distribution With One Docker Command 
tags:
  - beginners
  - linux
  - productivity
  - docker
---
## The Why

Lots of developers use Linux, but "Linux" is a vast category.  There are a lot of similarities, but a lot of differences, too.  [Ubuntu](https://ubuntu.com/) and [Fedora](https://getfedora.org/) look and feel pretty similar until you try to install a package - whoops, `apt` is a little different than `dnf`. Specific system settings may be stored in different places, and particular commands may be included by default in one but not the other.  Then there's even more niche stuff like [Arch](https://archlinux.org/), which you install piece by piece from a very minimal package set, or [Gentoo](https://www.gentoo.org/), which is similar to Arch with the additional caveat that the user compiles all the software locally for their specific hardware.  Users of these distros may end up with pretty different-looking operating systems that all fall under the broad Linux umbrella.

All of the above adhere to a structure called the [Filesystem Hierarchy Standard](https://en.wikipedia.org/wiki/Filesystem_Hierarchy_Standard), FHS for short.  This specifies the standard top-level hierarchy common to these different flavors, like `/etc` for configuration, `/boot` for bootloader files, `/proc` for process management, and `/home` for user-specific home directories.  See the Wikipedia link for a more complete list.  If you're a Linux user, this structure will feel familiar to you.

However, even the FHS is not universal.  My personal development machine is running a super weird Linux flavor called [NixOS](https://nixos.org/).  This fully declarative system stores every single component of functionality in a unique directory called `/nix/store` and maintains a web of symlinks.  Software compiled for standard Linux distributions won't run on NixOS or vice versa without specifically patching the resulting executable binaries.  To complicate things further, I'm pinning to the unstable channel instead of a tagged release, meaning the package set is liable to change at any time.  While there are a lot of benefits, it means my local machine is fundamentally incompatible with the Linux computers I want the software I produce to run on.

## The Point

I'm primarily writing code in [Rust](https://www.rust-lang.org/), which has powerful facilities for cross-compiling non-native targets built-in, and Nix can help me fill in the rest.  This is great!  From my local computer, I can produce working binaries for many different types of computers.

For example, we want to support [Ubuntu 18.04](https://releases.ubuntu.com/18.04/), one [Long-Term Support](https://ubuntu.com/blog/what-is-an-ubuntu-lts-release) release behind the current LTS, 20.04.  This is several years old by this point, and as a result, only has, for example, [`glibc`](https://www.gnu.org/software/libc/) version 2.27, instead of the current 2.34.  This is crucial for compatibility, because almost every program depends on your OS providing this library and being able to use whichever version it finds.

However, how would I know that my result on my bleeding-edge NixOS box works as intended?  [Containers](https://www.docker.com/resources/what-container) to the rescue!  We can ask [Docker](https://www.docker.com/) to build an Ubuntu 18.04 container and drop us into a shell with the *current* filesystem available.  It's kind of like the `su` command, except instead of switching the active user, you're changing your whole OS on the fly.

Here's the line:

```
$ docker run --rm -it -v $PWD:/working-dir docker.io/ubuntu:18.04
root@6bb49a338644:/# cat /etc/lsb-release 
DISTRIB_ID=Ubuntu
DISTRIB_RELEASE=18.04
DISTRIB_CODENAME=bionic
DISTRIB_DESCRIPTION="Ubuntu 18.04.6 LTS"
root@487693de818e:/# cd working-dir/
root@487693de818e:/working-dir# ls
Cargo.lock  Cargo.toml  README.md  custom-target.json  dist  flake.lock  flake.nix  hello  hello_build  main.rs  scripts  target  x86_64-unknown-linux-gnu2.24.json  x86_64-unknown-linux-musldynamic.json  zig
root@7ae91a94a888:/working-dir# exit
$
```

Perfect!  The `/working-dir` directory inside your new Ubuntu 18.04 container now has the contents of whichever directory you were in when you ran this command.  That's the `-v $PWD:/working-dir` part.  The `$PWD` variable returns the current working directory, and after the colon, you provide a location in the new container to mount this directory.

As far as any software inside is concerned, it's running in a standard Ubuntu installation.  This lets me quickly verify that my program's cross-compiled, binary-patched version runs as expected on this target environment.  When you're done, just type `exit` to return to your native shell.  The `-it` flag made the container interactive, and the `--rm` flag will clean up the Docker container when it quits.

You can check out the [Docker Hub](https://hub.docker.com/search?category=os&source=verified&type=image) or [quay.io](https://quay.io/) for other available docker containers to spin up.

This tip works with any tool that supports the Docker API, in addition to Docker itself.  I'm running it via [Podman](https://podman.io/), and it works the same way.

Now you can use whatever crazy environment you want and still responsibly ensure whatever you're compiling will work as intended for your users.

Cover image photo by [Braydon Anderson](https://unsplash.com/@braydona?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText) on [Unsplash](https://unsplash.com/s/photos/disguise?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText)
  