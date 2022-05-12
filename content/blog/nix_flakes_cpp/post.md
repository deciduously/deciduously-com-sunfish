---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--XfaxMGa4--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://dev-to-uploads.s3.amazonaws.com/uploads/articles/nfo5hzms269zpdv9yzib.jpg
date: 2021-10-31T12:00:00.000Z
title: "Workstation Management With Nix Flakes: Build a Cmake C++ Package" 
tags:
  - tutorial
  - cpp
  - beginners
  - nix
---
[Last time](/blog/nix_flakes_jupyter/), we looked at how to produce a development shell using Nix Flakes that contained the Python interpreter alongside a few dependencies the project required. In this post, we'll create a compiled binary, using source code hosted on GitHub, for other people to include in their environments.

## The Target

For this demonstration, I'll be packaging the [LightGBM](https://github.com/microsoft/LightGBM) CLI tool. Nixpkgs already provides derivations for the native libraries for Python and R, which will suffice for most users, but I didn't see one to build the CLI tool directly (and, of course, will submit mine upstream as well).

Per [the documentation](https://lightgbm.readthedocs.io/en/latest/Installation-Guide.html#linux), building this application from source is relatively straightforward:

```
git clone --recursive https://github.com/microsoft/LightGBM
cd LightGBM
mkdir build
cd build
cmake ..
make -j4
```

We can learn a few things: this git repository has submodules, meaning we need to use `--recursive`, and the build can be parallelized. These instructions also have you create a build directory, but with Nix, we can skip that. The whole build will take place in a purpose-made build directory.

## The Flake

As before, I'll show the complete flake first. We'll walk through it in pieces below.

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
      lightgbm-cli = (with pkgs; stdenv.mkDerivation {
          pname = "lightgbm-cli";
          version = "3.3.1";
          src = fetchgit {
            url = "https://github.com/microsoft/LightGBM";
            rev = "v3.3.1";
            sha256 = "pBrsey0RpxxvlwSKrOJEBQp7Hd9Yzr5w5OdUuyFpgF8=";
            fetchSubmodules = true;
          };
          nativeBuildInputs = [
            clang
            cmake
          ];
          buildPhase = "make -j $NIX_BUILD_CORES";
          installPhase = ''
            mkdir -p $out/bin
            mv $TMP/LightGBM/lightgbm $out/bin
          '';
        }
      );
    in rec {
      defaultApp = flake-utils.lib.mkApp {
        drv = defaultPackage;
      };
      defaultPackage = lightgbm-cli;
      devShell = pkgs.mkShell {
        buildInputs = [
          lightgbm-cli
        ];
      };
    }
  );
}
```

If you read the last post, much of this looks the same. We have `inputs` and `outputs`, a `let...in` section defining variables, and a `devShell` in the output.

The new bits are the `lightgbm-cli` variable, defined using `stdenv.mkDerivation`, and the `defaultApp` and `defaultPackage` keys in the `outputs` section.

## The Walkthrough

I won't repeat the explanations of the shared elements. Take a look at the [previous post](https://dev.to/deciduously/workspace-management-with-nix-flakes-jupyter-notebook-example-2kke) for the full descriptions. This post will focus on the `mkDerivation` section.

The `pkgs.stdenv.mkDerivation` function is a wrapper around the low-level Nix concept of a `derivation`. See the [Nix Pills](https://nixos.org/guides/nix-pills/our-first-derivation.html) for a thorough walkthrough of how to work with them. Using `mkDerivation` sets some sensible defaults. It provides more tools on top of this base to greatly streamline the process, but it's important to know that this `derivation` concept is what ultimately gets evaluated under the hood.

We want to refer to this derivation in multiple places, so it gets a specific name assigned in the `let...in` block:

```nix
lightgbm-cli = (with pkgs; stdenv.mkDerivation {
    # ...
  }
);
```

By prefixing the function using `with pkgs;` we can avoid explicitly referring to `pkgs` every time we use something from this set. The first example is right here. the `stdenv` object lives in `pkgs`, so fully qualified, this opener would be `pkgs.stdenv.mkDerivation`.

### Name

Inside, we first set the package name. You can directly set a `name`, but the preferred way to handle this is to set the package name using `pname`, and the version separately. Nix will then combine them into a whole `name` key for you with the format `${pname}-${version}`.

```nix
pname = "lightgbm-cli";
version = "3.3.1";
```

### Source

Then, we need to point Nix at our source code. If you were packaging a source that you were building yourself, and this flake lived in the project directory, you could point at the current folder:

```nix
src = ./.;
```

However, we're just creating a derivation for an existing codebase hosted online in a Git repository. We can use the `pkgs.fetchgit` function to direct Nix to download the source directly before building:

```nix
src = fetchgit {
  url = "https://github.com/microsoft/LightGBM";
  rev = "v3.3.1";
  sha256 = "pBrsey0RpxxvlwSKrOJEBQp7Hd9Yzr5w5OdUuyFpgF8=";
  fetchSubmodules = true;
};
```

We provide the URL and the specific tag, so we're not just pointing right at the main branch. You can also set a particular commit, tagged or not, by giving the complete git commit hash:

```nix
rev = "d4851c3381495d9a065d49e848fbf291a408477d";
```

If we didn't need to fetch any submodules, and the repo was on GitHub specifically, there's an even better function to use called `pkgs.fetchFromGithub`:

```nix
src = fetchFromGitHub {
  owner = "microsoft";
  repo = "LightGBM";
  rev = "v3.3.1";
  hash = "sha256-pBrsey0RpxxvlwSKrOJEBQp7Hd9Yzr5w5OdUuyFpgF8=";
}
```

You don't need to provide a full URL, Nix needs the owner and repo name, and you can use the more generic `hash` key, prefixing the actual hash so that Nix knows how to resolve it. However, I was unable to get this working with submodules, and this wouldn't work for code hosted elsewhere, like [GitLab](https://about.gitlab.com/) or [sourcehut](https://sr.ht/).

Before you run your derivation the first time, you probably won't know the sha256 hash of the source repository. While you could download it separately and use the `nix hash-path path/to/clone` command, I find it more convenient to provide a bad hash first and let Nix tell you what it should be. There's even a built-in feature for this:

```nix
sha256 = lib.fakeSha256;
```

On the first run, you'll get an error like this:

```
$ nix build
error: hash mismatch in fixed-output derivation '/nix/store/5ysvmxay83fnc14w5r2s450i39byd4ks-source.drv':
         specified: sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=
            got:    sha256-pBrsey0RpxxvlwSKrOJEBQp7Hd9Yzr5w5OdUuyFpgF8=
