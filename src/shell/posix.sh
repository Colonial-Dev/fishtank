set -eu

buildah() {
    if [ "$1" = 'from' ]; then
        ctr=$(command buildah "$@")
        
        if [ $? -ne 0 ]; then
            exit $?
        fi
        
        buildah config \
            -a manager=box \
            -a box.path=$__BOX_BUILD_PATH \
            -a box.hash=$__BOX_BUILD_HASH \
            -a box.tree=$__BOX_BUILD_TREE \
            -a box.name=$__BOX_BUILD_NAME \
            "$ctr"

        export __BOX_BUILD_CTR="$ctr"
        echo "$ctr"
    else
        command buildah "$@"
        
        if [ $? -ne 0 ]; then
            exit $?
        fi
    fi
}

FROM() {
    buildah from "$@"
}

RUN() {
    bx config run "$@"
}

ADD() {
    bx config add "$@"
}

COPY() {
    ADD "$@"
}

CMD() {
    bx config cmd "$@"
}

LABEL() {
    bx config label "$@"
}

EXPOSE() {
    bx config expose "$@"
}

ENV() {
    bx config env "$@"
}

ENTRYPOINT() {
    bx config entrypoint "$@"
}

VOLUME() {
    bx config volume "$@"
}

USER() {
    bx config user "$@"
}

WORKDIR() {
    bx config workdir "$@"
}

PRESET() {
    bx config preset "$@"
}

cd $__BOX_BUILD_DIR
