use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

static mut CONFIGURATION: Option<serde_json::Value> = None;

pub fn get() -> &'static serde_json::Value {
    unsafe {
        match &mut CONFIGURATION {
            Some(value) => value,
            None => {
              get_configurations();
              get()
            }
        }
    }
}

fn get_configurations() {
    let home = env::var("HOME").expect("Unable to find home");
    let config_path = ".config/battery_notifier_s/config.json";
    let config_path: PathBuf = [home, config_path.to_string()].iter().collect();

    let mut f = File::open(config_path).expect("Unable to open file");
    let mut buffer = String::new();

    f.read_to_string(&mut buffer).expect("Unable to read string");

    unsafe {
        CONFIGURATION = serde_json::from_str(&buffer)
          .expect("Unable to parse JSON");
    }
}
