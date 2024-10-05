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

-a, --all

: Remove all managed containers.

-h, --help

:  Displays this manual page.

EXAMPLES
========

Stop and remove a container called `cpp`:

```
tankctl down cpp
```

Stop and remove all managed containers:
```
tankctl down --all
```

SEE ALSO
========

**tankcfg(1)**, **podman(1)**, **buildah(1)**