# ToDo
Simple software for creating todo lists. It uses one-file database (PersyDB) to store todos persistantly.

## Usage

### Subcommands

- `todo list`
![list usage](img/list.png) 

- `todo add`
![add usage](img/add.png) 

- `todo do / undo`
![do usage](img/do.png) 
![undo usage](img/undo.png) 

- `todo delete`
![delete usage](img/delete.png) 

### Configuration

For now, the only thing that can be configured is persydb path. By default it will try to default to `~/.config/todo/`. To change this behaviour you can set `TODO_CONFIG` env variable and point it to your desired path.
