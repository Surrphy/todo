# Design

ToDo will store user data and their todo list in a one-file database in $HOME/.config/todo/--. Alpha release will just have simple text todos, with done field. After that there should be project that this task will be binded to, which will make things more readable. Modular design

## Data storage

Approach for storing todos data

- Database: Perky

## "UX" design

Simple cargo-like module → module_function design. Default module is todos.

```sh 
todo {module_name} {module_function}

# example for creating new todo

todo todos new [todos_name]
#or
todo new [todos_name]
```

## Error handling

Learn anyhow :)
