set FT_MANAGER fishtank

# Enumerate all containers (stopped or otherwise) managed by Fishtank.
function enumerate_ctrs
    for id in (podman ps -a --format "{{.ID}}")
        # The 'manager' annotation is not standardized, but distrobox uses it,
        # which is good enough for me.
        set -l manager (ctr_annotation $id "manager")
        # fish splits on newlines by default, so directly echoing
        # the container IDs means that callers will automatically
        # capture the output as a list.
        if [ "$manager" = $FT_MANAGER ]
            echo $id
        end
    end
end

# Enumerate all images managed by Fishtank.
function enumerate_imgs
    for id in (podman image ls --format "{{.ID}}" --filter dangling=false)
        set -l manager (img_annotation $id "manager")

        if [ "$manager" = $FT_MANAGER ]
            echo $id
        end
    end
end

# Check if the provided container exists.
function ctr_exists -a ctr
    # 'command' bypasses the exit code trap set during script initialization.
    return (command podman container exists $ctr)
end

# Check if the provided container is started.
function ctr_started -a ctr
    set -l s (podman inspect -t container $ctr --format "{{ .State.Status }}")

    if [ "$s" != running ]
        return 1
    else
        return 0
    end
end

# Look up, by key, an annotation on the provided container.
function ctr_annotation -a ctr key
    podman inspect -t container $ctr --format "{{index .Config.Annotations \"$key\"}}"
end

# Look up, by key, an annotation on the provided image.
function img_annotation -a img key
    podman inspect -t image $img --format "{{index .Annotations \"$key\"}}"
end

# Check that the provided container exists, aborting if it does not.
function check_ctr -a ctr
    if not ctr_exists $ctr
        abort "container '$ctr' does not exist"
    end
end

function make_ctr -a img replace
    set -l name (img_annotation $img "fishtank.name")
    set -l command podman run -d

    for a in $__ANNOTATIONS
        if [ $a = args ]
            continue
        end

        for entry in (img_annotation $img "fishtank.$a" | string split \x1F)
            set -a command "--$a"
            set -a command $entry
        end
    end

    set -a command --name $name
    set -a command --hostname $name

    for entry in (img_annotation $img "fishtank.args" | string split \x1F)
        set -a command $entry
    end

    set -a command --annotation "manager=fishtank"
    set -a command --annotation "fishtank.name=$name"

    if set -q replace
        set -a command --replace
    end

    $command $img
    printf " (%s)\n" $name
end

function start_ctr -a ctr
    podman start $ctr
end

function restart_ctr -a ctr
    podman restart -t 0 $ctr
end

function stop_ctr -a ctr
    podman stop -t 0 $ctr
end

function rm_ctr -a ctr
    set -l name (ctr_annotation $ctr "fishtank.name")
    podman rm -ft 0 $ctr
    printf " (%s)\n" $name
end
