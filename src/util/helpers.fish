# Wrapper around printf that emits to standard error.
function eprintf
    printf $argv >&2
end

# Wrapper around printf that emits to standard error only if a flag is set.
function vprintf
    if [ -n "$fish_verbose" ]
        eprintf $argv
    end
end