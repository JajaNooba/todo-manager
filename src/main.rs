use clap::Parser;

pub mod cli;
pub mod taskfs;
pub mod commands;
pub mod utils;

use cli::{ Cli, Commands };
use commands::{run_add_command, run_important_command, run_show_command};

use crate::commands::{run_remove_command, run_complete_command};

fn main() {

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(args) => run_add_command(args),
        Commands::Remove(args) => run_remove_command(args),
        Commands::Complete(args) => run_complete_command(args),
        Commands::Important(args) => run_important_command(args),
        Commands::Show(args) => run_show_command(args)
    }
}
