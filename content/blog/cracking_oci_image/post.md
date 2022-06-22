---
author: Ben Lovy
date: 2022-06-09T12:00:00.000Z
description: A tour of the OCI Image spec
title: Cracking Open an OCI Image
---

Containerization has become a staple of the modern computing landscape, and if you're reading this, there's a good chance you've got [`docker`](https://www.docker.com) or [`podman`](https://podman.io) installed on your development machine already. If not, you likely use a web service that's distributed as an OCI image and perhaps orchestrated by a tool like [Kubernetes](https://kubernetes.io).

All of these containerization tools deal in [OCI](https://opencontainers.org) images to ship descriptions of your application and the userland it needs to run. However, you usually don't interact directly with the actual files on disk that comprise your images. You'd write a [Dockerfile](https://docs.docker.com/engine/reference/builder/) or pull prebuilt images from remote degistries, and let the runtimes deal with the details. What are you actually producing on disk when you run `docker build`?

The quick answer is that an OCI image is a directory whose contents conform to the [OCI Image Spec](https://github.com/opencontainers/image-spec/blob/main/spec.md), If you're anything like me, that can be a pretty dense and abstract way to approach the topic for the first time. In this post, we'll get hands-on and explore an actual OCI image ourselves, one file at a time.

## Tools

- [`skopeo`](https://github.com/containers/skopeo). This is a lightweight tool to intract with OCI-compliaant registries. It just downloads the content directly, without lugging along the whole engine. This is all you need if you just want to poke around.
- [`umoci`](https://umo.ci/). This is a CLI tool designed to create and manipulate OCI images by hand. We'll use it to interact with the filesystem our image has bundled.

Optionally, you could also install an OCI engine like `docker` or `podman`. Both `skopeo` and `umoci` and able to run without superuser access and don't require installing any system services to use. In contrast, getting a full container engine installed is a bit more involved, but it's only necessary if you'd like to actually build a container from your image and use it. We won't be using it in this post. If you're not sure which to pick, it may help to note that podman has [an absurdly cute logo](https://podman.io/images/podman.svg), so, you know. Do with that what you will.

<!-- These instructions work on Linux, skopeo copy failed on M1 MacOS. **TODO** windows?? -->

## Obtaining The Image

We're going to poke around inside the [Alpine Linux](https://www.alpinelinux.org) container, a popular choice for minimalists. If you have to lug an entire Linux userland around to run a single appliction, it pays to make sure you only bring what you strictly need. This container is only 5MB, but still allows you to access and install packages from a Linux package registry to obtain the tools your app needs to run.

We'll grab the [official one](https://hub.docker.com/_/alpine) from [DockerHub](https://hub.docker.com). Switch to a convenient location and obtain the OCI image:

```txt
$ cd ~/images
~/images $ skopeo copy docker://docker.io/alpine:3.16.0 oci:$PWD/alpine:3.16.0
Getting image source signatures
Copying blob 2408cc74d12b done
Copying config e66264b987 done
Writing manifest to image destination
Storing signatures
```

This command downloads the image to your current directory. The colon and tag are necessary - we'll see why in a moment.

## Inspecting The Structure

Let's take a look at what we've got:

```txt
~/images $ ls -l
drwxr-xr-x@ - deciduously  9 Jun 21:44 alpine

~/images $ du -h alpine/
2.7M    alpine/blobs/sha256
2.7M    alpine/blobs
2.7M    alpine/

~/images $ tree alpine/
alpine
├── blobs
│  └── sha256
│     ├── 1db22e12238c94042b930c0e3559a8b283473b989d621f2c145ebe72829cef25
│     ├── 2408cc74d12b6cd092bb8b516ba7d5e290f485d3eb9672efc00f0583730179e8
│     └── a366738a1861dcdfa50823e8a3701aecd68f8b9e1a8af3820df23f8bd71b1e1d
├── index.json
└── oci-layout
```

Skopeo downloaded a plain directory with only 2.7MB worth of contents - definitely surprisingly lean for an entire Linux distribution's userspace! Further, nearly all of that data lives inside a subdirectory called `blobs/sha256`. Looking more closely at the contents, we have two files at the top level. The index is a [JSON](https://www.ecma-international.org/publications-and-standards/standards/ecma-404/) file and the `oci-layout` is ominously unspecified. Let's check for sure:

```txt
~/images $ file alpine/oci-layout
alpine/oci-layout: JSON text data
```

It's just more JSON. Anticlimactic, but fine. That blob directory is a little more cryptic. What are those?

```txt
~/images $ for file in alpine/blobs/sha256/*; do file $file; done
alpine/blobs/sha256/1db22e12238c94042b930c0e3559a8b283473b989d621f2c145ebe72829cef25: JSON text data
alpine/blobs/sha256/2408cc74d12b6cd092bb8b516ba7d5e290f485d3eb9672efc00f0583730179e8: gzip compressed data, original size modulo 2^32 5811200
alpine/blobs/sha256/a366738a1861dcdfa50823e8a3701aecd68f8b9e1a8af3820df23f8bd71b1e1d: JSON text data
```

We've got some more JSON, and something that's been compressed, but it's unclear what. Some readers may recognize this as a content-adressed store, but we'll get back to that in a moment.

THis layout is documented in the spec on the [Image Layout](https://github.com/opencontainers/image-spec/blob/main/image-layout.md) page.

Let's start cracking some of these files.

## Components

There's only five files here! Not so bad.

### oci-layout

We'll start with the smallest, `oci-layout`:

```json
{ "imageLayoutVersion": "1.0.0" }
```

Not much to say about this one. Images are tagged with the version of the spec they implement, which is probably a good idea. This is documented [here](https://github.com/opencontainers/image-spec/blob/main/image-layout.md#oci-layout-file).
