function tankcfg_wrap -a ctr

end

# entrypoint
# env
# healthcheck
# hostname
# port
# user
# workingdir
# cap-add
# cap-drop
# cpus
# ram
# ulimit
# device
# userns
# security_opts
# mounts
# args (arbitrary)
# restart
# secret

# --- EFFECTIVE ENTRYPOINT --- #

require podman
require buildah

if [ -z "$__FISHTANK_IN_BUILD" ]
    abort "must be executed in a tankctl build context"
end

if [ -z "$argv[1]" ]
    tankcfg_help
    abort "no subcommand specified"
else if not functions -q "tankcfg_$argv[1]"
    tankcfg_help
    abort "unknown subcommand '$argv[1]'"
else
    tankcfg_$argv[1] $argv[2..]
end
