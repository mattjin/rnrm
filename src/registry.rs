use colored::*;
use ini::Ini;
use std::{collections::HashMap, fs};
use open;

use crate::config;

pub struct Registry {
    registry_list: HashMap<String, String>,
    home_list: HashMap<String, String>,
    npmrc_path: String,
    nrmrc_path: String,
}

impl Registry {
    pub fn new() -> Self {
        let (mut registry_list, home_list) = config::get_registry_config();
        let nrmrc_path = config::get_registry_config_path(".nrmrc");
        let npmrc_path = config::get_registry_config_path(".npmrc");
        let conf = Ini::load_from_file(&nrmrc_path).unwrap();
        let general_section_name = "";
        for (sec, prop) in &conf {
            let section_name = sec.as_ref().unwrap_or(&general_section_name);
            for (_k, v) in prop.iter() {
                registry_list.insert(section_name.to_string(), v.to_string());
            }
        }
        Self {
            npmrc_path,
            nrmrc_path,
            registry_list,
            home_list,
        }
    }

    fn get_current_registry_url(&self) -> String {
        let contents =
            fs::read_to_string(&self.npmrc_path).expect("Something went wrong reading the file");
        let url = contents
            .split('\n')
            .filter(|x| x.starts_with("registry="))
            .map(|x| x.replace("registry=", ""))
            .collect::<String>();
        url
    }

    fn is_registry_exist(&self, name: &str) -> bool {
        let registry_list = &self.registry_list;
        let result = registry_list.keys().find(|x| *x == &name);
        match result {
            Some(_) => true,
            None => false,
        }
    }

    pub fn list_registry(&self) {
        let current_url = self.get_current_registry_url();
        let registry_list = &self.registry_list;
        let mut max_len = 0;
        for key in registry_list.keys() {
            if key.len() > max_len {
                max_len = key.len();
            }
        }
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
    }

    pub fn use_registry(&self, name: &str) {
        let is_exist = self.is_registry_exist(name);
        if !is_exist {
            println!("{:?} no exist", name);
            return;
        }
        let registry_list = &self.registry_list;
        let registry_url = registry_list.get(name).unwrap();

        let path = &self.npmrc_path;
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

    pub fn del_registry(&self, name: &str) {
        let is_exist = self.is_registry_exist(name);
        if !is_exist {
            println!("registry {} no exist", name.red());
            return;
        }
        let nrmrc_path = &self.nrmrc_path;
        let conf = Ini::load_from_file(&nrmrc_path).unwrap();
        let mut new_conf = Ini::new();

        let general_section_name = "__General__";
        for (sec, prop) in &conf {
            let section_name = sec.as_ref().unwrap_or(&general_section_name);
            for (_key, value) in prop.iter() {
                if *section_name != name {
                    new_conf
                        .with_section(Some(section_name.to_string()))
                        .set("registry", value);
                }
            }
        }
        new_conf.write_to_file(&nrmrc_path).unwrap();
    }

    pub fn add_registry(&self, name: &str, url: &str) -> bool {
        let nrmrc_path = &self.nrmrc_path;
        let mut conf = Ini::load_from_file(&nrmrc_path).unwrap();
        for (sec, prop) in &conf {
            for (_key, value) in prop.iter() {
                match sec {
                    Some(sec_name) => {
                        if sec_name == name && url == value {
                            println!("{}", "The registry name or url is already included in the nrm registries. Please make sure that the name and url are unique.".red());
                            return false;
                        }
                    }
                    None => (),
                }
            }
        }
        conf.with_section(Some(name)).set("registry", url);
        conf.write_to_file(&nrmrc_path).unwrap();
        return true;
    }

    pub fn open_registry(&self, name: &str) {
        let home = self.home_list.get(name).unwrap();
        open::that(home).unwrap();
    }
}
