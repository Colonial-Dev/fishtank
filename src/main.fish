function ft_help
    printf "Good luck!\n"
end

function ft_up

end

function ft_down

end

function ft_start

end

function ft_restart

end

function ft_stop

end

function ft_list

end

function ft_build

end

function ft_create

end

function ft_exec -a container command

end

function ft_enter -a container

end

function ft_try -a tank command

end

# --- EFFECTIVE ENTRYPOINT --- #

trap rm cp mv ls mkdir

if [ -n "$XDG_CONFIG_HOME" ]
    set -x tank_dir "$XDG_CONFIG_HOME/fishtank"
else
    set -x tank_dir "$HOME/.config/fishtank"
end

mkdir -p $tank_dir

if [ -z "$argv[1]" ]
    eprintf "tankctl: no subcommand specified\n"
    ft_help
else if not functions -q "ft_$argv[1]"
    eprintf "tankctl: unknown subcommand $argv[1]\n"
    ft_help
else
    eval (ft_$argv[1] $argv[2..])
end