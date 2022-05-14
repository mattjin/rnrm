use crate::actions;
use crate::logger;
use ini::Ini;

pub fn add_registry(reg: &actions::Registry, name: &str, url: &str, home: &Option<String>) {
    let nrmrc_path = &reg.nrmrc_path;
    let result = Ini::load_from_file(&nrmrc_path);
    match result {
        Ok(i) => {
            for (sec, prop) in &i {
                for (_key, value) in prop.iter() {
                    match sec {
                        Some(sec_name) => {
                            if sec_name == name && url == value {
                                logger::log_add_exist_err();
                                return;
                            }
                        }
                        None => (),
                    }
                }
            }
        },
        Err(_) => (),
    }
    let mut conf = Ini::new();
    conf.with_section(Some(name)).set("registry", url);
    match home {
        Some(home_url) => {
            conf.with_section(Some(name)).set("home", home_url);
        },
        None => (),
    }
    conf.write_to_file(&nrmrc_path).unwrap();
    logger::log_add_success(name);
}
