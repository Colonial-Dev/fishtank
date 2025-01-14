# Shell-Based Definitions

## DSL Commands

Box provides (approximate) implementations of all OCI Containerfile operations as shell functions, as well as several additional tools.

### `FROM`

### `RUN`

### `ADD` and `COPY`

### Other OCI Containerfile Operations

### `CFG`

### `PRESET`

### `COMMIT`

## Environment Variables
Box sets the following environment variables when evaluating a definition. These are primarily an implementation detail
and should not be considered a stable interface, but knowlege of their presence may be helpful:

- `__BOX_BUILD_CTR` - the working container name. Set when `FROM` is called.
- `__BOX_BUILD_PATH` - the path to the definition.
- `__BOX_BUILD_DIR` - the path to the *directory* containing the definition.
- `__BOX_BUILD_NAME` - the name of the definition.
- `__BOX_BUILD_HASH` - the hash of the definition.
- `__BOX_BUILD_TREE` - the (somewhat poorly named) combined hash of the definition and all its dependencies.

## Functions

The harness for `fish`-based definitions includes a function called `trap` that can be used to emulate the POSIX `set -e`:

```sh
# Wraps 'cp' to exit the script on a non-zero exit code.
trap cp
```

This is not included in the POSIX harness, which automatically applies `set -eu` to abort on non-zero exit codes or uses of unset variables.