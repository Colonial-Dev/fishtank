# Start one or more specified container(s),
# or all Fishtank-managed containers if no arguments are provided.
function tankctl_start
    for_each check_ctr $argv

    if [ (count $argv) -ne 0 ]
        map start_ctr $argv
    else
        map start_ctr (enumerate_containers)
    end
end

# Restart one or more specified container(s),
# or all Fishtank-managed containers if no arguments are provided.
function tankctl_restart
    for_each check_ctr $argv

    if [ (count $argv) -ne 0 ]
        map restart_ctr $argv
    else
        map restart_ctr (enumerate_containers)
    end
end

# Stop one or more specified container(s),
# or all Fishtank-managed containers if no arguments are provided.
function tankctl_stop
    for_each check_ctr $argv

    if [ (count $argv) -ne 0 ]
        map stop_ctr $argv
    else
        map stop_ctr (enumerate_containers)
    end
end

# Remove one or more specified container(s).
function tankctl_down
    for_each check_ctr $argv

    if [ (count $argv) -eq 0 ]
        abort "no container names or IDs provided"
    end

    map rm_ctr $argv
end

# Create one or more specified containers from their respective images.
function tankctl_up

end

function tankctl_list

end

function tankctl_edit

end

function tankctl_create

end

# Attempts to execute the provided command inside
# the specified container.
function tankctl_exec -a container command
    # Check if provided container exists
    # Check if we manage it, warn and ask for confirmation otherwise
    # Start if needed
end

# Attempts to execute $SHELL inside the provided container.
#
# Note that the value of $SHELL *inside* the container is used,
# *not* the value on the host.
function tankctl_enter -a container
    # Check if provided container exists
    # Check if we manage it, warn and ask for confirmation otherwise
    # Start if needed
    # exec /bin/sh -c "exec \$SHELL"
end

# --- EFFECTIVE ENTRYPOINT --- #

require podman
require buildah

trap rm cp mv ls ln mkdir podman buildah
trap curl realpath find md5sum fish

if [ -n "$XDG_CONFIG_HOME" ]
    set -x __TANK_DIR "$XDG_CONFIG_HOME/fishtank"
else
    set -x __TANK_DIR "$HOME/.config/fishtank"
end

mkdir -p $__TANK_DIR

if [ -z "$argv[1]" ]
    tankctl_help
    abort "no subcommand specified"
else if not functions -q "tankctl_$argv[1]"
    tankctl_help
    abort "unknown subcommand '$argv[1]'"
else
    tankctl_$argv[1] $argv[2..]
end
