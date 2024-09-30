use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
};

use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args = args.join(" ");

    let socket_path = "/tmp/diligo.sock";
    let mut stream = UnixStream::connect(socket_path)?;
    stream.write_all(args.as_ref())?;
    stream.flush()?;
    let mut buffer = [0u8; 100];
    stream.read(&mut buffer)?;
    println!("{}", String::from_utf8_lossy(&buffer));
    Ok(())
}
