set FT_MANAGER fishtank

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
    if not type -P "$command" >/dev/null 2>&1
        eprintf "runtime dependency '$command' not found in \$PATH"
        exit 1
    end
end

# Check if the provided container exists.
function ctr_exists -a ctr
    # 'command' bypasses the exit code trap set during script initialization.
    return (command podman container exists $ctr)
end

# Check if the provided container is started.
function ctr_started -a ctr
    set -l status (podman inspect $ctr --format "{{ .State.Status }}")

    if [ "$status" != running ]
        return 1
    else
        return 0
    end
end

# Look up, by key, an annotation on the provided container.
function ctr_annotation -a ctr key
    echo (podman inspect $ctr --format "{{index .Config.Annotations \"$key\"}}")
end

# Enumerate all containers (stopped or otherwise) managed by Fishtank.
function enumerate_ctrs
    for id in (podman ps -a --format "{{.ID}}")
        # The 'manager' annotation is not standardized, but distrobox uses it,
        # which is good enough for me.
        set -l manager (ctr_annotation $id "manager")

        # fish splits on newlines by default, so directly echoing
        # the container IDs means that callers will automatically
        # capture the output as a list.
        if [ "$manager" == $FT_MANAGER ]
            echo $id
        end
    end
end

function check_ctr -a ctr
    if not ctr_exists $ctr
        abort "container '$ctr' does not exist"
    end

    if [ (ctr_annotation $ctr "manager") != $FT_MANAGER ]
        # TODO: ask for user confirmation instead
        abort "container '$ctr' is not managed by fishtank"
    end

    if not ctr_started $ctr
        vprintf "container '$ctr' is stopped, starting..."
        podman start $ctr
    end
end
