% FISHTANK(1) Version 1.0 | User Manual

NAME
====
**tankctl-up** - create one or more managed containers from a Fishtank-compiled image.

SYNOPSIS
========

| **tankctl up** \[*definition* ...\]
| **tankctl up** \[**-h**|**--help**\]

DESCRIPTION
===========

*up* creates one or more managed containers from a Fishtank-compiled image. With no additional arguments passed, *up* will try to create a container from all definitions that do not yet have an associated container.

If a container already exists, *up* will abort; to instead remove and replace existing containers, use the `-r/--replace` flag.

Options
-------

-r, --replace

: If a container already exists, remove and replace it instead of aborting.

-h, --help

:  Displays this manual page.

EXAMPLES
========

Create containers from all definitions that do not yet have an associated container:

```
tankctl up
```

Create a container from a definition called `cpp`:

```
tankctl up cpp
```

SEE ALSO
========

**tankcfg(1)**, **podman(1)**, **buildah(1)**