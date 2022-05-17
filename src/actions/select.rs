use crate::actions::{is_registry_exist, RegistryWrapper};
use std::fs;

pub fn use_registry(reg: &impl RegistryWrapper, name: &str) {
    let registry_list = &reg.get_registry_list();
    let is_exist = is_registry_exist(registry_list, name);
    if !is_exist {
        println!("{:?} no exist", name);
        return;
    }
    let registry_url = registry_list.get(name).unwrap();

    let path = &reg.get_npmrc_path();
    let contents = fs::read_to_string(path).expect("read npmrc file error!");

    let new_content = contents
        .split("\n")
        .map(|x| {
            if x.starts_with("registry=") {
                format!("registry={}", registry_url)
            } else {
                x.to_owned()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    fs::write(&path, new_content).expect("Unable to write npmrc file");
    println!("\nRegistry has been set to: {}\n", registry_url);
}
