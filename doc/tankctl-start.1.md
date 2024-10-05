% FISHTANK(1) Version 1.0 | User Manual

NAME
====
**tankctl-start** - start one or more managed containers.

SYNOPSIS
========

| **tankctl start** \[**-a**|**--all**\] *container* ...
| **tankctl start** \[**-h**|**--help**\]

DESCRIPTION
===========

`start` starts containers managed by Fishtank. By default, you must explicitly pass the container(s) you wish to start; the `-a/--all` flag can be used to override this behavior.

Options
-------

-a, --all

: Start all managed containers.

-h, --help

: Displays this manual page.

EXAMPLES
========

Start a container called `cpp`:

```
tankctl start cpp
```

Start all managed containers:

```
tankctl start --all
```

SEE ALSO
========

**tankcfg(1)**, **podman(1)**, **buildah(1)**
