% FISHTANK(1) Version 1.0 | User Manual

NAME
====
**tankctl-edit** - edit an existing container definition using **$EDITOR**.

SYNOPSIS
========

| **tankctl edit** *definition*
| **tankctl edit** \[**-h**|**--help**\]

DESCRIPTION
===========

`edit` opens an existing definition for modification using `$EDITOR`.

All edits are performed in a temporary file; upon saving and exiting, `tankctl` will perform a syntax validity check before writing any changes back to the original file.

Options
-------

-h, --help

:  Displays this manual page.

EXAMPLES
========

Edit a definition called `cpp`:

```
tankctl edit cpp
```

SEE ALSO
========

**tankcfg(1)**, **podman(1)**, **buildah(1)**