use std::{env, process};

use varman::Config;

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("解析命令出错: {}", err);
        process::exit(1);
    });
    
    env::set_current_dir(&config.path).unwrap_or_else(|e| panic!("{} {}", config.path, e));

    varman::run()
}