extern crate rustc_serialize;

use rustc_serialize::json::Json;
use notify_rust::Notification;
use battery::units;
use std::io;
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::copy;
use std::io::stdout;

fn main() -> battery::Result<()> {
    let manager = battery::Manager::new()?;
  
    let mut file = File::open("/home/sanket143/.config/battery_notifier_s/config.json").unwrap();
    let mut stdout = stdout();
    let str = &copy(&mut file, &mut stdout).unwrap().to_string();
    let data = Json::from_str(str).unwrap();

    println!("data: {:?}", data);

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
