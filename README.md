<h1 align="center">Fishtank</h1>
<h3 align="center">An interactive container manager for the <code>fish</code> shell.</h3>

<p align="center">
<img src="https://img.shields.io/github/license/Colonial-Dev/fishtank">
<img src="https://img.shields.io/github/stars/Colonial-Dev/fishtank">
</p>⏎

## Features

## Installation

Before installing, make sure you have the following:
- `fish`
- `podman`
- `coreutils`
- `gzip`

## Getting Started

## FAQ

### "Why `fish` instead of POSIX `sh`?"
Well, I *could* have written `fishtank` in POSIX `sh` - in the same sense that I *could* stick a fork into my eye repeatedly.

More seriously - I started `fishtank` as an excuse to learn some scripting after migrating from `bash` to `fish`, and it matured into something I thought others might find useful.

### "Isn't this similar to `toolbox` and `distrobox`?"
Yes! I even used to heavily rely on `toolbox` for my development environments.

My main gripe with both programs (which are otherwise excellent) is that they lack mechanisms to fully control how much they integrate with the host. `toolbox`, for example, always mounts your `$HOME` and runs containers as `--privileged`, even when I would frequently prefer to keep things locked down and only share a subset of my files.

Tanks, by default, are instead more akin to VMs or a remote host. They exist in a vacuum to start, and you choose exactly what (if anything) you want to share with them.