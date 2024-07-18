function arm -a victim -d "Arm a non-zero exit code trap for the provided executable."
    # This function remains in scope, even after 'trap' ends.
    function $victim -V victim -w $victim
        # Capture both stderr and stdout, using quoted substitution
        # to avoid splitting into a list.
        set -l output "$(command $victim $argv 2>&1)"
        # Capture the status.
        set -l stat   $status

        # If the status is not zero, we've trapped an error.
        if [ "$stat" -ne 0 ]
            # Split the output into a list (on newlines.)
            set -l output (echo $output)

            # Write out the command, arguments, and exit code.
            printf "%serror trapped%s\n" (set_color red -o) (set_color normal)
            printf "├── command\t%s\n" "$victim"
            printf "├── argv   \t%s\n" "$argv"
            printf "├── code   \t%s\n" "$stat"
            printf "└── "

            echo -n (set_color brblack)

            # Write out the output (pretty-formatted) if any exists.
            if [ -z "$output" ]
                printf "(no output)\n" 
            else
                printf "%s\n" $output[1]

                for line in $output[2..]
                    printf "    %s\n" $line
                end
            end

            # Print backtrace, if enabled.
            if [ -n "$fish_trace" ]
                echo -n (set_color normal)
                printf "\n--- BACKTRACE ---\n"
                status stack-trace
            end

            # Terminate with code if non-interactive.
            # Otherwise, just return the code.
            if not status is-interactive
                printf "\n%s: aborting\n" (status basename | string split '.')[1]
                printf "%s" (set_color normal)
                exit $stat
            else
                return $stat
            end
        # If the status is zero, all is well. Print the captured output and return.
        else
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
