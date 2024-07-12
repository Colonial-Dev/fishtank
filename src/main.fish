#!/usr/bin/env fish

#source ./util/*

#set -l env (table make)

if [ "$foo" = "true" ]
    function bar
        echo "bar"
    end
else
    function baz
        echo "baz"
    end
end