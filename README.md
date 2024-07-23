<h1 align="center">Fishtank</h1>
<h3 align="center">An interactive container manager for the <code>fish</code> shell.</h3>

<p align="center">
<img src="https://img.shields.io/github/actions/workflow/status/Colonial-Dev/fishtank/fish.yml">
<img src="https://img.shields.io/github/license/Colonial-Dev/fishtank">
<img src="https://img.shields.io/github/stars/Colonial-Dev/fishtank">
</p>

## Features
Easily create and manage container environments for interactive use. All host integration is strictly opt-in; you choose what (if anything) is shared with each container.

{Image or ASCIInema}

Bring your existing Docker-style container definitions...

{Image or ASCIInema}

... or take advantage of Fishtank's custom shell-based format that bundles together all the information needed to build *and* run your containers.

{Image or ASCIInema}

Lightweight, easy to install, and works on almost any Linux machine with `podman`.

{Image or ASCIInema}

## Installation
Before installing, make sure you have the following packages on your system:
- `fish`
- `podman`
- `coreutils`

```sh
curl --proto '=https' --tlsv1.2 -sSf $URL | source - install self
```

This downloads the latest stable version of Fishtank and uses the self-update functionality to bootstrap a persistent install.

By default, this installs two scripts (`tankctl` and `tankcfg`) in `$XDG_CONFIG_HOME` (by default, `$HOME/.local/bin`) - to override the install location, simply pass your preferred path as the third argument:

```sh
curl --proto '=https' --tlsv1.2 -sSf $URL | source - install self /usr/bin
```

### From Source
If you are allergic to `curl | exec`, "building" Fishtank from source using the bundled `do` script is also possible.

```sh
git clone https://github.com/Colonial-Dev/fishtank && cd fishtank
./do build
cp target/* $HOME/.local/bin
```

## Getting Started

## FAQ

### "Why `fish` instead of POSIX `sh`?"
Well, I *could* have written Fishtank in POSIX `sh` - in the same sense that I *could* stick a fork into my eye repeatedly.

More seriously - I started this as an excuse to learn some scripting after migrating from `bash` to `fish`, and it matured into something I thought others might find useful.

If you prefer a different shell, you can still use Fishtank! The scripts are self-contained and properly shebanged, so simply installing `fish` and placing them somewhere in your `$PATH` should work fine.

### How does this compare to Toolbx or Distrobox?
It depends!

I used to heavily rely on Toolbx for my development environments, and I also dabbled with Distrobox. Both are excellent tools, but I have one big gripe with both: host integration.

- Toolbx automatically runs as `--privileged` with (among other things) your entire `$HOME` and `$XDG_RUNTIME_DIR` mounted into the container, and offers no way to opt-out.
- Distrobox is similar, but does offer some opt-outs. You can also choose to use an alternate `$HOME` on the host (not inside the container.)

Fishtank, by contrast, is entirely opt-in when it comes to host integrations. You get to choose precisely what (if anything) is shared.

Fishtank also offers a custom container definition format ("tanks") that bundle together information on how to build *and* run the container. 

This works by baking information you would normally see in a `docker-compose` file or Kubernetes YAML (mounts, security options...) into the image as OCI annotations, which are then read back and applied during container creation.

