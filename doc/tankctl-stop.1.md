% FISHTANK(1) Version 1.0 | User Manual

NAME
====
**tankctl-stop** - stop one or more managed containers.

SYNOPSIS
========

| **tankctl stop** \[**-a**|**--all**\] *container* ...
| **tankctl stop** \[**-h**|**--help**\]

DESCRIPTION
===========

`stop` stops containers managed by Fishtank. By default, you must explicitly pass the container(s) you wish to stop; the `-a/--all` flag can be used to override this behavior.

Options
-------

-a, --all

: Stop all managed containers.

-h, --help

: Displays this manual page.

EXAMPLES
========

Stop a container called `cpp`:

```
tankctl stop cpp
```

Stop all managed containers:

```
tankctl stop --all
```

SEE ALSO
========

**tankcfg(1)**, **podman(1)**, **buildah(1)**
