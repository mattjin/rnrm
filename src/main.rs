use clap::{Parser, Subcommand};
use std::env;
use std::path::Path;
use std::fs;
use std::collections::HashMap;
use ini::Ini;
use colored::*;

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
    Add { name: Option<String>, url: Option<String> },
    Del { name: Option<String> },
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

fn get_nrmrc_path() -> String {
    let os_path = get_user_path();
    let home_path = env::var(os_path).unwrap();
    let path_buf = Path::new(&home_path.to_owned()).join(".nrmrc");
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

fn get_registry_list() -> HashMap<String, String> {
    let mut registry_list = HashMap::from([
        ("npm".to_string(), "https://registry.npmjs.org/".to_string()),
        ("yarn".to_string(), "https://registry.yarnpkg.com/".to_string()),
        ("tencent".to_string(), "https://mirrors.cloud.tencent.com/npm/".to_string()),
        ("taobao".to_string(), "https://registry.npmmirror.com/".to_string()),
        ("npmMirror".to_string(), "https://skimdb.npmjs.com/registry/".to_string()),
    ]);
    let nrmrc_path = get_nrmrc_path();
    let conf = Ini::load_from_file(&nrmrc_path).unwrap();
    let general_section_name = "__General__";
    for (sec, prop) in &conf {
        let section_name = sec.as_ref().unwrap_or(&general_section_name);
        for (_k, v) in prop.iter() {
            registry_list.insert(section_name.to_string(), v.to_string());
        }
    }
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
    for (name, url) in registry_list {
        let star = if &url == current_registry { "*" } else { " " };
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

fn use_registry(name: &str) {
    let is_exist = is_registry_exist(name);
    if !is_exist {
        println!("{:?} no exist", name);
        return;
    }
    let registry_list = get_registry_list();
    let registry_url = registry_list.get(name).unwrap();

    let path = get_npmrc_path();
    let contents = fs::read_to_string(&path)
        .expect("Something went wrong reading the file");

    let new_content = contents.split("\n").map(|x| if x.starts_with("registry=") {
        format!("registry={}", registry_url)
    } else { x.to_owned() }).collect::<Vec<String>>().join("\n");

    fs::write(&path, new_content).expect("Unable to write file");
}

fn add_registry(name: &str, url: &str) -> bool {
    let nrmrc_path = get_nrmrc_path();
    let mut conf = Ini::load_from_file(&nrmrc_path).unwrap();
    for (sec, prop) in &conf {
        for (_key, value) in prop.iter() {
            match sec {
                Some(sec_name) => {
                    if sec_name == name && url == value {
                        println!("{}", "The registry name or url is already included in the nrm registries. Please make sure that the name and url are unique.".red());
                        return  false;
                    }
                }
                None => (),
            }
        }
    }
    conf.with_section(Some(name))
        .set("registry", url);
    conf.write_to_file(&nrmrc_path).unwrap();
    return true;
}

fn del_registry(name: &str) {
    let is_exist = is_registry_exist(name);
    if !is_exist {
        println!("registry {} no exist", name.red());
        return;
    }
    let nrmrc_path = get_nrmrc_path();
    let conf = Ini::load_from_file(&nrmrc_path).unwrap();
    let mut new_conf = Ini::new();

    let general_section_name = "__General__";
    for (sec, prop) in &conf {
        let section_name = sec.as_ref().unwrap_or(&general_section_name);
        for (_key, value) in prop.iter() {
            if *section_name != name {
                new_conf.with_section(Some(section_name.to_string())).set("registry", value);
            }
        }
    }
    new_conf.write_to_file(&nrmrc_path).unwrap();
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
                None => (),
            };
        },
        Commands::Add { name, url } => {
            if let (Some(registry_name), Some(registry_url)) = (name, url) {
                add_registry(registry_name, registry_url);
            }
        },
        Commands::Del { name } => {
            match name {
                Some(n) => del_registry(n),
                None => (),
            };
        },
    }
}