use crate::actions::RegistryWrapper;

pub fn open_registry(reg: &impl RegistryWrapper, name: &str) {
    let home_list = reg.get_home_list();
    let home = home_list.get(name).unwrap();
    let path = home.clone();
    reg.open(path);
}

#[cfg(test)]
mod open_tests {
    use super::*;
    use std::collections::BTreeMap;

    struct MockRegistry {
        test_list: BTreeMap<String, String>,
    }

    impl RegistryWrapper for MockRegistry {
        fn open(&self, p: String) {
            println!("{}", p);
        }

        fn get_registry_list(&self) -> &BTreeMap<String, String> {
            &self.test_list
        }

        fn get_home_list(&self) -> &BTreeMap<String, String> {
            &self.test_list
        }
    }

    #[test]
    #[should_panic]
    fn if_no_exist_open_not_works() {
        let test: BTreeMap<String, String> =
            BTreeMap::new();
        let mut test_wrapper = MockRegistry { test_list: test };
        open_registry(&mut test_wrapper, "not_exist_key");
    }

    #[test]
    #[should_panic]
    fn if_no_found_open_not_works() {
        let test: BTreeMap<String, String> =
            BTreeMap::from([("npm".to_string(), "https://registry.npmjs.org/".to_string())]);
        let mut test_wrapper = MockRegistry { test_list: test };
        open_registry(&mut test_wrapper, "test");
    }

    #[test]
    fn open_works() {
        let test: BTreeMap<String, String> =
            BTreeMap::from([("npm".to_string(), "https://registry.npmjs.org/".to_string())]);
        let mut test_wrapper = MockRegistry { test_list: test };
        open_registry(&mut test_wrapper, "npm");
    }
}
