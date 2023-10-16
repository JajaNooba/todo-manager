use clap::Parser;
use todo_manager::{ Cli, Commands };

fn main() {

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(args) => println!("Add {}", args.task_name),
        Commands::Remove(args) => println!("Remove {}", args.task_name),
        Commands::Complete(args) => println!("Complete {}", args.task_name),
        Commands::Important(args) => println!("Important {}", args.task_name),
        Commands::Show => println!("Show")
    }

    println!("Hello, world!");
}
