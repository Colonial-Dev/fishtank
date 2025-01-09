function bx
    bx $argv

    if [ $status -ne 0 ]
        exit $status
    else
        return 0
    end
end

function RUN
    bx config run $argv
end

function ADD
    bx config add $argv
end

function COPY
    ADD $argv
end

function CMD
    bx config cmd $argv
end

function LABEL
    bx config label $argv
end

function EXPOSE
    bx config expose $argv
end

function ENV
    bx config env $argv
end

function ENTRYPOINT
    bx config entrypoint $argv
end

function VOLUME
    bx config volume $argv
end

function USER
    bx config user $argv
end

function WORKDIR
    bx config workdir $argv
end

function PRESET
    bx config preset $argv
end

