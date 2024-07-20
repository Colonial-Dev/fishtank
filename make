#!/usr/bin/env fish

switch $argv[1]
    case "build"
        rm -f target/tankctl
        rm -f target/tankcfg

        cat src/common/* src/tankctl/* >> target/tankctl
        cat src/common/* src/tankcfg/* >> target/tankcfg
        
        chmod +x target/tankctl
        chmod +x target/tankcfg

    case "run"
        ./target/tankctl $argv[2..]

    case "lint"
        for file in (find src/ -name "*.fish")
            if not fish_indent -c $file
                set -x failure true
            end
        end

        if [ "$failure" = true ]
            exit 1
        end

    case "format"
        for file in (find src/ -name "*.fish")
            fish_indent -w $file
        end

    case "*"
        printf "tankmake: unrecognized target '$argv[1]' - aborting.\n"
        exit 1
end
