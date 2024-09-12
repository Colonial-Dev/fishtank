% FISHTANK(1) Version 1.0 | User Manual

NAME
====
**tankctl-down** - stop and remove one or more managed containers.

SYNOPSIS
========

| **tankctl down** \[**-a**|**--all**\] *container* ...
| **tankctl down** \[**-h**|**--help**\]

DESCRIPTION
===========

`down` stops and removes containers managed by Fishtank. By default, you must explicitly pass the container(s) you wish to remove; the `-a/--all` flag can be used to override this behavior.

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