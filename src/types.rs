use std::io;
use rand::Rng;
use serde_json::Value;

use crate::types;

pub struct SBattery;

impl SBattery {
    pub fn batteries(&self) -> Result<battery::Battery, battery::Error> {
        match self.manager().batteries()?.next() {
            Some(Ok(battery)) => Ok(battery),
            Some(Err(e)) => {
                eprintln!("Unable to access battery information");
                return Err(e);
            }
            None => {
                eprintln!("Unable to find any batteries");
                return Err(io::Error::from(io::ErrorKind::NotFound).into());
            }
        }
    }

    pub fn manager(&self) -> battery::Manager {
        battery::Manager::new()
          .expect("Unable to create manager")
    }
}

pub struct NudgePayload <'a> {
    pub configurations: &'a serde_json::Value,
    pub battery: &'a mut battery::Battery,
    pub prev: i32,
    pub checked: bool
}

impl NudgePayload <'_> {
    pub fn percentage(&mut self) -> i32 {
        let percentage = self.battery.state_of_charge();
        let percentage = format!("{:.2?}", percentage);
        let percentage: f32 = percentage
          .parse()
          .expect("Unable to parse");

        let percentage: i32 = format!("{}", percentage * 100.0)
          .parse()
          .expect("Unable to parse");

        if self.prev != percentage {
            self.prev = percentage;
            self.checked = false;
        }

        percentage
    }

    pub fn check(&mut self) -> bool {
        self.percentage();

        let checked = self.checked;

        if !checked {
            self.checked = true;
        }

        return !checked;
    }

    pub fn get_notify_payload(&mut self) -> Option<types::NotifyPayload> {
        let state = self.battery.state();
        let percentage = &self.percentage().to_string();
        let configurations = &self.configurations;
        let messages_n = &configurations["messages"];
        let messages = messages_n[percentage].as_array();

        match messages {
            Some(messages) => {

                let messages: Vec<&Value> = messages
                  .iter()
                  .filter(|&x| x["status"] == "CHARGING")
                  .collect();

                let no_of_msgs = messages.len();

                if no_of_msgs == 0 {
                    return None
                }

                let msg_no = rand::thread_rng()
                  .gen_range(0, no_of_msgs);

                let messages_n = &messages_n["100"];
                let message = messages_n[0]["message"]
                  .as_str()
                  .expect("Unable to convert into string")
                  .to_string();

                let name = configurations["name"]
                  .as_str()
                  .expect("Unable to convert into string")
                  .to_string();

                Some(types::NotifyPayload {
                    message,
                    name
                })
            },
            None => None
        }
    }
}

pub struct NotifyPayload {
    pub message: String,
    pub name: String
}
