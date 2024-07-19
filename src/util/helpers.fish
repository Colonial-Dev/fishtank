set FT_MANAGER fishtank

# Wrapper around printf that emits to standard error.
function eprintf
    printf "%s: " (status basename | string split '.')[1]
    printf $argv >&2
end

# Wrapper around printf that emits to standard error only if a flag is set.
function vprintf
    if [ -n "$fish_verbose" ]
        printf $argv >&2
    end
end

function abort
    eprintf $argv
    exit 1
end

# Checks if the provided command exists in $PATH,
# aborting with an error message if not.
function require -a command
    if not type -P "$command" >/dev/null 2>&1
        eprintf "runtime dependency '$command' not found in \$PATH\n"
        exit 1
    end
end
