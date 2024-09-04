function RUN
    buildah run $__FISHTANK_BUILD_CTR $argv
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

function ADD
    buildah add $__FISHTANK_BUILD_CTR $argv
end

function COPY
    buildah copy $__FISHTANK_BUILD_CTR $argv
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

function p_annotation -a key value
    set -l data (r_annotation $key)
    set -a data $value
    w_annotation $key $data
end

function tankcfg_preset -a preset
    switch $preset
        case cp-user
            set -l USER

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
        case dotfiles
            tankcfg_preset bind-fix

            if [ (count $argv) -gt 2 ]
                set dst $argv[3]
            else
                set dst /home/$USER/(basename $argv[2])
            end

            buildah add --chown $USER:$USER $__FISHTANK_BUILD_CTR $argv[2] $dst
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
    tankcfg_help
    abort "no subcommand specified"
else if functions -q "tankcfg_$argv[1]"
    tankcfg_$argv[1] $argv[2..]
else if contains "$argv[1]" $__CONFIG_FLAGS
    buildah config "--$argv[1]" $argv[2..] $__FISHTANK_BUILD_CTR
else if contains "$argv[1]" $__ANNOTATIONS
    p_annotation "fishtank.$argv[1]" "$argv[2..]"
else
    tankcfg_help
    abort "unknown subcommand '$argv[1]'"
end
