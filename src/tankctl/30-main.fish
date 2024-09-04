set -a options (fish_opt -s a -l all)

# Start one or more specified container(s).
function tankctl_start
    argparse -i $options -- $argv

    if set -q _flag_all
        map start_ctr (enumerate_ctrs)
        return
    end

    for_each check_ctr $argv

    if [ (count $argv) -ne 0 ]
        map start_ctr $argv
    else
        abort "no container names or IDs provided"
    end
end

# Restart one or more specified container(s).
function tankctl_restart
    argparse -i $options -- $argv

    if set -q _flag_all
        map restart_ctr (enumerate_ctrs)
        return
    end

    for_each check_ctr $argv

    if [ (count $argv) -ne 0 ]
        map restart_ctr $argv
    else
        abort "no container names or IDs provided"
    end
end

# Stop one or more specified container(s),
# or all Fishtank-managed containers if no arguments are provided.
function tankctl_stop
    argparse -i $options -- $argv

    if set -q _flag_all
        map stop_ctr (enumerate_ctrs)
        return
    end

    for_each check_ctr $argv

    if [ (count $argv) -ne 0 ]
        map stop_ctr $argv
    else
        abort "no container names or IDs provided"
    end
end

# Remove one or more specified container(s).
function tankctl_down
    argparse -i $options -- $argv

    if set -q _flag_all
        map rm_ctr (enumerate_ctrs)
        return
    end

    for_each check_ctr $argv

    if [ (count $argv) -eq 0 ]
        abort "no container names or IDs provided"
    end

    map rm_ctr $argv
end

# Create one or more specified containers from their respective images.
function tankctl_up
    for img in (enumerate_imgs)
        if podman ps --format "{{.ImageID}}" | grep -q $img
            # Ignore unless --replace is set
        else
            make_ctr $img
        end
    end
end

function tankctl_list

end

function tankctl_edit
    set -l tmpfile (mktemp)
    cp (locate_def $argv[1]) $tmpfile

    while true
        $EDITOR $tmpfile

        if not command fish -n $tmpfile
            if confirm "definition contains syntax errors; would you like to resume editing?"
                continue
            else
                return
            end
        end

        break
    end

    cp $tmpfile (locate_def $argv[1])
end

function tankctl_create
    argparse -i (fish_opt -r -s l -l link-to) -- $argv

    if set -q _flag_l
        set dst (locate_def $_flag_l)
        set src $argv

        if [ -f "$dst" ]
            ln -s (dirname $dst)/$_flag_l.tank "$__TANK_DIR/$src.tank"
        else
            abort "cannot link to non-existent tank definition $_flag_l"
        end
    else
        touch $__TANK_DIR/$argv.tank
    end

    tankctl_edit $argv[1]
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

trap rm cp mv ls ln mkdir podman
trap curl realpath find touch

if [ -n "$XDG_CONFIG_HOME" ]
    set -x __TANK_DIR "$XDG_CONFIG_HOME/fishtank/"
else
    set -x __TANK_DIR "$HOME/.config/fishtank/"
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
