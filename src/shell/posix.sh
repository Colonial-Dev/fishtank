set -eu

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
