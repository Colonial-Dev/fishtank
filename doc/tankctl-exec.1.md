% FISHTANK(1) Version 1.0 | User Manual

NAME
====
**tankctl-exec** - execute a command inside the specified container.

SYNOPSIS
========

| **tankctl exec** *container* *command* ...
| **tankctl exec** \[**-h**|**--help**\]

DESCRIPTION
===========

`exec` executes an arbitrary command inside the provided container.

Options
-------

-h, --help

:  Displays this manual page.

EXAMPLES
========

Get the contents of `/etc/os-release` inside a container called `cpp`:

```
tankctl exec cpp cat /etc/os-release
```

SEE ALSO
========

**tankcfg(1)**, **podman(1)**, **buildah(1)**