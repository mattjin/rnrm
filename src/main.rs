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

fn get_user_path() -> &'static str {
    let os = env::consts::OS;
    let os_path;
    if os == "windows" {
        os_path = "USERPROFILE";
    } else {
        os_path = "HOME";
    }
    os_path
}

fn get_npmrc_path() -> String {
    let os_path = get_user_path();
    let home_path = env::var(os_path).unwrap();
    let path_buf = Path::new(&home_path.to_owned()).join(".npmrc");
    let path = path_buf.as_path().display().to_string();
    path
}

fn get_current_registry_url() -> std::string::String {
    let path = get_npmrc_path();
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");
    let url = contents.split('\n').filter(|x| x.starts_with("registry=")).map(|x| x.replace("registry=", "")).collect::<String>();
    url
}

fn get_registry_list() -> HashMap<&'static str, &'static str> {
    let registry_list = HashMap::from([
        ("npm", "https://registry.npmjs.org/"),
        ("yarn", "https://registry.yarnpkg.com/"),
        ("tencent", "https://mirrors.cloud.tencent.com/npm/"),
        ("taobao", "https://registry.npmmirror.com/"),
        ("npmMirror", "https://skimdb.npmjs.com/registry/"),
    ]);
    return registry_list;
}

fn print_registry_list(current_registry: &String) {
    let registry_list = get_registry_list();
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

fn is_registry_exist(name: &str) -> bool {
    let registry_list = get_registry_list();
    let result = registry_list.keys().find(|x| *x == &name);
    match result {
        Some(_) => true,
        None => false,
    }
}

fn use_registry(name: &str) -> &'static str {
    let is_exist = is_registry_exist(name);
    if !is_exist {
        println!("{:?} no exist", name);
        return "";
    }
    let registry_list = get_registry_list();
    let registry_url = registry_list.get(name).unwrap();

    let path = get_npmrc_path();
    let contents = fs::read_to_string(&path)
        .expect("Something went wrong reading the file");
    // let url = contents.split('\n').filter(|x| x.starts_with("registry=")).map(|x| x.replace("registry=", "")).collect::<String>();
    let new_content = contents.split("\n").map(|x| if x.starts_with("registry=") {
        format!("registry={}", registry_url)
    } else { x.to_owned() }).collect::<Vec<String>>().join("\n");

    fs::write(&path, new_content).expect("Unable to write file");

    return registry_url;
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Ls {} => {
            let path = get_current_registry_url();
            print_registry_list(&path);
        },
        Commands::Use { name } => {
            match name {
                Some(n) => use_registry(n),
                None => "name empty",
            };
        }
    }
}