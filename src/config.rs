use ini::Ini;
use std::collections::HashMap;
use std::env;
use std::path::Path;

pub fn get_registry_config() -> (
    HashMap<std::string::String, std::string::String>,
    HashMap<std::string::String, std::string::String>,
) {
    let path = get_init_path();
    println!("{}", path);
    let mut registry_list = HashMap::new();
    let mut home_list = HashMap::new();
    let conf = Ini::load_from_file(path).unwrap();

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
    return (registry_list, home_list);
}

fn get_init_path() -> String {
    let current_dir = std::env::current_exe().unwrap();
    let path = current_dir.join("config.ini").as_path().display().to_string();
    path
}

fn get_user_path() -> &'static str {
    let os = env::consts::OS;
    let os_path;
    if os == "windows" {
        os_path = "USERPROFILE";
    } else {
        os_path = "HOME";
    }
    os_path
}

pub fn get_registry_config_path(name: &str) -> String {
    let os_path = get_user_path();
    let home_path = env::var(os_path).unwrap();
    let path_buf = Path::new(&home_path.to_owned()).join(name);
    let path = path_buf.as_path().display().to_string();
    path
}

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn it_get_right_config_path() {
        env::set_var("HOME", "/Users/home");
        let path = "/Users/home/abc";
        assert_eq!(path, get_registry_config_path("abc"));
    }
}
