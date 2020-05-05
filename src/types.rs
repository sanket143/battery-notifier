use std::io;

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

        println!("{} {}", self.prev, percentage);

        if self.prev != percentage {
            self.prev = percentage;
            self.checked = false;
        }

        percentage
    }

    pub fn check(&mut self) -> bool {
        let checked = self.checked;

        if !checked {
            self.checked = true;
        }

        return !checked;
    }
}

pub struct NotifyPayload {
    pub percentage: i32,
    pub message: String,
    pub name: String
}
