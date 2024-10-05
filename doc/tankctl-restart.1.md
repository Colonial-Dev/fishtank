% FISHTANK(1) Version 1.0 | User Manual

NAME
====
**tankctl-restart** - restart one or more managed containers.

SYNOPSIS
========

| **tankctl restart** \[**-a**|**--all**\] *container* ...
| **tankctl restart** \[**-h**|**--help**\]

DESCRIPTION
===========

`restart` restarts containers managed by Fishtank. By default, you must explicitly pass the container(s) you wish to remove; the `-a/--all` flag can be used to override this behavior.

Options
-------

-a, --all

: Remove all managed containers.

-h, --help

:  Displays this manual page.

EXAMPLES
========

Restart a container called `cpp`:

```
tankctl restart cpp
```

Restart all managed containers:
```
tankctl restart --all
```

SEE ALSO
========

**tankcfg(1)**, **podman(1)**, **buildah(1)**