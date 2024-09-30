use std::{
    collections::HashMap,
    os::linux::raw::stat,
    time::{Duration, SystemTime},
};

use anyhow::{anyhow, Result};

pub struct ServiceState {
    idle: bool,
    ts: SystemTime,
    current_state: String,
    track: HashMap<String, Duration>,
}

impl ServiceState {
    pub fn new() -> ServiceState {
        ServiceState {
            idle: false,
            ts: SystemTime::now(),
            current_state: String::from("any"),
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
                self.set_state(state.to_string());
                Ok(String::from("Changed state to \"{state}\""))
            }
            _ => Err(anyhow!("command not covered for {msg}")),
        }
    }

    fn set_state(&mut self, state: String) -> Result<()> {
        let elapsed = self.ts.elapsed()?;
        self.track.insert(state.clone(), elapsed);
        self.current_state = state;
        self.ts = SystemTime::now();
        Ok(())
    }
}
