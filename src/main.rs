mod config;
mod registry;
mod logger;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Ls {},
    Use {
        name: Option<String>,
    },
    Add {
        name: Option<String>,
        url: Option<String>,
    },
    Del {
        name: Option<String>,
    },
    Open {
        name: Option<String>,
    },
}

fn main() {
    let reg = registry::Registry::new();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Ls {} => {
            reg.list_registry();
        }
        Commands::Use { name } => {
            match name {
                Some(n) => reg.use_registry(n),
                None => (),
            };
        }
        Commands::Add { name, url } => {
            if let (Some(registry_name), Some(registry_url)) = (name, url) {
                reg.add_registry(registry_name, registry_url);
            }
        }
        Commands::Del { name } => {
            match name {
                Some(n) => reg.del_registry(n),
                None => (),
            };
        }
        Commands::Open { name } => {
            match name {
                Some(n) => reg.open_registry(n),
                None => (),
            };
        }
    }
}
