function ft_help
    printf "Good luck!\n"
end

function ctr_started -a ctr
    set -l status (podman inspect $ctr --format "{{ .State.Status }}")

    if [ "$status" != running ]
        return 1
    else
        return 0
    end
end

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
    # 'command' bypasses the exit code trap set during script initialization.
    if not command podman container exists $ctr
        abort "container '$ctr' does not exist\n"
    end

    if [ (ctr_annotation $ctr "manager") != $FT_MANAGER ]
        # TODO: ask for user confirmation instead
        abort "container '$ctr' is not managed by fishtank\n"
    end
end

function ft_up

end

function ft_down

end

# Start one or more specified container(s),
# or all Fishtank-managed containers if no arguments are provided.
function ft_start
    if [ (count $argv) -ne 0 ]
        for name in $argv
            podman start $id
        end
    else
        for id in (enumerate_containers)
            podman start $id
        end
    end
end

# Restart one or more specified container(s),
# or all Fishtank-managed containers if no arguments are provided.
function ft_restart
    if [ (count $argv) -ne 0 ]
        for name in $argv
            podman restart -t 0 $id
        end
    else
        for id in (enumerate_containers)
            podman restart -t 0 $id
        end
    end
end

# Stop one or more specified container(s),
# or all Fishtank-managed containers if no arguments are provided.
function ft_stop
    if [ (count $argv) -ne 0 ]
        for name in $argv
            podman stop -t 0 $id
        end
    else
        for id in (enumerate_containers)
            podman stop -t 0 $id
        end
    end
end

function ft_list

end

function ft_build

end

function ft_create

end

# Attempts to execute the provided command inside
# the specified container.
function ft_exec -a container command
    # Check if provided container exists
    # Check if we manage it, warn and ask for confirmation otherwise
    # Start if needed
end

# Attempts to execute $SHELL inside the provided container.
#
# Note that the value of $SHELL *inside* the container is used,
# *not* the value on the host.
function ft_enter -a container
    # Check if provided container exists
    # Check if we manage it, warn and ask for confirmation otherwise
    # Start if needed
    # exec /bin/sh -c "exec \$SHELL"
end

# --- EFFECTIVE ENTRYPOINT --- #

require podman

trap rm cp mv ls mkdir podman

if [ -n "$XDG_CONFIG_HOME" ]
    set -x tank_dir "$XDG_CONFIG_HOME/fishtank"
else
    set -x tank_dir "$HOME/.config/fishtank"
end

mkdir -p $tank_dir

if [ -z "$argv[1]" ]
    ft_help
    abort "no subcommand specified\n"
else if not functions -q "ft_$argv[1]"
    ft_help
    abort "unknown subcommand '$argv[1]'\n"
else
    eval (ft_$argv[1] $argv[2..])
end
