use colored::*;

pub enum LogErr {
    Err,
    Warning,
    Success,
}

pub fn log(message: &str, log_err: LogErr) {
    match log_err {
        LogErr::Err => println!("{}", message.red()),
        LogErr::Warning => println!("{}", message.yellow()),
        LogErr::Success => println!("{}", message.green()),
    }
}

pub fn log_add_success(name: &str) {
    let message = format!("rnrm use {}", name);
    println!(
        "\nAdd registry {name} success, run {} command to use {name} registry.\n",
        message.green(),
        name = name
    );
}

pub fn log_add_exist_err() {
    log("\nThe registry name or url is already included in the nrm registries. Please make sure that the name and url are unique.\n", LogErr::Err);
}
