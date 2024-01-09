use clap::{Parser, Subcommand};

mod update;
use update::*;

mod install;
use install::*;

mod remove;
use remove::*;

mod list;

mod search;
use search::*;

mod packages;

mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    List,
    Update(UpdateArgs),
    Search(SearchArgs),
    Install(InstallArgs),
    #[clap(visible_alias = "uninstall")]
    Remove(RemoveArgs),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Update(update_args) => {
            update::update_packages(update_args);
        }

        Commands::List => {
            list::list();
        }

        Commands::Search(search_args) => {
            search::search_packages(search_args);
        }

        Commands::Install(install_args) => {
            install::install_package(install_args);
        }

        Commands::Remove(remove_args) => {
            remove::remove_package(remove_args);
        }
    }
}
