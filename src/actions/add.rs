use crate::actions::{is_registry_exist, RegistryWrapper};
use crate::logger;
use ini::Ini;

pub fn add_registry(
    reg: &impl RegistryWrapper,
    name: &str,
    url: &str,
    home: &Option<String>,
) -> bool {
    let is_exist = is_registry_exist(reg.get_registry_list(), name);
    if is_exist {
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
    use std::env;
    use std::fs;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    struct MockRegistry {
        test_list: BTreeMap<String, String>,
        nrmrc_path: String,
    }

    impl RegistryWrapper for MockRegistry {
        fn get_registry_list(&self) -> &BTreeMap<String, String> {
            &self.test_list
        }

        fn get_home_list(&self) -> &BTreeMap<String, String> {
            &self.test_list
        }

        fn get_nrmrc_path(&self) -> String {
            self.nrmrc_path.clone()
        }
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    fn count_lines(path: String) -> u32 {
        let mut l = 0;
        if let Ok(lines) = read_lines(path) {
            for line in lines {
                if let Ok(txt) = line {
                    println!("{}", txt);
                    l += 1;
                }
            }
        }
        return l;
    }

    #[test]
    fn it_works() {
        let test: BTreeMap<String, String> = BTreeMap::new();
        let path = env::current_dir().unwrap();
        let file_path = path.join("add_test.ini");
        let path_str = file_path.as_path().display().to_string();
        let mut test_wrapper = MockRegistry {
            test_list: test,
            nrmrc_path: path_str.clone(),
        };
        add_registry(&mut test_wrapper, "new_key", "test_url", &None);
        let l = count_lines(path_str.clone());

        assert_eq!(l, 2);
        add_registry(
            &mut test_wrapper,
            "new_key",
            "test_url",
            &Some("home_url".to_string()),
        );
        let l = count_lines(path_str.clone());

        assert_eq!(l, 3);
        fs::remove_file(path_str).unwrap();
    }

    #[test]
    #[should_panic]
    fn wrong_nrmrc_path_not_works() {
        let test: BTreeMap<String, String> = BTreeMap::new();
        let mut test_wrapper = MockRegistry {
            test_list: test,
            nrmrc_path: String::from("/no_exist"),
        };
        add_registry(&mut test_wrapper, "new_key", "test_url", &None);
    }

    #[test]
    fn add_exist_key() {
        let test: BTreeMap<String, String> =
            BTreeMap::from([("npm".to_string(), "https://registry.npmjs.org/".to_string())]);
        let path = env::current_dir().unwrap();
        let file_path = path.join("text.txt");
        let path_str = file_path.as_path().display().to_string();
        fs::write(path_str.clone(), "").unwrap();
        let mut test_wrapper = MockRegistry {
            test_list: test,
            nrmrc_path: path_str.clone(),
        };
        let flag = add_registry(&mut test_wrapper, "npm", "test_url", &None);
        assert_eq!(flag, false);
        fs::remove_file(path_str).unwrap();
    }
}
