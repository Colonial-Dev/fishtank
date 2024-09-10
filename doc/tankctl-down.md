# NAME
```
tankctl down
```

# DESCRIPTION
`down` stops and removes containers managed by Fishtank. By default, you must explicitly pass the container(s) you wish to remove; the `-a/--all` flag can be used to override this behavior.

# SYNOPSIS
**tankctl down**

```
-a/--all: remove all managed containers
```

# EXAMPLES
Stop and remove a container called `cpp`:

```
tankctl down cpp
```

Stop and remove all managed containers:
```
tankctl down --all
```