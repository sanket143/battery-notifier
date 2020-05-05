use notify_rust::Notification;
use std::thread;
use std::time::Duration;

use crate::types;
use crate::config;

static SBATTERY: types::SBattery = types::SBattery;

pub fn ticker() {
    let mut battery = SBATTERY.batteries()
      .expect("Unable to extract batteries");

    let configurations = config::get();
    let mut payload = types::NudgePayload {
        configurations,
        battery: &mut battery,
        prev: 0,
        checked: false
    };

    loop {
        nudge(&mut payload);
    }
}

fn nudge(payload: &mut types::NudgePayload) {
    let state = payload.battery.state();
    let percentage = payload.percentage();

    if payload.check() {
      println!("{}", percentage.to_string());
      println!("{:?}", payload.configurations);

      let message = payload
        .configurations["messages"]["100"][0]["message"]
        .as_str()
        .expect("Unable to convert into string")
        .to_string();
      let name = payload
        .configurations["name"]
        .as_str()
        .expect("Unable to convert into string")
        .to_string();

      let npayload = types::NotifyPayload {
          percentage,
          message,
          name
      };

      show_notification(&npayload);

    }

    thread::sleep(Duration::from_secs(5));

    SBATTERY.manager().refresh(&mut payload.battery)
      .expect("Unable to refresh battery");
}

fn show_notification(payload: &types::NotifyPayload) {
    let body = String::from("Current Battery: ");

    Notification::new()
      .summary(&payload.name)
      .body(&payload.message)
      .show()
      .unwrap();
}
