use crate::utils::Task;
use crate::cli::TaskDescArg;
use crate::taskfs;
use std::path::Path;

pub fn run_add_command(args: &TaskDescArg) {
    let mut tasks = Vec::new();

    if Path::new(".tdml").exists() {
        tasks = taskfs::parse_file();   
    }

    let task: Task = Task { 
        name: args.task_name.clone(), 
        description: args.task_description.clone(), 
        is_complete: false, 
        is_important: false
    };

    tasks.push(task);

    taskfs::write_file(tasks);
}