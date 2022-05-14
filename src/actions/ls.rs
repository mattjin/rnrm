use crate::actions;

use std::fs;

fn get_current_registry_url(reg: &actions::Registry) -> String {
    let contents =
        fs::read_to_string(&reg.npmrc_path).expect("Something went wrong reading the file");
    let url = contents
        .split('\n')
        .filter(|x| x.starts_with("registry="))
        .map(|x| x.replace("registry=", ""))
        .collect::<String>();
    url
}

pub fn list_registry(reg: &actions::Registry) {
    let current_url = get_current_registry_url(reg);
    let registry_list = &reg.registry_list;
    let mut max_len = 0;
    for key in registry_list.keys() {
        if key.len() > max_len {
            max_len = key.len();
        }
    }
    println!("");
    for (name, url) in registry_list {
        let star = if *url == current_url { "*" } else { " " };
        println!(
            "{:1} {} {:-<width$} {}",
            star,
            name,
            "-",
            url,
            width = max_len + 3 - name.len()
        );
    }
    println!("");
}
