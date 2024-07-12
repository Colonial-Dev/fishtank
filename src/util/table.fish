#!/usr/bin/env fish

function table -d 'Create and manipulate hash tables'
    function table_hash -a key
        echo -n $key | md5sum | awk '{ print $1 }'
    end
    
    function table_make
        # Initialize temporary directory for table data
        # We (ab)use /dev/shm as backing storage
        set -l table (mktemp -d -p /dev/shm)
        # Flag the directory as a table
        touch $table/.fish_table
        # Return the table path
        printf "table\n%s" "$table"
    end
    
    function table_clear -a table 
        if count $table/* > /dev/null
            # Remove all entries.
            rm $table/* 
        end
    end
    
    function table_drop -a table
        rm -r $table
    end 
    
    function table_add -a table key val
        # Write the contents of 'val' to a file in the table directory
        # derived from the md5sum of `key`.
        #
        # We use an md5sum instead of the key directly because
        # keys may contain '/' characters (not allowed in filenames.)
        echo -n "$val" > $table/(table_hash $key)
        # Tag the file with a key attribute, for later reference.
        setfattr -n user.table.key -v "$key" $table/(table_hash $key)
    end
    
    function table_get -a table key
        if [ -e "$table/(table_hash $key)" ]
            cat $table/(table_hash $key) | string collect
        else
            return 1
        end
    end
    
    function table_del -a table key
        if [ -e "$table/(table_hash $key)" ]
            rm $table/(table_hash $key)
        end
    end
    
    function table_has -a table key
        if [ -e "$table/(table_hash $key)" ]
            return 0
        else
            return 1
        end
    end
    
    function table_keys -a table
        for file in $table/*
            echo (getfattr --absolute-names --only-values -n user.table.key $file)
        end
    end
    
    function table_values -a table
        set -l out
    
        for file in $table/*
            set -a out (cat $file | string collect)
        end
    
        string collect $out
    end
    
    function table_length -a table
        if count $table/* > /dev/null
            ls -l $table/* | wc -l
        else
            echo "0"
        end
    end
    
    function table_help
echo "\
table - create and manipulate hash tables

Usage:
    * set table_name (table make) to create a table
    * \$table_name OPERATION [ARGUMENTS]... to manipulate a table

Operations:
    * add [KEY VALUE]        add or update a KEY and VALUE pair in the table
    * get [KEY]              fetch the value associated with KEY from the table
    * del [KEY]              delete the value associated with KEY from the table
    * has [KEY]              check if the table has a value associated with KEY
    * keys                   return a list of all keys in the table
    * values                 return a list of all values in the table
    * length                 return the number of entries in the table
    * clear                  remove all entries from the table
    * drop                   permanently delete this table
\
"
    end
    
    switch $argv[1]
    case "help"
        table_help
    case "make"
        table_make
    case "*"
        set -l op $argv[2]

        if not test -e "$argv[1]" || not test -e "$argv[1]/.fish_table"
            echo "table: $argv[1] is not a valid table (has it been dropped?)"
            return 1
        end

        if not contains "table_$op" (functions)
            echo "table: unknown operation $op"
            table_help
        else
            eval (echo table_$op $argv[1] $argv[3..])
        end
    end

    functions -e table_hash
    functions -e table_make
    functions -e table_clear
    functions -e table_drop
    functions -e table_add
    functions -e table_get
    functions -e table_del
    functions -e table_keys
    functions -e table_values
    functions -e table_length
    functions -e table_help
end