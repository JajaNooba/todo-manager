use crate::utils::Task;
use crate::cli::{TaskDescArg, TaskArg};
use crate::taskfs;
use std::path::Path;
use std::process;

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

pub fn run_remove_command(args: &TaskArg) {
    let mut tasks = taskfs::parse_file(); 

    let index = find_task(args.task_name.clone(), &tasks).unwrap_or_else(|err| {
        eprintln!("Failed to find '{}' task!", args.task_name);
        eprintln!("Error: {}", err);
        process::exit(1);
    });
    
    tasks.remove(index);

    taskfs::write_file(tasks);
}

pub fn run_complete_command(args: &TaskArg) {
    let mut tasks = taskfs::parse_file(); 

    let index = find_task(args.task_name.clone(), &tasks).unwrap_or_else(|err| {
        eprintln!("Failed to find '{}' task!", args.task_name);
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    tasks[index].is_complete = !tasks[index].is_complete;

    taskfs::write_file(tasks);
}

pub fn run_important_command(args: &TaskArg) {
    let mut tasks = taskfs::parse_file(); 

    let index = find_task(args.task_name.clone(), &tasks).unwrap_or_else(|err| {
        eprintln!("Failed to find '{}' task!", args.task_name);
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    tasks[index].is_important = !tasks[index].is_important;

    taskfs::write_file(tasks);
}

fn find_task(task_name: String, tasks: &Vec<Task>) -> Result<usize, &str> {
    let mut index = usize::MAX;
    for i in 0..tasks.len() {
        let item = tasks[i].clone();
        if item.name == task_name {
            index = i;
            break;
        }
    }

    if index != usize::MAX {
        return Ok(index);
    } else {
        return Err("Cannot find task with provided name!");
    }
}