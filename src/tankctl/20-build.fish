function enumerate_defs
    for path in (find -L $__TANK_DIR -name "*.tank")
        vprintf "definition at '$path' enumerated"
        echo $path
    end
end

function locate_def -a name
    find -L $__TANK_DIR -name "$name.tank"
end

function filter_dupes
    set -l nonuniq (echo $argv | tr ' ' '\n' | sort | uniq -d)

    if [ (count $nonuniq) -ne 0 ]
        eprintf "the following definition file names occur more than once:"
        echo $nonuniq | tr ' ' '\n'
        abort "duplicate definition file names are not allowed"
    end
end

function filter_unchanged
    # Initialize a new hash table.
    set -l path_hash (table make)

    # Read path -> hash mappings from existing containers into the table.
    for id in (enumerate_imgs)
        set -l path (img_annotation $id "fishtank.path")
        set -l hash (img_annotation $id "fishtank.hash")

        $path_hash add $path $hash
    end

    for path in $argv
        if set -l hash ($path_hash get $path)
            # If the path maps to a hash, check if the current file has a different hash.
            # If yes, we know there has been a change.
            if [ (md5 $path) != $hash ]
                echo $path
            end
        else
            # If the path does not map to a hash, assume the file is new and therefore
            # functionally 'has changed.'
            echo $path
        end
    end

    # Drop the hash table. 
    $path_hash drop
end

function do_build -a def
    vprintf "['%s'] Starting build..." "$def"

    # Parse any directives in the file.
    # (Unknown directives are not an error; they are simply ignored.)
    for match in (grep -o "^# fishtank [a-z-]*" $def)
        vprintf "['%s'] found directive %s" "$def" "$match"
        set __(echo $match | sed 's/# fishtank //') yes
    end

    set -l name (basename -s .tank $def)

    if [ -n "$__containerfile" ]
        vprintf "['%s'] Containerfile directive set" "$def"

        set -a invoke podman build \
            --pull=newer \
            --annotation manager=fishtank \
            --annotation fishtank.path=$def \
            --annotation fishtank.hash=(md5 $def) \
            --annotation fishtank.name=$name \
            --tag $name \
            --file $def
    else
        if not [ -x $def ]
            chmod +x $def
        end

        set -a invoke fish \
            -C "
                # Change working directory to that of the definition file
                cd $(dirname $def)
                # Set flag for tankcfg to check
                set -gx __FISHTANK_IN_BUILD yes
            
                function buildah
                    if [ \$argv[1] = 'from' ]
                        set -l ctr (command buildah \$argv)
                        
                        buildah config \
                            -a manager=fishtank \
                            -a fishtank.path=$def \
                            -a fishtank.hash=$(md5 $def) \
                            -a fishtank.name=$name \
                            \$ctr

                        set -gx __FISHTANK_BUILD_CTR \$ctr
                        echo \$ctr
                    else
                        command buildah \$argv

                        if [ \$status -ne 0 ]
                            error buildah \$status \"\$argv\"
                        end
                    end
                end

                source (which tankcfg)
                trap tankcfg
            "

        if [ -n "$__unshare" ]
            set -p invoke buildah unshare
        end

        set -a invoke $def
    end

    command $invoke </dev/null

    if [ $status -ne 0 ]
        abort "build failed for definition $name!"
    end

    vprintf "['%s'] Build complete!" "$def"
end

function tankctl_build
    set -l options

    set -a options (fish_opt -s h -l help)
    set -a options (fish_opt -s f -l force)

    argparse -i $options -- $argv

    if set -q _flag_help
        tankctl_build_help
        return
    end

    if [ (count $argv) -eq 0 ]
        set defs (enumerate_defs)
    else
        set defs (map locate_def $argv)
    end

    filter_dupes $defs

    if not set -q _flag_force
        set defs (filter_unchanged $defs)
    end

    if [ (count $defs) -eq 0 ]
        abort "no valid definitions found - did you make a typo or forget --force?"
    end

    map do_build $defs
end
