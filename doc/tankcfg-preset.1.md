% FISHTANK(1) Version 1.0 | User Manual

NAME
====
**tankcfg-preset** - apply a prewritten preset to a container.

SYNOPSIS
========

| **tankcfg preset** *preset* *args* ...

DESCRIPTION
===========

`preset` applies a prewritten preset to a container under construction, such as copying in a user from the host or fixing bind mounts under SELinux.

Available Presets
-----------------

cp-user \[*username*\]

: Copies the specified user from the host into the container, or the current user if none is specified. The created user will have `sudo` privileges.

bind-fix

: Modifies the security options and `userns` mode of the container to allow rootless bind mounts to work under SELinux.

ssh-agent

: Mounts the host SSH agent into the container. Implies `bind-fix`.

dbus

: Mounts the host DBUS socket into the container. Implies `bind-fix`.

SEE ALSO
========

**tankcfg(1)**, **podman(1)**, **buildah(1)**