use std::collections::BTreeMap;
use std::env;
use std::path::Path;

pub fn get_registry_config() -> (
    BTreeMap<String, String>,
    BTreeMap<String, String>,
) {
    let mut registry_list = BTreeMap::new();
    let mut home_list = BTreeMap::new();

    let default_registry: BTreeMap<&str, &str> = BTreeMap::from([
        ("npm", "https://registry.npmjs.org/"),
        ("yarn", "https://registry.yarnpkg.com/"),
        ("tencent", "https://mirrors.cloud.tencent.com/npm/"),
        ("taobao", "https://registry.npmmirror.com/"),
        ("npmMirror", "https://skimdb.npmjs.com/registry/"),
    ]);
    
    let default_home: BTreeMap<&str, &str> = BTreeMap::from([
        ("npm", "https://www.npmjs.org"),
        ("yarn", "https://yarnpkg.com"),
        ("tencent", "https://mirrors.cloud.tencent.com/npm/"),
        ("taobao", "https://npmmirror.com"),
        ("npmMirror", "https://skimdb.npmjs.com"),
    ]);
    for (key, value) in default_registry {
        registry_list.insert(key.to_string(), value.to_string());
    }

    for (key, value) in default_home {
        home_list.insert(key.to_string(), value.to_string());
    }
    return (registry_list, home_list);
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
