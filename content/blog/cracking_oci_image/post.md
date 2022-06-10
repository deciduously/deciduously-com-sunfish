---
author: Ben Lovy
description: A tour of the OCI Image spec
title: Cracking Open an OCI Image
---

Containerization has become a staple of the modern computing landscape, and if you're reading this, there's a good chance you've got [`docker`](https://www.docker.com) or [`podman`](https://podman.io) installed on your development machine already. If not, you likely use a web service that's distributed as an OCI image and maybe orchestrated by something like [Kubernetes](https://kubernetes.io).

All of these containerization tools deal in [OCI](https://opencontainers.org) images to ship descriptions of your application and the userland it needs to run. However, you usually don't interact directly with the actual files on disk that comprise your images. You'd write a [Dockerfile](https://docs.docker.com/engine/reference/builder/) or pull prebuilt images from remote degistries, and let the runtimes deal with the details.

The quick answer is that an OCI image is a directory whose contents conform to the [OCI Image Spec](https://github.com/opencontainers/image-spec/blob/main/spec.md). In this post, we'll explore an actual OCI image alongside this spec.

## Tools

If you'd like to follow allong, you'll need a few tools installed.. See the links for installation instructions.

- [`skopeo`](https://github.com/containers/skopeo). This is a lightwieght tool to intract with OCI-compliaant registries. It just downloads the content directly, without lugging along the whole engine. This is all you need if you just want to poke around.
- Optional: An OCI engine like `docker` or `podman`. Only necessary if you'd like to actually build a container from your image and use it.

<!-- These instructions work on Linux, skopeo copy failed on M1 MacOS. **TODO** windows?? -->

## Obtaining the source

We'll explore the [Alpine Linux](https://www.alpinelinux.org) container, a popular choice because it's very small. If you have to lug an entire Linux userland around to run a single appliction, it pays to make sure you only bring what you strictly need. This container is only 5MB!

We'll grab the [official one](https://hub.docker.com/_/alpine) from [DockerHub](https://hub.docker.com). Switch to a convenient location and obtain the OCI image:

```sh
$ cd ~/images
$
```
