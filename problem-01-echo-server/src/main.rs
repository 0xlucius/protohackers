//Implementation of a TCP echo server(RFC 862) that echoes back everything it receives.
use::std::net::{TcpListener, TcpStream, Shutdown};
use::std::io::{Read, Write, ErrorKind};
use::std::thread;

//echos back everything it receives
fn handle_client(mut stream: TcpStream) -> std::io::Result<()>{
    let peer = stream.peer_addr()?;
    let mut buffer = [0; 1024];

loop {
    match stream.read(&mut buffer) {
        Ok(0) => {
            log::debug!("Connection closed by {}", peer);
            break;
        }
        Ok(bytes_read) => {
            log::debug!("Read {} bytes from {}", bytes_read, peer);
            stream.write_all(&buffer[..bytes_read])?;
            log::debug!("Wrote {} bytes to {}.", bytes_read, peer);
        }
        Err(e) => {
            if e.kind() != ErrorKind::WouldBlock {
                log::error!("Error reading from socket: {}", e);
                break;
            }
        }
    }
}
    log::debug!("Closing connection with {}", peer);
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    env_logger::init();

    let listener = TcpListener::bind("0.0.0.0:10000")?;
    log::info!("Listening on {}", listener.local_addr()?);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                log::debug!("New connection from: {}", stream.peer_addr()?);
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                log::error!("Error: {}", e);
            }
        }
    }
    Ok(())
}
