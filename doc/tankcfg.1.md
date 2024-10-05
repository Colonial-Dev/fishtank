% FISHTANK(1) Version 1.0 | User Manual

NAME
====
**tankcfg** - a companion script to **tankctl(1)**, used to apply build-time and run-time options during a container build.

SYNOPSIS
========

| **tankcfg** \[subcommand\]

DESCRIPTION
===========

Defers to the provided subcommand. See below for the list of available subcommands. Except for **preset**, all subcommands correspond to an argument to **podman-run(1)**.

Note that **tankcfg** will refuse to run outside the context of a Fishtank container build.

Subcommands
-----------

args

: Provide additional arbitrary arguments to `podman run`.

cap-add

: Add a runtime Linux capability to the definition.

cap-drop

: Drop a runtime Linux capability from the definition.

cpus

: Provide the number of CPUs the container will be allowed to use at runtime.

device

: Provide a device (such as `/dev/dri`) that the container will be allowed access to at runtime.

env

: Provide an environment variable to be set inside the container.

entrypoint

: Set the runtime entrypoint for the container.

healthcheck

: Set the runtime healthcheck command for the container.

hostname

: Set the runtime hostname for the container.

mount

: Provide a mount (in the form `type=$,src=$,dst=$`) to apply to the container at runtime.

port

: Provide a port to forward from the container to the host.

preset

: Apply a preset. See **tankcfg-preset(1)** for available options.

memory

: Provide the maximum amount of memory the container will be allowed to use at runtime, in the form NNNNb/k/m/g.

restart

: Provide the restart policy for the container.

secret

: Provide a secret that the container should be given access to at runtime.

security-opt

: Provide a security option to apply to the container at runtime.

shell

: Provide the default shell for the container.

ulimit

: Provide a `ulimit` rule to apply to the container at runtime.

user

: Provide the default user for the container.

volume

: Provide a bind mount (in the form source:destination:options) to apply to the container at runtime.

workingdir

: Provide the default working directory for the container.

userns

: Provide the `userns` mode to apply to the container at runtime.

SEE ALSO
========

**tankctl(1)**, **podman(1)**, **buildah(1)**