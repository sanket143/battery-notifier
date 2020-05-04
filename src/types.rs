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

pub struct Config;

pub struct Message {
    message: String,
    status: bool
}