error: 1 dependencies of derivation '/nix/store/jaip55q9ix8hq4l7srd9kigxlq7nimyr-lightgbm-cli-3.3.1.drv' failed to build
```

It downloaded the repository, checked the hash, and (thankfully) noticed it's not the same as the `fakeSha256` hash you provided. Conveniently, it tells us what the hash _should_ be so that you can change your code:

```diff
- sha256 = lib.fakeSha256;
+ sha256 = "pBrsey0RpxxvlwSKrOJEBQp7Hd9Yzr5w5OdUuyFpgF8=";
```

When you rerun it, the hashes will match, and Nix will continue to the next phase.

### Inputs

Now that we have the source, we need to tell Nix what tools are necessary to build the software. We can use the `nativeBuildInputs` key for this:

```nix
nativeBuildInputs = [
  clang
  cmake
];
```

Configuration is handled via `cmake`, and we'll have `clang` be our C++ compiler.

### Package

To control the build, you can use the phases provided by `mkDerviation`: `unpack`, `patch`, `configure`, `build`, `check`, and `install`. However, if you need default behavior for any of these, you can omit them. You only need to define logic when it differs from the defaults. To learn more about these phases, see [this article](https://github.com/samdroid-apps/nix-articles/blob/master/04-proper-mkderivation.md).

Setting `cmake` in our `nativeBuildInputs` means that Nix already configures this project by default. We don't have any unique unpacking to do or any patches to apply, and we're not going to check the compiled output. These defaults mean we only need to define a `buildPhase` and an `installPhase`.

The `build` part is easy:

```nix
buildPhase = "make -j $NIX_BUILD_CORES";
```

By this point, Nix has already implicitly run `cmake .`, so our Makefiles are in place and ready to go. We could omit this phase and have Nix execute `make`, but we want to use as many cores as possible to speed up the build. Nix provides a special environment variable `$NIX_BUILD_CORES` that we can pass to `make`'s job flag to be sure this build runs as efficiently as it can.

Then, we need to copy the resulting binary to the output of the flake:

```nix
installPhase = ''
  mkdir -p $out/bin
  mv $TMP/LightGBM/lightgbm $out/bin
