# NAME
```
tankctl edit
```

# DESCRIPTION
`edit` opens an existing definition for modification using `$EDITOR`.

All edits are performed in a temporary file; upon saving and exiting, `tankctl` will perform a syntax validity check before writing any changes back to the original file.

# EXAMPLES
Edit a definition called `cpp`:

```
tankctl edit cpp
```