use battery::units;
use notify_rust::Notification;
use std::io;
use std::thread;
use std::time::Duration;

fn main() -> battery::Result<()> {
    let manager = battery::Manager::new()?;
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
