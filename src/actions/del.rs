use crate::actions::{is_registry_exist, RegistryWrapper};
use crate::logger;
use crate::logger::log;
use ini::Ini;

pub fn del_registry(reg: &impl RegistryWrapper, name: &str) -> bool {
    let is_exist = is_registry_exist(reg.get_registry_list(), name);
    if !is_exist {
        let warning_msg = format!("\nregistry {} no exist\n", name);
        log(warning_msg.as_str(), logger::LogErr::Warning);
        return false;
    }
    let nrmrc_path = &reg.get_nrmrc_path();
    let result = Ini::load_from_file(&nrmrc_path);
    match result {
        Ok(conf) => {
            let mut new_conf = Ini::new();

            let general_section_name = "";
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
            let success_msg = format!("\nThe registry '{}' has been deleted successfully.\n", name);
            log(success_msg.as_str(), logger::LogErr::Success)
        }
        Err(_) => {
            panic!("Fail to read config file")
        }
    }
    return true;
}

#[cfg(test)]
mod del_tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::env;
    use std::fs;

    struct MockRegistry {
        registry_list: BTreeMap<String, String>,
        nrmrc_path: String,
    }

    impl RegistryWrapper for MockRegistry {
        fn get_registry_list(&self) -> &BTreeMap<String, String> {
            &self.registry_list
        }

        fn get_home_list(&self) -> &BTreeMap<String, String> {
            &self.registry_list
        }

        fn get_nrmrc_path(&self) -> String {
            self.nrmrc_path.clone()
        }
    }

    #[test]
    fn if_no_exist_del_not_works() {
        let registry_list: BTreeMap<String, String> = BTreeMap::new();
        let mut registry_wrapper = MockRegistry {
            registry_list,
            nrmrc_path: String::from("/no_exist"),
        };
        let result = del_registry(&mut registry_wrapper, "not_exist_key");
        assert_eq!(result, false);
    }

    #[test]
    #[should_panic(expected = "Fail to read config file")]
    fn if_no_found_config_file_should_panic() {
        let registry_list: BTreeMap<String, String> =
            BTreeMap::from([("npm".to_string(), "https://registry.npmjs.org/".to_string())]);
        let mut registry_wrapper = MockRegistry {
            registry_list,
            nrmrc_path: String::from("/no_exist"),
        };
        del_registry(&mut registry_wrapper, "npm");
    }

    #[test]
    fn del_works() {
        let registry_list: BTreeMap<String, String> =
            BTreeMap::from([("npm".to_string(), "https://registry.npmjs.org/".to_string())]);
        let path = env::current_dir().unwrap();
        let file_path = path.join("del_test.ini");
        let path_str = file_path.as_path().display().to_string();
        let mut conf = Ini::new();
        conf.with_section(Some("npm"))
            .set("registry", "https://registry.npmjs.org/");
        conf.write_to_file(&path_str).unwrap();
        let mut registry_wrapper = MockRegistry {
            registry_list,
            nrmrc_path: path_str.clone(),
        };
        del_registry(&mut registry_wrapper, "npm");

        let result = Ini::load_from_file(&path_str);
        match result {
            Ok(conf) => {
                let mut i = 0;
                for (_sec, _prop) in &conf {
                    i += 1;
                }
                assert_eq!(i, 0);
            }
            Err(_) => (),
        }
        fs::remove_file(path_str).unwrap();
    }
}
