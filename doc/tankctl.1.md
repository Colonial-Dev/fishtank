% FISHTANK(1) Version 1.0 | User Manual

NAME
====
**tankctl** - entrypoint for Fishtank, a simple interactive container manager

SYNOPSIS
========

| **tankctl** \[subcommand\]
| **tankctl** \[**-h**|**--help**|**-v**|**--version**\]

DESCRIPTION
===========

Defers to the provided subcommand for managing OCI images and containers. See below for the list of available subcommands; run **man tankctl \[subcommand\]** for detailed individual information.

Options
-------

-h, --help

:  Displays this manual page.

-v, --version

:  Displays the current version number.

Subcommands
-----------

build

: Compile Fishtank-managed container definitions into OCI images.

create

: Create a new container definition, or a symbolic link to an existing one.

down

: Stop and remove one or more managed containers.

edit

: Edit an existing container definition using **$EDITOR**.

enter

: Execute **$SHELL** inside a container.

exec

: Execute an arbitrary command inside a container.

list

: List all managed containers and their status.

restart 

: Restart one or more managed containers.

reup

: Remove and re-create one or more managed containers.

start

: Start one or more managed containers.

stop

: Stop one or more managed containers.

up

: Create managed containers from stored images.

GETTING STARTED
===============

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

Once you have a definition, run `tankctl build` to compile it into an OCI image, followed by `tankctl up` to create a container from the image.

Complete details on all commands can be found in the `man` pages bundled with Fishtank. Alternatively, you can view their Markdown versions [here](https://github.com/Colonial-Dev/fishtank/tree/master/doc).

SEE ALSO
========

**tankcfg(1)**, **podman(1)**, **buildah(1)**