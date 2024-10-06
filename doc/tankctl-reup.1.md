% FISHTANK(1) Version 1.0 | User Manual

NAME
====
**tankctl-reup** - remove and recreate one or more managed containers.

SYNOPSIS
========

| **tankctl reup** \[**-a**|**--all**\] *container* ...
| **tankctl reup** \[**-h**|**--help**\]

DESCRIPTION
===========

`reup` removes and recreates containers managed by Fishtank. By default, you must explicitly pass the container(s) you wish to recreate; the `-a/--all` flag can be used to override this behavior.

Options
-------

-a, --all

: Restart and recreate all managed containers.

-h, --help

: Displays this manual page.

EXAMPLES
========

Restart and recreate a container called `cpp`:

```
tankctl reup cpp
```

Restart and recreate all managed containers:

```
tankctl reup --all
```

SEE ALSO
========

**tankcfg(1)**, **podman(1)**, **buildah(1)**
