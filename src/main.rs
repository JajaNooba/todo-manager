use clap::Parser;

pub mod cli;
pub mod taskfs;
pub mod commands;
pub mod utils;

use cli::{ Cli, Commands };
use commands::run_add_command;

fn main() {

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(args) => run_add_command(args),
        Commands::Remove(args) => println!("Remove {}", args.task_name),
        Commands::Complete(args) => println!("Complete {}", args.task_name),
        Commands::Important(args) => println!("Important {}", args.task_name),
        Commands::Show(args) => println!("Show {}", args.task_name)
    }

    println!("Hello, world!");
}
