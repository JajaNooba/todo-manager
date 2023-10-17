use std::fs;
use std::process;

#[derive(Clone)]
pub struct Task {
    name: String,
    description: String,
    is_complete: bool,
    is_important: bool
}

impl Task {
    pub fn to_string(&self) -> String {
        let mut str = String::new();
        str.push_str(&self.name);
        str.push('\n');
        str.push_str(&self.description);
        str.push_str("\ntask_description_end\n");
        if self.is_complete { 
            str.push_str("true") 
        } else { 
            str.push_str("false") 
        };
        if self.is_important { 
            str.push_str("true") 
        } else { 
            str.push_str("false")
        };
        str
    }
}

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
    while i < lines.len() {
        let mut line = lines[i];
        // Parsing name
        task.name = line.to_string();
        
        // Parsing task description
        while line != "task_description_end" {
            i += 1;
            line = lines[i];
            task.description.push_str(line);
        }

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