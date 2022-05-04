use open;

use crate::actions;

pub fn open_registry(reg: &actions::Registry, name: &str) {
    let home = reg.home_list.get(name).unwrap();
    open::that(home).unwrap();
}
