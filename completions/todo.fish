set -l commands add do undo delete list

complete -c todo -f

complete -c todo -n "not __fish_seen_subcommand_from $commands" \
    -a "add" -d "add a todo entry"

complete -c todo -n "not __fish_seen_subcommand_from $commands" \
    -a "do" -d "set 'done' byte to TRUE"

complete -c todo -n "not __fish_seen_subcommand_from $commands" \
    -a "undo" -d "set 'done' byte to FALSE"

complete -c todo -n "not __fish_seen_subcommand_from $commands" \
    -a "delete" -d "delete todo entry"

complete -c todo -n "not __fish_seen_subcommand_from $commands" \
    -a "list" -d "list all todo entries"

complete -c todo -n "__fish_seen_subcommand_from add" \
    -a "-d" -d "set done byte to TRUE"
