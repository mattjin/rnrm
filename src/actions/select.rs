use crate::actions;
use std::fs;

pub fn use_registry(reg: &actions::Registry, name: &str) {
    let is_exist = reg.is_registry_exist(name);
    if !is_exist {
        println!("{:?} no exist", name);
        return;
    }
    let registry_list = &reg.registry_list;
    let registry_url = registry_list.get(name).unwrap();

    let path = &reg.npmrc_path;
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

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

    fs::write(&path, new_content).expect("Unable to write file");
}
