use clap::{Parser, Subcommand};
use std::env;
use std::path::Path;
use std::fs;
use std::collections::HashMap;

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
    Use { name: Option<String> },
}

fn get_npmrc_path() -> String {
    let os = env::consts::OS;
    let os_path;
    if os == "windows" {
        os_path = "USERPROFILE";
    } else {
        os_path = "HOME";
    }
    let home_path = env::var(os_path).unwrap();
    let path_buf = Path::new(&home_path.to_owned()).join(".npmrc");
    let path = path_buf.as_path().display().to_string();
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");
    let p = contents.split('\n').filter(|x| x.starts_with("registry=")).map(|x| x.replace("registry=", "")).collect::<String>();
    p
}

fn print_registry_list(current_registry: &String) {
    let registry_list = HashMap::from([
        ("npm", "https://registry.npmjs.org/"),
        ("yarn", "https://registry.yarnpkg.com/"),
        ("tencent", "https://mirrors.cloud.tencent.com/npm/"),
        ("taobao", "https://registry.npmmirror.com/"),
        ("npmMirror", "https://skimdb.npmjs.com/registry/"),
    ]);
    let mut max_len = 0;
    for key in registry_list.keys() {
        if key.len() > max_len {
            max_len = key.len();
        }
    }
    for (name, url) in &registry_list {
        let star = if url == current_registry { "*" } else { " " };
        println!("{:1} {} {:-<width$} {}", star, name, "-", url, width = max_len + 3 - name.len());
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Ls {} => {
            let path = get_npmrc_path();
            print_registry_list(&path);
        },
        Commands::Use { name } => {
            println!("to do use {:?}", name)
        }
    }
}