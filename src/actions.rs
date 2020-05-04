use std::thread;
use std::time::Duration;
use notify_rust::Notification;
use battery::units;
use crate::types;

static SBATTERY: types::SBattery = types::SBattery {};

pub fn nudge() {
    let mut battery = SBATTERY.batteries()
      .expect("Unable to extract batteries");

    show_notification(&battery.state_of_charge());

    thread::sleep(Duration::from_secs(5));

    SBATTERY.manager().refresh(&mut battery)
      .expect("Unable to refresh battery");
}

fn show_notification(ratio: &units::Ratio) {
    let mut body = String::from("Current Battery: ");
    let ratio = format!("{:.2?}", ratio);
    let ratio: f32 = ratio.parse().expect("Unable to parse");
    let ratio = format!("{}", ratio * 100.0);

    body.push_str(&ratio);

    Notification::new()
        .summary("Lisa")
        .body(&body)
        .show()
        .unwrap();
}
