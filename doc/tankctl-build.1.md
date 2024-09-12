% FISHTANK(1) Version 1.0 | User Manual

NAME
====
**tankctl-build** - compile Fishtank-managed container definitions into OCI images.

SYNOPSIS
========

| **tankctl build** \[**-f**|**--force**\] *container* ...
| **tankctl build** \[**-h**|**--help**\]

DESCRIPTION
===========

`build` compiles container definitions (either Containerfiles or harnessed `fish` scripts) into ready-to-run container images. 

By default, any new or changed definition will be built, but specific definition names can be passed to only build those definitions.

Options
-------

-f, --force

: Specify an existing definition to symlink to rather than creating a new file

-h, --help

:  Displays this manual page.

EXAMPLES
========

Build all "out of date" definitions:
```
tankctl build
```

Build *all* definitions, no matter what:
```
tankctl build --force
```

Build only the definition called `cpp`:
```
tankctl build cpp
```

Build only the definition called `cpp`, even if it hasn't changed:
```
tankctl build --force cpp
```

SEE ALSO
========

**tankcfg(1)**, **podman(1)**, **buildah(1)**