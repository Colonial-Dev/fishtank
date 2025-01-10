function trap -a command
    function $command
        command $command $argv
        
        if [ $status -ne 0 ]
            exit $status
        else
            return 0
        end
    end
end

trap bx

function buildah
    if [ $argv[1] = 'from' ]
        set -l ctr (command buildah $argv)

        if [ $status -ne 0 ]
            exit $status
        end
        
        buildah config \
            -a manager=box \
            -a box.path=$__BOX_BUILD_PATH \
            -a box.hash=$__BOX_BUILD_HASH \
            -a box.tree=$__BOX_BUILD_TREE \
            -a box.name=$__BOX_BUILD_NAME \
            $ctr

        set -gx __BOX_BUILD_CTR $ctr
        echo $ctr
    else
        command buildah $argv

        if [ $status -ne 0 ]
            exit $status
        end
    end
end

function FROM
    buildah from $argv
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

cd $__BOX_BUILD_DIR
