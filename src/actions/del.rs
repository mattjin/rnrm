use crate::actions;
use crate::logger;
use crate::logger::log;
use ini::Ini;

pub fn del_registry(reg: &actions::Registry, name: &str) {
    let is_exist = reg.is_registry_exist(name);
    if !is_exist {
        let warning_msg = format!("\nregistry {} no exist\n", name);
        log(warning_msg.as_str(), logger::LogErr::Warning);
        return;
    }
    let nrmrc_path = &reg.nrmrc_path;
    let result = Ini::load_from_file(&nrmrc_path);
    match result {
        Ok(conf) => {
            let mut new_conf = Ini::new();

            let general_section_name = "";
            for (sec, prop) in &conf {
                let section_name = sec.as_ref().unwrap_or(&general_section_name);
                for (_key, value) in prop.iter() {
                    if *section_name != name {
                        new_conf
                            .with_section(Some(section_name.to_string()))
                            .set("registry", value);
                    }
                }
            }
            new_conf.write_to_file(&nrmrc_path).unwrap();
            let success_msg = format!("\nThe registry '{}' has been deleted successfully.\n", name);
            log(success_msg.as_str(), logger::LogErr::Success)
        },
        Err(_) => (),
    }
}
