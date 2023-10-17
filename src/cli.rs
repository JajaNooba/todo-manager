use clap::{ Parser, Subcommand, Args };

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

/// Simple ToDo app in CLI
#[derive(Subcommand)]
pub enum Commands {
    /// Adds task to ToDo list
    Add(TaskDescArg),
    /// Removes task from ToDo list
    Remove(TaskArg),
    /// Toggles 'Complete' flag on provided task
    Complete(TaskArg),
    /// Toggles 'Important' flag on provided task
    Important(TaskArg),
    /// Prints all task for current dir
    Show(TaskArgNotRequied)
}

#[derive(Args)]
pub struct TaskDescArg {
    /// Name of task
    pub task_name: String,
    /// Description of task
    #[arg(required = false, default_value = "Description was not provided")]
    pub task_description: String
}

#[derive(Args)] 
pub struct TaskArg {
    /// Name of task
    pub task_name: String
}

#[derive(Args)]
pub struct TaskArgNotRequied {
    /// Name of task
    #[arg(required = false, default_value = "")]
    pub task_name: String
}