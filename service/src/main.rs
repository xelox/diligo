use anyhow::Result;
use simple_logger::SimpleLogger;
use socket::listen;
use state::ServiceState;
use time::macros::format_description;

mod socket;
mod state;

fn main() -> Result<()> {
    SimpleLogger::new()
        .env()
        .with_timestamp_format(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second]"
        ))
        .init()
        .unwrap();

    let mut state = ServiceState::new();
    listen(&mut state)
}
