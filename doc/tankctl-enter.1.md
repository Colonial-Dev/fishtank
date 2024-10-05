% FISHTANK(1) Version 1.0 | User Manual

NAME
====
**tankctl-enter** - execute `$SHELL` inside the specified container.

SYNOPSIS
========

| **tankctl enter** *container*
| **tankctl enter** \[**-h**|**--help**\]

DESCRIPTION
===========

`enter` executes `$SHELL` inside the specified container.

Note that the value of `$SHELL` *inside* the container is used, not the value on the host.

Options
-------

-h, --help

:  Displays this manual page.

EXAMPLES
========

Enter a container called `cpp`:

```
tankctl enter cpp
```

SEE ALSO
========

**tankcfg(1)**, **podman(1)**, **buildah(1)**