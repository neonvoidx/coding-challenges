use lb::{Server, ThreadPool};
use std::{
    io::{BufReader, prelude::*},
    net::{Ipv4Addr, TcpListener, TcpStream},
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    let pool = ThreadPool::new(4);

    let servers: Vec<Server> = vec![
        Server::new(Ipv4Addr::new(127, 0, 0, 1), 8080),
        Server::new(Ipv4Addr::new(127, 0, 0, 1), 8081),
        Server::new(Ipv4Addr::new(127, 0, 0, 1), 8082),
    ];

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    thread::sleep(std::time::Duration::from_secs(3));
    stream.write_all(response.as_bytes()).unwrap();
}
