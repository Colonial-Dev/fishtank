# Definition Reference

## Metadata
All definitions can contain metadata as TOML key-value pairs with a special prefix:

```
#~ key = value
```

Metadata can be placed anywhere in the file. When Box evaluates a definition, each line of metadata is extracted and concatenated into a single TOML document; any intervening lines are ignored.

Currently, only two keys are recognized:
- `depends_on` (`[string]`) - a list of definition names that this definition depends on. Defaults to empty.

## Commands

Box provides (approximate) implementations of all OCI Containerfile operations as shell functions, as well as several additional tools.

### `FROM`
> *Corresponding manual page: `buildah from`*

`FROM` creates a new working container from a specified image. A `FROM` command is _required_ to precede all other operations.

```sh
FROM fedora-toolbox:latest
```

In addition to an image name, any options from the corresponding manual page can be used:

```sh
# Check if our local copy of the image is up to date,
# pulling the newer version if not.
FROM --pull=newer fedora-toolbox:latest
```

### `RUN`
> *Corresponding manual page: `buildah run`*

`RUN` executes a command inside the working container created by `FROM`:

```sh
# Most package managers will need their 'assume yes' flag
# to be set!
RUN dnf install -y gcc
```

In addition to a command, any options from the corresponding manual page can be used. You must separate any options from the command with `--` to avoid parsing ambiguity:

```sh
# This would fail due to parsing ambiguity if the '--' was removed!
RUN -v $HOME/.cache/dnf:/var/cache/dnf:z -- dnf install -y gcc
```

`RUN`'s behavior differs slightly from its OCI Containerfile equivalent when it comes to shell syntax like pipes and redirections.

This is valid `RUN` syntax in a Containerfile, but not in a Box definition:

```dockerfile
RUN echo "foo" > bar
```

In order to disambiguate, you should wrap the entire operation in a `$SHELL -c`:

```sh
RUN sh -c "echo \"foo\" > bar"
```

### `ADD` and `COPY`
> *Corresponding manual page: `buildah add`*

`ADD` and `COPY` (which defers to `ADD`) copies the contents of a file, directory, or URL into the working container created by `FROM`:

```sh
ADD $HOME/.gitconfig /home/$USER/.gitconfig
```

In addition to path(s), any options from the corresponding manual page can be used. You must separate any options from the command with `--` to avoid parsing ambiguity:

```sh
ADD --chown $USER:$USER -- $HOME/.gitconfig /home/$USER/.gitconfig
```

### Other OCI Containerfile Operations

Besides the above, Box also polyfills most other directives from the [OCI Containerfile](https://docs.docker.com/reference/dockerfile/) (aka Dockerfile) reference, including:

- `CMD`
- `ENTRYPOINT`
- `ENV`
- `EXPOSE`
  - Note that this does *not* alter any runtime behavior - it only applies metadata. Use `CFG args -p=xxxx:yyyy` to ensure ports are actually forwarded at runtime.
- `HEALTHCHECK`
- `LABEL`
- `SHELL`
- `STOPSIGNAL`
- `USER`
- `VOLUME`
- `WORKDIR`

### `CFG`

`CFG` is a catch-all tool used to bake runtime arguments (such as mounts)[^1] into an image:

```sh
CFG <FUNCTION> [ARGS...]
```

| Name | Function | More Info |
| ---- | -------- | --------- |
| `args` | Additional arbitrary arguments to pass to `podman run`. | Self-explanatory. |
| `cap-add` | Add a Linux capability to the container. | See `man capabilities`. |
| `cap-drop` | Remove a Linux capability from the container. | See `man capabilities`. |
| `cpus` | Number of CPUs the container can utilize. | Self-explanatory. |
| `memory` | Container memory limit. | Supports `b`, `kb`, `mb`, and `gb` as suffixes. |
| `ulimit` | Set the `ulimit` parameters for the container. | See `man ulimit`. |
| `device` | Add a host device to the container. Uses `--volume` syntax. | Self-explanatory. |
| `userns` | Set the user namespace mode for the container. | [Podman docs](https://docs.podman.io/en/v4.4/markdown/options/userns.container.html) |
| `security-opt` | Set a security option for the container. | [Podman docs](https://docs.podman.io/en/v4.6.1/markdown/options/security-opt.html) |
| `mount` | Add a mount (`bind` or otherwise) to the container. Uses `--mount` syntax. | [Podman docs](https://docs.podman.io/en/v5.1.0/markdown/podman-create.1.html#mount-type-type-type-specific-option) |
| `restart` | Set the container restart policy. | [Podman docs](https://docs.podman.io/en/v5.1.0/markdown/podman-create.1.html#restart-policy)
| `secret` | Give the container access to a secret. | [Podman docs](https://docs.podman.io/en/v5.1.0/markdown/podman-create.1.html#secret-secret-opt-opt) |

### `PRESET`

`PRESET` provides several "micro scripts" to take care of common operations:

```sh
PRESET <NAME> [ARGS...]
```

| Name | Function | Arguments |
| ---- | -------- | --------- |
| `cp-user` | Copies a user from the host into the container. | Optionally takes the name of the user to copy. If one is not provided, it defaults to the user executing the program (i.e. the result of `whoami`.) |
| `bind-fix` | Fixes permission issues encountered with rootless bind mounts on SELinux systems. Disables SELinux label separation and maps the host user to the same UID inside the container. | None. |
| `ssh-agent` | Mounts and exports `SSH_AUTH_SOCK` into the container. | None. |
| `devices` | Mounts `/dev` into the container. Implies `--privileged`! | None. |

### `COMMIT`
> *Corresponding manual page: `buildah commit`*

`COMMIT` is the inverse of `FROM` - it takes the working container and commits it as an image:

```sh
COMMIT image_name
```

In addition to an image name, any options from the corresponding manual page can be used. You must separate any options from the command with `--` to avoid parsing ambiguity:

```sh
COMMIT --rm -- image_name
```

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

[^1]: If you're wondering "how the hell does it do that" - it saves them as OCI annotations that are read back at creation time. <br> [Did you know you can just use the ASCII separator characters to separate things?](https://github.com/Colonial-Dev/box/blob/0c45cfe2c51a4ff1c3f62b3f753bcfeab882a56b/src/podman.rs#L341-L352) They're right there. Nobody can stop you.
