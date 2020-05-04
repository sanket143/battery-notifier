pub mod config {

  use rustc_serialize::json::Json;
  use std::env;
  use std::fs::File;
  use std::io::Read;
  use std::path::PathBuf;

  pub fn get() -> Json {
    let home = env::var("HOME").expect("Unable to find home");
    let config_path = ".config/battery_notifier_s/config.json";
    let config_path: PathBuf = [home, config_path.to_string()].iter().collect();

    let mut f = File::open(config_path).expect("Unable to open file");
    let mut buffer = String::new();

    f.read_to_string(&mut buffer).expect("Unable to read string");

    Json::from_str(&buffer).unwrap()
  }
}
