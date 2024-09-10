# NAME
```
tankctl build
```

# DESCRIPTION
`build` compiles container definitions (either Containerfiles or harnessed `fish` scripts) into ready-to-run container images. 

By default, any new or changed definition will be built, but specific definition names can be passed to only build those definitions.

# SYNOPSIS
**tankctl build**

```
-f/--force: build definitions regardless of whether or not they have changed
```

# EXAMPLES
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