**ToDo Manager**
***
**ToDo Manager** is simple ToDo app in CLI

**Features**:
+ Saving list of task in ```.tdml``` file in project directory.
+ Adding tasks with name and description.
+ Removing tasks
+ Marking tasks as ```Completed``` and ```IMPORTANT```
+ Showing list of all tasks
+ Showing details about one specific task

**Usage**
```tdm <command> [arguments]```

**Commands**

| Command | Usage                                        | Description |
| ------- | -------------------------------------------- | ----------- |
| add     | ```tdm add <task_name> [task_description]``` | Adds new task to list with name na optional description |
| remove  | ```tdm remove <task_name>``` | Removes task from the list |
| complete | ```tdm complete <task_name>``` | Toggles ```Completed``` mark for the task |
| important | ```tdm important <task_name>``` | Toggles ```IMPORTANT``` mark for the task |
| show | ```tdm show [taks_name]``` | Shows list of all task or if task name provided shows detali about one specific task |