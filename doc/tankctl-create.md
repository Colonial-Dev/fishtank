# NAME
```
tankctl create
```

# DESCRIPTION
`create` creates a new file or symbolic link for a container definition in `$XDG_CONFIG_HOME/fishtank/` (typically `$HOME/.config/fishtank/`) and opens it for editing using `$EDITOR`.

All edits are performed in a temporary file; upon saving and exiting, `tankctl` will perform a syntax validity check before writing any changes back to the original file.

# SYNOPSIS
**tankctl create**

```
-l/--link-to: specify an existing definition to symlink to rather than creating a new file
```

# EXAMPLES
Create a new definition called `cpp`:

```
tankctl create cpp
```

Create a new definition called `cpp` that symlinks to an existing definition called `c`:

```
tankctl create --link-to c cpp
```