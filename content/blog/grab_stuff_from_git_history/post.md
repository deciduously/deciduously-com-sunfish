---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--BhtSCPcR--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://dev-to-uploads.s3.amazonaws.com/uploads/articles/1tnt5rzkxpj8uig2c8a6.jpg
date: 2021-10-22T12:00:00.000Z
title: Quickly Grab Stuff From Your Git History 
tags:
  - beginners
  - tutorial
  - git
  - productivity
---
While working through a problem, my colleague remembered a prior version of our application had a syntax example we could use.  Thankfully, the codebase has been checked into [`git`](https://git-scm.com/)!  We knew the code in question revolved around the [`mkDerivation`](https://blog.ielliott.io/nix-docs/stdenv-mkDerivation.html) functionality in [nix](https://nixos.org/).

Since then, this particular code has been moved out of our codebase and added to [`nixpkgs`](https://github.com/NixOS/nixpkgs), so we can pull the derivation from the main tree instead of defining our own out-of-tree logic.  My first instinct was to start digging through that (huge) repository to find the file and use that as a reference.

No need!  We have it in our `git` history.  We can query this using `git log -S`:

```
$ git log -S mkDerivation
commit 3a275488e740ae1b4314208a908c5300f9563ee0
Author: David Yamnitsky <david@yamnitsky.com>
Date:   Mon Jul 19 11:51:47 2021 -0400

    use mold and wasm-bindgen from nixpkgs

commit a3a042b5b90ad57ff11bc47a5db6e68dc1ca55e7
Author: David Yamnitsky <david@yamnitsky.com>
Date:   Wed Jun 16 10:26:35 2021 -0400

    use mold as the linker to speed up incremental compiles on x86_64-linux
```

Beautiful - that top commit looks like it represents when we switched to pull this derivation directly from `nixpkgs`.  Removing the code is sufficient - each `git` commit represents a diff.  This commit should show us the code:

```
$ git show 3a275488e740ae1b4314208a908c5300f9563ee0
commit 3a275488e740ae1b4314208a908c5300f9563ee0
Author: David Yamnitsky <david@yamnitsky.com>
Date:   Mon Jul 19 11:51:47 2021 -0400

    use mold and wasm-bindgen from nixpkgs
...
flake. nix

───┐
1: │
───┘
{
  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-unstable";
      url = "github:nixos/nixpkgs/nixos-unstable-small";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";

────┐
48: │
────┘
        CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER = toString ./. + "/scripts/clang";
        CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
        buildInputs = with pkgs; [
          (stdenv.mkDerivation {
            pname = "mold";
            version = "0.9.1";
            src = fetchgit {
              url = "https://github.com/rui314/mold";
              rev = "v0.9.1";
              sha256 = "sha256-yIkW6OCXhlHZ1jC8/yMAdJbSgY9K40POT2zWv6wYr5E=";
            };
            nativeBuildInputs = [ clang_12 cmake lld_12 tbb xxHash zlib openssl git ];
            dontUseCmakeConfigure = "true";
            buildPhase = "make -j $NIX_BUILD_CORES";
            installPhase = "mkdir -p $out $out/bin $out/share/man/man1 && PREFIX=$out make install";
          })
          cachix
          cargo-insta
          clang_12

```

There it is, in the text!  In my terminal, additions are highlighted in green and removals are in red.  This was a removal, but you still get the full removed text.  I was able to copy that `stdenv.mkDerivation` code and work from there.  Thanks, `git`.  (Thit).

As an aside, I highly recommend the following `git` alias:

```
l = "log --all --graph --decorate --abbrev-commit --format=format:'%C(bold blue)%h%C(reset) - %C(bold white)%an%C(reset) %C(bold yellow)%d%C(reset)%n%C(bold cyan)%aD%C(reset) - %C(bold green)(%ar)%C(reset)%n%C(white)%s%C(reset)'";
```

It's a mess of text, but it produces super easy to read git histories:

```
* 9b24232 - Ben Lovy  (HEAD -> main, origin/main, origin/HEAD)
| Fri, 22 Oct 2021 11:49:42 -0400 - (8 hours ago)
| Remove maplit
* 65016b6 - David Yamnitsky 
| Fri, 22 Oct 2021 11:39:12 -0400 - (8 hours ago)
| update deps
| * 6d55d3e - Ben Lovy  (refs/stash)
|/| Fri, 22 Oct 2021 11:49:01 -0400 - (8 hours ago)
| | WIP on main: e07e691 clear 1.56 warnings
| * 6bf8ab1 - Ben Lovy 
|/  Fri, 22 Oct 2021 11:49:01 -0400 - (8 hours ago)
|   index on main: e07e691 clear 1.56 warnings
* e07e691 - David Yamnitsky 
| Fri, 22 Oct 2021 11:09:02 -0400 - (8 hours ago)
| clear 1.56 warnings
```

The coloration doesn't reflect here, in your terminal this will be *even cooler*.  As a `git` novice, this sort of output is instrumental in keeping track of changes to the codebase.
