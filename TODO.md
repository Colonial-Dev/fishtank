### Commands:
- Up (recreate, rebuild)
- Down 
- Start
- Restart
- Stop
- List
- Build
- Create (link-to, name)
    - By default, creates a new container definition with the specified name and opens it in `$EDITOR`.
    - With `--link-to <name>`, rather than creating a new file, a symbolic link is created to the existing definition specified by `--link-to`. 
- Exec (container, command)
- Enter (container)
- Install
    - Used for self-installation and update via `curl`, plus injecting certain extra functionality (such as an autostart `systemd` unit.)

1. Where applicable, commands operate on all tanks by default, and can be provided a list of tank(s) to operate on specifically.
2. Fishtank identifies containers by looking for the annotation `manager=fishtank`. 
    - (This is not exactly standard, but `distrobox` does it, which is good enough for me.)
3. 

### Annotations
Except for `manager`, all annotations are prefixed with `fishtank`.

These are always set:
- `manager` (only valid value is `fishtank`)
- `definition` (path to the tank definition file used to build the container)
- `hash` (MD5 hash of the above file, used to identify if anything has changed since build)
- `name` (can be set by the user, but a default is derived from the filename)

These are set by the user using `tankcfg`:
- `args` (additional `podman create` arguments)
- `mounts` (volumes, binds, and such using the newer syntax)
- `security_opts`
- `cap_add`/`cap_drop`
- `userns`
- `cpu`/`ram`/`ulimit`
- `autostart`

### Directives
These go *inside* tank definitions.

Format:
```sh
# fishtank directive-name directive-arguments
```

Recognzied directives:
- `containerfile` - indicates that this definition is a `Containerfile` rather than a `buildah` script.
- `unshare` - indicates that the script should be executed in a user namespace using `buildah unshare` (needed to create mounts w/o root.)
- `build-priority [INTEGER]` - supports 0-65535, lower values are built first where applicable. 

The first instance of a directive is always the one used. Later instances, if any, are ignored.

### List Format
```shell
Name          Image          Status          Up to Date?
```