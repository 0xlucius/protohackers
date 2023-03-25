//Implementation of a TCP echo server(RFC 862) that echoes back everything it receives.
use::std::net::{TcpListener, TcpStream, Shutdown};
use::std::io::{Read, Write};
use::std::thread;

//echos back everything it receives
fn handle_client(mut stream: TcpStream) -> std::io::Result<()>{
    let mut buffer = vec![];
    loop {
        let bytes_read = stream.read_to_end(&mut buffer)?;

        stream.write_all(&buffer[..bytes_read])?;

        stream.shutdown(Shutdown::Both)?;
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:7")?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    Ok(())
}
