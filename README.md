<h1 align="center">Fishtank</h1>
<h3 align="center">An interactive container manager for the <code>fish</code> shell.</h3>

<p align="center">
<img src="https://img.shields.io/github/actions/workflow/status/Colonial-Dev/fishtank/fish.yml">
<img src="https://img.shields.io/github/license/Colonial-Dev/fishtank">
<img src="https://img.shields.io/github/stars/Colonial-Dev/fishtank">
</p>

## Features
Easily create and manage container environments for interactive use. All host integration is strictly opt-in; you choose what (if anything) is shared with each container.

<p align="center">
    <img src=".github/demo.gif">
</p>

Bring your existing Docker-style container definitions...

<p align="center">
    <img src=".github/README_B.png">
</p>

... or take advantage of Fishtank's custom shell-based format that bundles together all the information needed to build *and* run your containers.

<p align="center">
    <img src=".github/README_A.png">
</p>

Lightweight[^1], easy to install, and works on any[^2] Linux machine with `podman` and `fish`,

<p align="center">
    <img src=".github/demo_install.gif">
</p>

## Installation
Before installing, make sure you have the following packages on your system:
- `fish`
- `podman`
- `coreutils`

```sh
curl -Lf https://github.com/Colonial-Dev/fishtank/releases/latest/download/tankctl | source - install
```

This downloads the latest stable version of Fishtank and uses the self-update functionality to bootstrap a persistent install.

By default, this installs two scripts (`tankctl` and `tankcfg`) in the XDG-specified `$HOME/.local/bin` directory - to override the install location, simply pass your preferred path as the third argument:

```sh
curl -Lf https://github.com/Colonial-Dev/fishtank/releases/latest/download/tankctl | source - install /usr/bin
```

### From Source
If you are allergic to `curl | exec`, "building" Fishtank from source using the bundled `do` script is also possible.

```sh
git clone https://github.com/Colonial-Dev/fishtank && cd fishtank
./do install
```

## Getting Started

Fishtank requires a definition ("tank") for each container you'd like to create. Definitions can be in two different formats:
- Standard Container/Dockerfiles - just add `# fishtank containerfile` to the text and you're good to go.
- `fish` shell scripts that run in a special harness. This injects additional functions and wraps a few others to provide additional functionality not present in Containerfiles, like the ability to declare runtime arguments such as mounts.

Either type must be stored under `$XDG_CONFIG_HOME/fishtank/` (typically `~/.config/fishtank/`) with the file extension `.tank`.

To create and edit a new definition, you can simply run `tankctl create <NAME>`. This will create the file and open it using your `$EDITOR`.

`tankctl edit <NAME>` can be used to alter existing definitions; both commands will use a temporary file for editing and perform syntax checks before finalizing any changes.

Shell-based definitions run in the same directory as the definition, and should look something like this:

```sh
# Create a new working container.
set ctr (buildah from fedora-toolbox:latest)

# Set up the new container...
RUN dnf install gcc

# Commit the configured container as an image.
buildah commit $ctr toolbox
```

The harness for shell-based definitions enables two primary toolkits for setting up your container.
- All Containerfile directives like `RUN` and `ADD` are polyfilled as Fish functions, and generally act the same as their real counterparts. 
  - (The most notable exception is pipes and redirections in `RUN` - you must wrap them in an `sh -c` to execute them wholly inside the working container.)
- The `tankcfg` script, which lets you:
  - Set various build-time (some of which are duplicated from the above) and runtime switches
  - Provide arbitrary additional arguments to pass to `podman run`
  - Apply several prepackaged presets (such as copying a user from the host into the container, or applying security options to fix bind mounts with SELinux)

Click to show all `tankcfg` options:

```sh
tankcfg entrypoint
tankcfg env
tankcfg healthcheck
tankcfg hostname
tankcfg port
tankcfg shell
tankcfg user
tankcfg volume
tankcfg workingdir

tankcfg args
tankcfg cap-add
tankcfg cap-drop
tankcfg cpus
tankcfg ram
tankcfg ulimit
tankcfg device
tankcfg userns
tankcfg security-opt
tankcfg mount
tankcfg restart
tankcfg secret

tankcfp preset cp-user
tankcfg preset bind-fix
tankcfg preset ssh-agent
```

## FAQ

### "Why `fish` instead of POSIX `sh`?"
Well, I *could* have written Fishtank in POSIX `sh` - in the same sense that I *could* stick a fork into my eye repeatedly.

More seriously - I started this as an excuse to learn some scripting after migrating from `bash` to `fish`, and it matured into something I thought others might find useful.

If you prefer a different shell, you can still use Fishtank! The scripts are self-contained and properly shebanged, so simply installing `fish` and placing them somewhere in your `$PATH` should work fine.

### "How does this compare to Toolbx or Distrobox?"
It depends!

I used to heavily rely on Toolbx for my development environments, and I also dabbled with Distrobox. Both are excellent tools, but I have one big gripe with both: host integration.

- Toolbx automatically runs as `--privileged` with (among other things) your entire `$HOME` and `$XDG_RUNTIME_DIR` mounted into the container, and offers no way to opt-out.
- Distrobox is similar, but does offer some opt-outs. You can also choose to use an alternate `$HOME` on the host (not inside the container.)

Fishtank, by contrast, is entirely opt-in when it comes to host integrations. You get to choose precisely what (if anything) is shared.

Fishtank also requires that every container be associated with a "definition," rather than defaulting to a standard "toolbox" image for each container. These can either be standard Containerfiles, or they can use Fishtank's custom shell-based format to declare runtime arguments (like mounts)[^3] during build time.

So:
- If you don't mind the above caveats and want containerized environments that Just Work with the host, use Toolbx or Distrobox.
- If you *do* mind the above caveats and/or want some declarative-ness in your containers, give Fishtank a try.

[^1]: Only ~1000 lines of pure Fish shell code.

[^2]: Fishtank was developed on a system that uses GNU Coreutils and GNU `libc` - if you find that Fishtank doesn't work with alternative implementations, please file an issue!

[^3]: If you are wondering how this works: Fishtank bakes the arguments you provide at build time into the image using OCI annotations, then reads them out and applies them when creating a container from the image.