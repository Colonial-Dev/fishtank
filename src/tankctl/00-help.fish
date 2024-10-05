# TODO all help for commands (automate somehow?)
# TODO man pages
# TODO completions

function tankctl_help
    echo "\
A simple interactive container manager for the fish shell.

Usage: $(status basename) <COMMAND>

Commands:
  [ Work with container definitions and images ]
  build   - compile definitions into container images
  create  - create a new definition
  down    - stop and remove one or more containers
  edit    - edit a definition
  up      - create and start one or more containers
  reup    - remove and recreate one or more containers

  [ Work with live containers ]
  enter   - execute \$SHELL inside a container
  exec    - execute a command inside a container
  list    - list all managed containers
  restart - restart one or more containers
  start   - start one or more containers
  stop    - stop one or more containers
"
end

function tankctl_build_help

end
