function ft_help
    set -l nrm (set_color normal)
    set -l uln (set_color -ou)
    set -l bld (set_color -o)

    printf "An interactive container manager for the fish shell."
    printf "\n\n"

    printf "%sUsage:%s " $uln $nrm
    printf "%s%s%s <COMMAND>" $bld (status basename | string split '.')[1] $nrm
    printf "\n\n"

    printf "%sCommands:%s\n" $uln $nrm
    printf "\n\n"
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

function ft_rm
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
    abort "no subcommand specified"
else if not functions -q "ft_$argv[1]"
    ft_help
    abort "unknown subcommand '$argv[1]'"
else
    eval (ft_$argv[1] $argv[2..])
end
