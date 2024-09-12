#!/usr/bin/env fish

function build
    mkdir -p target/
    
    rm -f target/tankctl
    rm -f target/tankcfg

    for file in (find src/ -name "*.fish")
        if not fish -n $file
            set -x failure true
        end
    end

    if [ "$failure" = true ]
        echo "do: syntax errors detected, refusing to build!"
        exit 1
    end

    cat src/common/* src/tankctl/* >> target/tankctl
    cat src/common/* src/tankcfg/* >> target/tankcfg
    
    chmod +x target/tankctl
    chmod +x target/tankcfg
end

function lint
    for file in (find src/ -name "*.fish")
        if not fish_indent -c $file
            set -x failure true
        end
    end

    if [ "$failure" = true ]
        exit 1
    end

    set -l uniq (find src/ -name "*.fish" | xargs grep -ho "^function [a-zA-Z1-9_]*" | sort | uniq -d)

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

function manual
    mkdir -p ./doc/man

    for file in (find doc/ -name "*.md")
        pandoc --standalone --to man --from markdown-smart -o $file man/(basename -s .md $file)
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
    case "install"
        build
        cp ./target/* ~/.local/bin
    case "*"
        printf "do: unrecognized target '$argv[1]' - aborting.\n"
        exit 1
end
