function arm -a victim -d "Arm a non-zero exit code trap for the provided executable."
    # This function remains in scope, even after 'trap' ends.
    function $victim -V victim -w $victim
        # Capture both stderr and stdout, using quoted substitution
        # to avoid splitting into a list.
        set -l output "$(command $victim $argv 2>&1)"
        # Capture the status.
        set -l stat $status

        # If the status is not zero, we've trapped an error.
        if [ "$stat" -ne 0 ]
            error $victim $stat "$argv" "$output"
        else
            # If the status is zero, all is well. Print the captured output and return.
            printf "%s" "$(echo $output)"
        end
    end
end

function trap -d "Trap non-zero exit codes for the provided exectable(s)."
    for victim in $argv
        arm $victim
    end
end

function disarm -d "Remove a non-zero exit code trap for the provided executable(s), if one exists."
    for victim in $argv
        functions -e $victim
    end
end

# Pretty-prints an error.
function error -a victim stat args output
    # Split the output into a list (on newlines.)
    set -l output (echo $output)

    # Write out the command, arguments, and exit code.
    printf "%serror trapped%s\n" (set_color red -o) (set_color normal) >&2
    printf "├── command\t%s\n" "$victim" >&2
    printf "├── argv   \t%s\n" "$args" >&2
    printf "├── code   \t%s\n" "$stat" >&2
    printf "└── " >&2

    echo -n (set_color brblack) >&2

    # Write out the output (pretty-formatted) if any exists.
    if [ -z "$output" ]
        printf "(no output)\n" >&2
    else
        printf "%s\n" $output[1] >&2

        for line in $output[2..]
            printf "    %s\n" $line >&2
        end
    end

    # Print backtrace, if enabled.
    if [ -n "$fish_backtrace" ]
        echo -n (set_color normal) >&2
        printf "\n--- BACKTRACE ---\n" >&2
        status stack-trace >&2
    end

    # Terminate with code if non-interactive.
    # Otherwise, just return the code.
    if not status is-interactive
        printf "\n%s: aborting\n" (status basename | string split '.')[1] >&2
        printf "%s" (set_color normal) >&2
        exit $stat
    else
        return $stat
    end
end
