use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use crate::types;

static CONFIGURATION: types::Config = types::Config;

pub fn get() -> serde_json::Value {
    let home = env::var("HOME").expect("Unable to find home");
    let config_path = ".config/battery_notifier_s/config.json";
    let config_path: PathBuf = [home, config_path.to_string()].iter().collect();

    let mut f = File::open(config_path).expect("Unable to open file");
    let mut buffer = String::new();

    f.read_to_string(&mut buffer).expect("Unable to read string");

    serde_json::from_str(&buffer)
      .expect("Unable to parse JSON")
}
