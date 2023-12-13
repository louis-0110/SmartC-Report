use serde::{Deserialize, Serialize};
use std::fs;

use crate::CONFIG_DIR;

 #[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    svnPath: String,
    gitPath: String,
    apiKey: String,
    secretKey: String,
}
pub fn read_conf() -> Settings {
            let p = CONFIG_DIR.get().unwrap().join("conf/settings.conf");
    println!("{:?}", p);
    let setting_str = match fs::read_to_string(p) {
        Ok(s) => s,
        Err(e) => e.to_string(),
    };
    
    let setting_str: Settings = serde_json::from_str(&setting_str).unwrap();
    setting_str
}
