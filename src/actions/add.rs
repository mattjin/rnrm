use crate::actions::RegistryWrapper;
use crate::logger;
use ini::Ini;

pub fn add_registry(reg: &impl RegistryWrapper, name: &str, url: &str, home: &Option<String>) -> bool {
    let is_exist = reg.is_registry_exist(name);
    if !is_exist {
        logger::log_add_exist_err();
        return false;
    }

    let nrmrc_path = &reg.get_nrmrc_path();
    let mut conf = Ini::new();
    conf.with_section(Some(name)).set("registry", url);
    match home {
        Some(home_url) => {
            conf.with_section(Some(name)).set("home", home_url);
        }
        None => (),
    }
    conf.write_to_file(&nrmrc_path).unwrap();
    logger::log_add_success(name);
    return true;
}

#[cfg(test)]
mod add_tests {
    use super::*;
    use std::collections::BTreeMap;

    struct MockRegistry {
        test_list: BTreeMap<String, String>,
    }

    impl RegistryWrapper for MockRegistry {

        fn get_registry_list(&self) -> &BTreeMap<String, String> {
            &self.test_list
        }

        fn get_home_list(&self) -> &BTreeMap<String, String> {
            &self.test_list
        }

        fn get_nrmrc_path(&self) -> String {
            String::from("/no_exist")
        }
    }

    #[test]
    #[should_panic]
    fn wrong_nrmrc_path_not_works() {
        let test: BTreeMap<String, String> =
            BTreeMap::new();
        let mut test_wrapper = MockRegistry { test_list: test };
        add_registry(&mut test_wrapper, "new_key", "test_url", &None);
    }

    #[test]
    #[should_panic]
    fn add_exist_key() {
        let test: BTreeMap<String, String> =
            BTreeMap::from([("npm".to_string(), "https://registry.npmjs.org/".to_string())]);
        let mut test_wrapper = MockRegistry { test_list: test };
        let flag = add_registry(&mut test_wrapper, "npm", "test_url", &None);
        assert_eq!(flag, false);
    }
}
