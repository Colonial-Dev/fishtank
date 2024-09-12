% FISHTANK(1) Version 1.0 | User Manual

NAME
====
**tankctl-create** - create a new container definition, or a symbolic link to an existing one.

SYNOPSIS
========

| **tankctl create** \[**-l**|**--link-to** *definition*\] *name*
| **tankctl create** \[**-h**|**--help**\]

DESCRIPTION
===========

`create` creates a new file or symbolic link for a container definition in `$XDG_CONFIG_HOME/fishtank/` (typically `$HOME/.config/fishtank/`) and opens it for editing using `$EDITOR`.

All edits are performed in a temporary file; upon saving and exiting, `tankctl` will perform a syntax validity check before writing any changes back to the original file.

Options
-------

-l, --link-to *definition*

: Build images even if the definition has not changed.

-h, --help

:  Displays this manual page.

EXAMPLES
========

Create a new definition called `cpp`:

```
tankctl create cpp
```

Create a new definition called `cpp` that symlinks to an existing definition called `c`:

```
tankctl create --link-to c cpp
```

SEE ALSO
========

**tankcfg(1)**, **podman(1)**, **buildah(1)**