set -l __CONFIG_FLAGS \
    entrypoint \
    env \
    healthcheck \
    hostname \
    port \
    shell \
    user \
    volume \
    workingdir

set -l __ANNOTATIONS \
    args \
    cap-add \
    cap-drop \
    cpus \
    ram \
    ulimit \
    devices \
    userns \
    security_opts \
    mounts \
    restart \
    secrets 

function w_annotation -a key
    buildah config -a "$key="(string join \x1F $argv[2..]) $__FISHTANK_BUILD_CTR
end

function r_annotation -a key
    buildah inspect -t container --format "{{index .ImageAnnotations \"$key\"}}" $__FISHTANK_BUILD_CTR | string split \x1F
end

function p_annotation -a key value
    set -l data (r_annotation $key)
    set -a data $value
    w_annotation $key $data
end

function tankcfg_wrap
    # RUN
    # CMD
    # LABEL
    # EXPOSE
    # ENV
    # ADD
    # COPY
    # ENTRYPOINT
    # VOLUME
    # USER
    # WORKDIR
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
