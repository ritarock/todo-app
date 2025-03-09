# todo-app

```
$ todo help
Usage:
  add <task>
  list
  completed <id>
  delete <id>
```

```
$ todo add task1
add: task1

$ todo add task2
add: task2

$ todo list
[ ] 1: task1
[ ] 2: task2

$ todo completed 1
completed: task1

$ todo list
[✓] 1: task1
[ ] 2: task2

$ todo delete 1
deleted: task1

todo list    
[ ] 1: task2
```
