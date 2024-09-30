use std::{collections::HashMap, time::SystemTime};

use anyhow::{anyhow, Result};
use colored::Colorize;

use crate::util::ms_to_str;

pub struct ServiceState {
    idle: bool,
    ts: SystemTime,
    current_state: String,
    track: HashMap<String, u128>,
}

impl ServiceState {
    pub fn new() -> ServiceState {
        ServiceState {
            idle: false,
            ts: SystemTime::now(),
            current_state: format!("{} Linux", "".bright_blue()),
            track: HashMap::new(),
        }
    }

    pub fn handle_socket_request(&mut self, msg: &str) -> Result<String> {
        match msg {
            msg if msg.starts_with("set ") => {
                let state = &msg[4..];
                if state.len() == 0 {
                    return Err(anyhow!("new state is empty."));
                }
                self.set_state(state.to_string())?;
                Ok(format!("Changed state to \"{state}\""))
            }
            "total" => self.get_current_state(),
            "session" => self.get_current_session(),
            _ => Ok(format!("command not covered: \"{msg}\"")),
        }
    }

    fn set_state(&mut self, new_state: String) -> Result<()> {
        // mesure time elapsed since the previous state was set.
        let elapsed = self.ts.elapsed()?;
        let elapsed = elapsed.as_millis();

        // calculate new total time elapsed on the previous state.
        let new_elapsed = match self.track.get(&self.current_state) {
            Some(old_dt) => elapsed + old_dt,
            None => elapsed,
        };

        // set the track hashmap for the previous state key with the new total time elapsed.
        self.track.insert(self.current_state.clone(), new_elapsed);

        // change the current_state to the `new_state` and update the ts of the last state change
        self.current_state = new_state;
        self.ts = SystemTime::now();
        Ok(())
    }

    fn get_current_session(&self) -> Result<String> {
        let elapsed = self.ts.elapsed()?;
        let elapsed = elapsed.as_millis();
        Ok(format!(
            "{}: {}",
            self.current_state,
            ms_to_str(elapsed).bright_yellow()
        )
        .to_string())
    }

    fn get_current_state(&self) -> Result<String> {
        let elapsed = self.ts.elapsed()?;
        let elapsed = elapsed.as_millis();

        let total = match self.track.get(&self.current_state) {
            Some(old_dt) => elapsed + old_dt,
            None => elapsed,
        };

        Ok(format!(
            "{}: {}",
            self.current_state,
            ms_to_str(total).bright_yellow()
        ))
    }
}
