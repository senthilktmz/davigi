use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mycli", version = "1.0", about = "A CLI tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Model {
        #[command(subcommand)]
        subcommand: ModelSubCommand,
    },
}

#[derive(Subcommand)]
enum ModelSubCommand {
    Describe { model_file_path: PathBuf },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Model { subcommand } => match subcommand {
            ModelSubCommand::Describe { model_file_path } => {
                println!("{:?}", model_file_path);
            }
        },
    }
}

//
//
//
//
//
//
//
