use crate::actions;
use crate::logger;
use ini::Ini;

pub fn add_registry(reg: &actions::Registry, name: &str, url: &str) {
    let nrmrc_path = &reg.nrmrc_path;
    let mut conf = Ini::load_from_file(&nrmrc_path).unwrap();
    for (sec, prop) in &conf {
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
    conf.with_section(Some(name)).set("registry", url);
    conf.write_to_file(&nrmrc_path).unwrap();
    logger::log_add_success(name);
}
