pub mod add;
pub mod del;
pub mod ls;
pub mod open;
pub mod select;

use ini::Ini;

use crate::config;
use std::collections::HashMap;

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

    fn is_registry_exist(&self, name: &str) -> bool {
        let registry_list = &self.registry_list;
        let result = registry_list.keys().find(|x| *x == &name);
        match result {
            Some(_) => true,
            None => false,
        }
    }
}
