pub mod add;
pub mod del;
pub mod ls;
pub mod open;
pub mod select;

use ini::Ini;

use crate::config;
use std::collections::BTreeMap;
extern crate open as open_rs;
use open_rs::that;

pub struct Registry {
    registry_list: BTreeMap<String, String>,
    home_list: BTreeMap<String, String>,
    npmrc_path: String,
    nrmrc_path: String,
}

impl Registry {
    pub fn new() -> Self {
        let (mut registry_list, mut home_list) = config::get_registry_config();
        let nrmrc_path = config::get_registry_config_path(".nrmrc");
        let npmrc_path = config::get_registry_config_path(".npmrc");
        let result = Ini::load_from_file(&nrmrc_path);

        match result {
            Ok(conf) => {
                let general_section_name = "";
                for (sec, prop) in &conf {
                    let section_name = sec.as_ref().unwrap_or(&general_section_name);
                    for (k, v) in prop.iter() {
                        if k == "registry" {
                            registry_list.insert(section_name.to_string(), v.to_string());
                        } else if k == "home" {
                            home_list.insert(section_name.to_string(), v.to_string());
                        }
                    }
                }
            }
            Err(_) => (),
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

pub trait RegistryWrapper {
    fn get_registry_list(&self) -> &BTreeMap<String, String>;
    fn get_home_list(&self) -> &BTreeMap<String, String>;
    fn get_nrmrc_path(&self) -> String {
        String::new()
    }
    fn get_npmrc_path(&self) -> String {
        String::new()
    }
    fn open(&self, _p: String) {}
    fn is_registry_exist(&self, _name: &str) -> bool {
        true
    }
}

impl RegistryWrapper for Registry {
    fn open(&self, p: String) {
        that(p).unwrap();
    }

    fn get_registry_list(&self) -> &BTreeMap<String, String> {
        &self.registry_list
    }

    fn get_home_list(&self) -> &BTreeMap<String, String> {
        &self.home_list
    }

    fn get_nrmrc_path(&self) -> String {
        self.nrmrc_path.clone()
    }
    fn get_npmrc_path(&self) -> String {
        self.npmrc_path.clone()
    }
}
