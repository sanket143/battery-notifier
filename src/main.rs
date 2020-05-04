extern crate rustc_serialize;

use rustc_serialize::json::Json;
use notify_rust::Notification;
use battery::units;
use std::io;
use std::io::Read;
use std::thread;
use std::time::Duration;
use std::fs::File;

fn main() -> battery::Result<()> {
    let manager = battery::Manager::new()?;
  
    let mut f = File::open("/home/sanket143/.config/battery_notifier_s/config.json")
      .expect("Unable to open");
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    println!("{}", buffer);

    let mut battery = match manager.batteries()?.next() {
        Some(Ok(battery)) => battery,
        Some(Err(e)) => {
            eprintln!("Unable to access battery information");
            return Err(e);
        }
        None => {
            eprintln!("Unable to find any batteries");
            return Err(io::Error::from(io::ErrorKind::NotFound).into());
        }
    };

    loop {
        show_notification(&battery.state_of_charge());
        thread::sleep(Duration::from_secs(5));
        manager.refresh(&mut battery)?;
    }
}

fn show_notification(ratio: &units::Ratio) {
    let mut _body = String::from("Current Battery: ");
    println!("{:?}", ratio);

    Notification::new()
        .summary("Lisa")
        .body("hello")
        .show()
        .unwrap();
}
