use lb::{Server, ThreadPool};
use std::{
    io::{self},
    net::{Ipv4Addr, Shutdown, SocketAddrV4, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    let pool = ThreadPool::new(4);

    let servers = Arc::new(vec![
        Server::new(Ipv4Addr::new(127, 0, 0, 1), 8000),
        Server::new(Ipv4Addr::new(127, 0, 0, 1), 8001),
        Server::new(Ipv4Addr::new(127, 0, 0, 1), 8002),
    ]);
    let next_index = Arc::new(Mutex::new(0usize));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let servers = Arc::clone(&servers);
        let next_index = Arc::clone(&next_index);

        pool.execute(move || {
            let current = {
                let mut idx = next_index.lock().unwrap();
                let current = *idx % servers.len();
                *idx += 1;
                current
            };
            println!(
                "Forwarding to backend #{current} at {}:{}",
                servers[current].address, servers[current].port
            );
            if let Err(err) = handle_connection(stream, &servers[current]) {
                eprintln!("connection failed: {err}");
            }
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut client: TcpStream, server: &Server) -> io::Result<()> {
    let backend_addr = SocketAddrV4::new(server.address, server.port);
    // Connect to backend
    let mut backend = TcpStream::connect(backend_addr)?;
    let mut client_reader = client.try_clone()?;
    let mut backend_writer = backend.try_clone()?;
    // Spawn a thread and forward data from client to backend
    let upstream = thread::spawn(move || -> io::Result<()> {
        io::copy(&mut client_reader, &mut backend_writer)?;
        backend_writer.shutdown(Shutdown::Write)?;
        Ok(())
    });
    io::copy(&mut backend, &mut client)?;
    client.shutdown(Shutdown::Write)?;
    upstream
        .join()
        .expect("upstream forwarding thread panicked")?;
    Ok(())
}
