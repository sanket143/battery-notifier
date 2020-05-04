pub mod config {

  use rustc_serialize::json::Json;
  use std::env;
  use std::fs::File;
  use std::io::Read;

  pub fn get() {
    let HOME = env::var("HOME");
    println!("{:?}", HOME);
    let mut f = File::open("/home/sanket143/.config/battery_notifier_s/config.json").expect("Unable to open file");
    let mut buffer = String::new();

    f.read_to_string(&mut buffer).expect("Unable to read string");

    let data = Json::from_str(&buffer).unwrap();

  }
}
