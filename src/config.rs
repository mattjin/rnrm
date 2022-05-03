use std::collections::HashMap;
use std::env;
use std::path::Path;

pub fn get_registry_config() -> HashMap<String, String> {
    let registry_list = HashMap::from([
        ("npm".to_string(), "https://registry.npmjs.org/".to_string()),
        (
            "yarn".to_string(),
            "https://registry.yarnpkg.com/".to_string(),
        ),
        (
            "tencent".to_string(),
            "https://mirrors.cloud.tencent.com/npm/".to_string(),
        ),
        (
            "taobao".to_string(),
            "https://registry.npmmirror.com/".to_string(),
        ),
        (
            "npmMirror".to_string(),
            "https://skimdb.npmjs.com/registry/".to_string(),
        ),
    ]);
    return registry_list;
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
