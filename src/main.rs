mod actions;
mod config;
mod logger;

use clap::{Parser, Subcommand};

use actions::add;
use actions::del;
use actions::ls;
use actions::open;
use actions::select;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all the registries
    Ls {},
    /// Change registry
    Use {
        name: Option<String>,
    },
    /// Add one custom registry
    Add {
        name: Option<String>,
        url: Option<String>,
    },
    /// Delete one custom registry
    Del {
        name: Option<String>,
    },
    /// Open the homepage of registry with optional browser
    Open {
        name: Option<String>,
    },
    /// View version
    Version {},
}

fn main() {
    let reg = actions::Registry::new();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Ls {} => {
            ls::list_registry(&reg);
        }
        Commands::Version {} => {
            let version: &str = env!("CARGO_PKG_VERSION");
            println!("v{}", version);
        }
        Commands::Use { name } => {
            match name {
                Some(n) => select::use_registry(&reg, n),
                None => (),
            };
        }
        Commands::Add { name, url } => {
            if let (Some(registry_name), Some(registry_url)) = (name, url) {
                add::add_registry(&reg, registry_name, registry_url);
            }
        }
        Commands::Del { name } => {
            match name {
                Some(n) => del::del_registry(&reg, n),
                None => (),
            };
        }
        Commands::Open { name } => {
            match name {
                Some(n) => open::open_registry(&reg, n),
                None => (),
            };
        }
    }
}
