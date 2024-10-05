function RUN
    # We can't use 'contains' here, as it consumes '--' when parsing arguments.
    for arg in $argv
        if [ $arg = -- ]
            set _has_opts yes
        end
    end

    if set -q _has_opts
        for arg in $argv
            if set -q _passed_opts
                set -a right_opts $arg
            else
                if [ $arg = -- ]
                    set _passed_opts yes
                    continue
                end

                set -a left_opts $arg
            end
        end
    else
        set right_opts $argv
    end

    buildah run $left_opts -- $__FISHTANK_BUILD_CTR $right_opts
end

function ADD
    # We can't use 'contains' here, as it consumes '--' when parsing arguments.
    for arg in $argv
        if [ $arg = -- ]
            set _has_opts yes
        end
    end

    if set -q _has_opts
        for arg in $argv
            if set -q _passed_opts
                set -a right_opts $arg
            else
                if [ $arg = -- ]
                    set _passed_opts yes
                    continue
                end

                set -a left_opts $arg
            end
        end
    else
        set right_opts $argv
    end

    buildah add $left_opts $__FISHTANK_BUILD_CTR $right_opts
end

function COPY
    ADD $argv
end

function CMD
    buildah config --cmd $argv $__FISHTANK_BUILD_CTR
end

function LABEL
    buildah config --label $argv $__FISHTANK_BUILD_CTR
end

function EXPOSE
    buildah config --port $argv $__FISHTANK_BUILD_CTR
end

function ENV
    buildah config --env $argv $__FISHTANK_BUILD_CTR
end

function ENTRYPOINT
    buildah config --entrypoint $argv $__FISHTANK_BUILD_CTR
end

function VOLUME
    buildah config --volume $argv $__FISHTANK_BUILD_CTR
end

function USER
    buildah config --user $argv $__FISHTANK_BUILD_CTR
end

function WORKDIR
    buildah config --workingdir $argv $__FISHTANK_BUILD_CTR
end

if string match -q -- "*from sourcing file*" (status)
    exit
end

# --- THE CODE BELOW IS ONLY RUN WHEN THE FILE IS *NOT* SOURCED ---

function w_annotation -a key
    buildah config -a "$key="(string join \x1F -- $argv[2..]) $__FISHTANK_BUILD_CTR
end

function r_annotation -a key
    buildah inspect -t container --format "{{index .ImageAnnotations \"$key\"}}" $__FISHTANK_BUILD_CTR | string split \x1F
end

function p_annotation -a key
    set -l data (r_annotation $key)
    set -a data $argv[2..]
    w_annotation $key (string split ' ' -- $data)
end

function tankcfg_preset -a preset
    switch $preset
        case cp-user
            if [ (count $argv) -gt 1 ]
                set USER $argv[2]
            else
                set USER (whoami)
            end

            set -l UID (id -u $USER)
            set -l GID (id -g $USER)
            set -l SHL (getent passwd $USER | cut -d : -f 7)

            RUN groupadd -g $GID $USER
            RUN useradd -u $UID -g $GID -m $USER -s $SHL
            RUN mkdir -p /etc/sudoers.d
            RUN sh -c "echo $USER ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USER"
            RUN chmod 0440 /etc/sudoers.d/$USER
        case bind-fix
            p_annotation "fishtank.security-opt" "label=disable"
            p_annotation "fishtank.userns" keep-id
        case ssh-agent
            tankcfg_preset bind-fix
            tankcfg mount type=bind,src=$SSH_AUTH_SOCK,dst=$SSH_AUTH_SOCK
        case dbus
            set -l socket (printf "%s" "$DBUS_SESSION_BUS_ADDRESS}" | sed -e 's/unix:path=\(.\+\)/src=\1,dst=\1/')
            tankcfg_preset bind-fix
            tankcfg mount type=bind,$socket
        case '*'
            eprintf "unknown preset $preset"
            exit 1
    end
end

# --- EFFECTIVE ENTRYPOINT --- #

require podman
require buildah

trap podman buildah

if [ -z "$__FISHTANK_IN_BUILD" ]
    abort "must be executed in a tankctl build context"
end

if [ -z "$argv[1]" ]
    abort "no subcommand specified"
else if functions -q "tankcfg_$argv[1]"
    tankcfg_$argv[1] $argv[2..]
else if contains "$argv[1]" $__CONFIG_FLAGS
    buildah config "--$argv[1]" $argv[2..] $__FISHTANK_BUILD_CTR
else if contains "$argv[1]" $__ANNOTATIONS
    p_annotation "fishtank.$argv[1]" $argv[2..]
else
    abort "unknown subcommand '$argv[1]'"
end
