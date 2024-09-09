# Respect NO_COLOR environment variable convention by overriding the set_color
# function with a no-op.
if [ -n "$NO_COLOR" ]
    function set_color
        printf ""
    end
end

# Wrapper around printf that emits to standard error.
function eprintf
    printf "%s: " (status basename | string split '.')[1]
    printf $argv >&2
    printf "\n" >&2
end

# Wrapper around printf that emits to standard error only if a flag is set.
function vprintf
    if [ -n "$fish_verbose" ]
        printf $argv >&2
        printf "\n" >&2
    end
end

# Emit the provided message to standard error,
# then exit with code 1.
function abort
    eprintf $argv
    exit 1
end

# Checks if the provided command exists in $PATH,
# aborting with an error message if not.
function require -a command
    if not command -q "$command"
        eprintf "runtime dependency '$command' not found in \$PATH"
        exit 1
    end
end

# Prompt the user to confirm an action.
function confirm -a message
    while true
        read -l -P "$message [y/N] " confirm

        switch $confirm
            case Y y
                return 0
            case '' N n
                return 1
        end
    end
end

# Compute the MD5 hash of the provided file, stripping the pointless
# repetition of the path from the output.
function md5 -a path
    md5sum $path | awk '{ print $1 }'
end

# Apply the provided function to every item in the argument vector.
function map -a fn
    for item in $argv[2..]
        $fn $item
    end
end

# Apply the provided function to every item in the argument vector.
# This version surpresses standard output from the call. (Standard error is untouched.)
function for_each -a fn
    for item in $argv[2..]
        $fn $item >/dev/null
    end
end

# Check if all items in the argument vector return true when passed to the provided function.
function all -a fn
    for item in $argv[2..]
        if not $fn $item >/dev/null 2>&1
            return 1
        end
    end

    return 0
end

# Check if any item in the argument vector returns true when passed to the provided function.
function any -a fn
    for item in $argv[2..]
        if $fn $item >/dev/null 2>&1
            return 0
        end
    end

    return 1
end

set __CONFIG_FLAGS \
    entrypoint \
    env \
    healthcheck \
    hostname \
    port \
    shell \
    user \
    volume \
    workingdir

set __ANNOTATIONS \
    args \
    cap-add \
    cap-drop \
    cpus \
    ram \
    ulimit \
    device \
    userns \
    security-opt \
    mount \
    restart \
    secret