'';
```

The result of your derivation gets assigned a special `$out` variable. If your derivation ultimately produces one or more binaries, you can create a `$out/bin` directory to hold these. Any Nix expression that depends on your derivation will automatically add the contents of this directory to your `$PATH`.

In this case, we need the `lightgbm` executable file produced in the `build` phase. When we downloaded the GitHub source, Nix placed it in the `LightGBM` directory within the special `$TMP` build directory for the derivation. `$TMP` is an alias for `$NIX_BUILD_TOP`. You can also use `$TMPDIR`, `$TEMPDIR`, or `TEMP`. Nix creates all these while running your derivation to prevent processes from using `/tmp` outside this special Nix-y place.

If you had opted to pin to a specific commit instead of a tag, this folder would be called, e.g. `$TMP/LightGBM-d4581c3`, including the beginning of the complete commit hash.

If you get confused about the directory structure, I find the easiest trick is to dump the whole thing:

```nix
installPhase = ''
  mkdir $out
  cp -r $TMP $out
'';
```

When you run `nix build`, you'll see the entire contents of the build directory in the `result` symlink, which you can explore to understand where everything ended up.

### Fixed-Output Derivations

I don't use this feature directly in this example, but it's worth mentioning that you don't have access to the internet in your phase descriptions. This restriction is to keep Nix derivations pure. However, what if you wanted to? You might need to `curl` something or use `maven` to download dependencies before building. There are myriad reasons you may wish to access the internet beyond simply grabbing the source code to build a given project.

To allow this to happen, you can provide an output hash of the result so that Nix can prove to itself that your derivation is reproducible. Like before, you won't know this result ahead of time, so you'll need to watch it fail once and tell you the intended result, but once you add that to your flake, Nix will happily grab these remote files and include them in your derivation result.

First, you need to add `cacert` to your derivation and set the `SSL_CERT_FILE` variable:

```nix
buildInputs = [ cacert ];
SSL_CERT_FILE = "${cacert}/etc/ssl/certs/ca-bundle.crt";
```

Now, Nix will pull the required SSL certificates into the build sandbox so that tools like `curl` can access them to make network requests.

Then, you add the hash:

```nix
outputHash = "sha256-I4UGDcrtmX/1TAQz89peXsqoetZmCM+1b3XYqexv/VA=";
```

This config should be sufficient if your derivation result is a file. However, if you're producing a folder structure, you'll also need to add this key:

```nix
outputHashMode = "recursive";
```

By default, `outputHashMode` is set to `"flat"`, so it will fail to hash a recursive directory tree properly. Luckily it's easy enough to toggle. Your derivation is still totally reproducible and safe to use in a pure evaluation environment, even if it requires downloading additional files from the internet.

In fact, this is what the `pkgs.fetchgit` convenience helper is doing under the hood.  _Everything_ in Nix is a derivation, and the hash you provide to this function gets used to satisfy this exact requirement.

### Outputs

Now, the `lightgbm-cli` variable points to the complete derivation. For usage, we want this flake to be usable via `nix build` using `defaultPackage`, `nix develop` using `devShell`, or `nix run` using `defaultApp`. All of these are easy to do now:

```nix
defaultApp = flake-utils.lib.mkApp {
  drv = defaultPackage;
};
defaultPackage = lightgbm-cli;
devShell = pkgs.mkShell {
  buildInputs = [
    lightgbm-cli
  ];
};
```

Now, users can access a ready-made `lightgbm` executable no matter how they intend to interact with this flake or integrate it into their environments. We lean on the `mkApp` feature provided by `flake-utils` to produce the particular structure required by `defaultApp`/`nix run` and pass it the `defaultPackage` key. Because this whole block was marked `rec`, it doesn't matter what order we define them. Nix will recursively evaluate this block until everything is thoroughly defined.

If you have Nix installed, you should be able to create a new folder, paste the contents above into `flake.nix`, and use these commands to grab, compile, and use `lightgbm` yourself no matter what platform you're using.

Neat!

_Cover photo by [Henry & Co.](https://unsplash.com/@hngstrm?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText) on [Unsplash](https://unsplash.com/s/photos/builder?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText)_
