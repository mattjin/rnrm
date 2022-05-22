use crate::actions;
use actions::RegistryWrapper;
use std::fs;

fn get_current_registry_url(reg: &impl RegistryWrapper) -> String {
    let contents =
        fs::read_to_string(&reg.get_npmrc_path()).expect("Something went wrong reading the npmrc file");
    let url = contents
        .split('\n')
        .filter(|x| x.starts_with("registry="))
        .map(|x| x.replace("registry=", ""))
        .collect::<String>();
    url
}

pub fn list_registry(reg: &impl RegistryWrapper) {
    let current_url = get_current_registry_url(reg);
    println!("{}", current_url);
    let registry_list = &reg.get_registry_list();
    let mut max_len = 0;
    for key in registry_list.keys() {
        if key.len() > max_len {
            max_len = key.len();
        }
    }
    println!("");
    for (name, url) in *registry_list {
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
    #[should_panic(expected = "Something went wrong reading the npmrc file")]
    fn it_should_panic_get_current_registry_url() {
        let registry_list: BTreeMap<String, String> = BTreeMap::new();
        let mut registry_wrapper = MockRegistry {
            registry_list,
            npmrc_path: String::from("/no_exist"),
        };
        get_current_registry_url(&mut registry_wrapper);
    }

    #[test]
    fn it_get_current_registry_url_works() {
        let path = env::current_dir().unwrap();
        let file_path = path.join(".ls_npmrc");
        let path_str = file_path.as_path().display().to_string();

        let mut file = match File::create(&path_str) {
            Err(why) => panic!("couldn't create: {}", why),
            Ok(file) => file,
        };
    
        let content: &str = "registry=https://registry.npmjs.org/\n";

        match file.write_all(content.as_bytes()) {
            Err(why) => panic!("couldn't write: {}", why),
            Ok(_) => (),
        }

        let registry_list: BTreeMap<String, String> = BTreeMap::from([("npm".to_string(), "https://registry.npmjs.org/".to_string())]);

        let mut registry_wrapper = MockRegistry {
            registry_list,
            npmrc_path: path_str.clone(),
        };

        let url = get_current_registry_url(&mut registry_wrapper);
        assert_eq!(url, "https://registry.npmjs.org/");
        fs::remove_file(path_str).unwrap();
    }
}
