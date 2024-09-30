use std::{
    fs,
    io::{Read, Write},
    os::unix::net::UnixListener,
};

use anyhow::Result;

use crate::state::ServiceState;

pub fn listen(state: &mut ServiceState) -> Result<()> {
    let socket_path = "/tmp/diligo.sock";
    if fs::metadata(socket_path).is_ok() {
        fs::remove_file(socket_path)?;
    }
    let listener = UnixListener::bind(socket_path)?;
    log::info!("Diligo Service Running.");
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut handle = || -> Result<()> {
                    log::info!("Client connected.");
                    let mut recv_buff = vec![0u8; 100];
                    stream.read(&mut recv_buff)?;
                    let recv = String::from_utf8(recv_buff)?;
                    let recv = recv.trim_matches('\0');
                    log::info!("Msg Recv: \"{recv}\"");
                    let resp = state.handle_socket_request(recv)?;
                    log::info!("Response: \"{resp}\"");
                    stream.write_all(resp.as_ref())?;
                    stream.flush()?;
                    Ok(())
                };
                if let Err(err) = handle() {
                    log::error!("{err}");
                }
            }
            Err(err) => {
                log::error!("Failed to connect: {err}");
            }
        }
    }
    Ok(())
}
