use clap::{Parser, Subcommand};

mod update;
use update::*;

mod install;
use install::*;

mod remove;
use remove::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    Update,
    Install(InstallArgs),
    Remove(RemoveArgs),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Update => {
            if let Some(status) = update_cli() {
                println!("Updated to version {}", status.version);
            } else {
                println!("Update failed!");
            };
        }
        Commands::Install(install_args) => {
            install::install_package(install_args);
        }

        Commands::Remove(remove_args) => {
            remove::remove_package(remove_args);
        }
    }
}