#!/usr/bin/env fish

function build
    mkdir -p target/
    
    rm -f target/tankctl
    rm -f target/tankcfg

    cat src/common/* src/tankctl/* >> target/tankctl
    cat src/common/* src/tankcfg/* >> target/tankcfg
    
    chmod +x target/tankctl
    chmod +x target/tankcfg
end

function lint
    function scan
        for path in (find src/ -name "*.fish")
            grep -o "^function [a-z1-9_]*" $path
        end
    end

    for file in (find src/ -name "*.fish")
        if not fish_indent -c $file
            set -x failure true
        end
    end

    if [ "$failure" = true ]
        exit 1
    end

    set -l uniq (scan | sort | uniq -d)

    if [ (count $uniq) -ne 0 ]
        scan | sort | uniq -d
        echo "do: duplicate function names detected!"
        exit 1
    end
end

function format
    for file in (find src/ -name "*.fish")
        fish_indent -w $file
    end
end

switch $argv[1]
    case "build"
        build
    case "run"
        ./target/tankctl $argv[2..]
    case "lint"
        lint
    case "format"
        format
    case "precheck"
        format
        build
        ./target/tankctl help
    case "*"
        printf "do: unrecognized target '$argv[1]' - aborting.\n"
        exit 1
end
