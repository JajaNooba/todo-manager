use std::fs;
use std::process;

use crate::utils::Task;

pub fn write_file(tasks: Vec<Task>) {
    let mut content: String = String::new();

    for task in tasks {
        content.push_str(task.to_string().as_str());
    }

    fs::write(".tdml", content).unwrap_or_else(|err| {
        eprintln!("Cannot open or create file '.tdml'!");
        eprintln!("Error: {}", err);
        process::exit(1);
    });
}

pub fn parse_file() -> Vec<Task> {
    let mut tasks: Vec<Task> = Vec::new();

    let contents = fs::read_to_string(".tdml").unwrap_or_else(|err| {
        eprintln!("Cannot read file '.tdml'!");
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let lines: Vec<&str> = contents.split('\n').collect();
    let mut task: Task = Task { 
        name: String::new(), 
        description: String::new(), 
        is_complete: false, 
        is_important: false 
    };
    let mut i = 0;
    while i < lines.len() - 1 {
        let mut line = lines[i];
        // Parsing name
        task.name = line.to_string();
        
        // Parsing task description
        i += 1;
        line = lines[i];
        task.description = line.to_string();

        // Parse is complete
        i += 1;
        line = lines[i];
        if line == "true" {
            task.is_complete = true;
        } else {
            task.is_complete = false;
        }

        // Parse is important
        i += 1;
        line = lines[i];
        if line == "true" {
            task.is_important = true;
        } else {
            task.is_important = false;
        }

        tasks.push(task.clone());

        i += 1;
    }

    tasks
}