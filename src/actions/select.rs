use crate::actions::{is_registry_exist, RegistryWrapper};
use std::fs;

pub fn use_registry(reg: &impl RegistryWrapper, name: &str) -> bool {
    let registry_list = &reg.get_registry_list();
    let is_exist = is_registry_exist(registry_list, name);
    if !is_exist {
        println!("{:?} no exist", name);
        return false;
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
    return true;
}

#[cfg(test)]
mod ls_tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::env;
    use std::fs;
    use std::fs::File;
    use std::io::prelude::*;

    struct MockRegistry {
        registry_list: BTreeMap<String, String>,
        npmrc_path: String,
    }

    impl RegistryWrapper for MockRegistry {
        fn get_registry_list(&self) -> &BTreeMap<String, String> {
            &self.registry_list
        }

        fn get_home_list(&self) -> &BTreeMap<String, String> {
            &self.registry_list
        }

        fn get_npmrc_path(&self) -> String {
            self.npmrc_path.clone()
        }
    }

    #[test]
    fn fail_to_use() {
        let registry_list: BTreeMap<String, String> = BTreeMap::from([("npm".to_string(), "https://registry.npmjs.org/".to_string())]);
        let mut registry_wrapper = MockRegistry {
            registry_list,
            npmrc_path: String::new(),
        };
        let flag = use_registry(&mut registry_wrapper, "test");
        assert_eq!(flag, false);
    }

    #[test]
    fn use_works() {
        let path = env::current_dir().unwrap();
        let file_path = path.join(".use_npmrc");
        let path_str = file_path.as_path().display().to_string();

        let mut file = match File::create(&path_str) {
            Err(why) => panic!("couldn't create: {}", why),
            Ok(file) => file,
        };
    
        let content: &str = "registry=https://registry.test.org/\n";

        match file.write_all(content.as_bytes()) {
            Err(why) => panic!("couldn't write: {}", why),
            Ok(_) => (),
        }

        let registry_list: BTreeMap<String, String> = BTreeMap::from([("npm".to_string(), "https://registry.npmjs.org/".to_string())]);

        let mut registry_wrapper = MockRegistry {
            registry_list,
            npmrc_path: path_str.clone(),
        };

        let flag = use_registry(&mut registry_wrapper, "npm");
        assert_eq!(flag, true);
        let new_content = fs::read_to_string(&path_str).unwrap();
        assert!(new_content.find("https://registry.npmjs.org").is_some());
        fs::remove_file(path_str).unwrap();
    }
}
