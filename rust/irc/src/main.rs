use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpStream,
};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    // Server to connect to
    server: String,
    // nickname
    nick: String,
    #[arg(short, long)]
    port: Option<u16>,
    #[arg(short, long)]
    channel: Option<String>,
    #[arg(short, long)]
    username: Option<String>,
    #[arg(short, long)]
    realname: Option<String>,
}

const DEFAULT_PORT: u16 = 6667;

fn main() {
    let cli = Cli::parse();
    connect_to_server(
        &cli.server,
        cli.port.unwrap_or(DEFAULT_PORT),
        &cli.nick,
        &cli.username,
        &cli.realname,
    );
}

fn connect_to_server(
    server: &str,
    port: u16,
    nick: &String,
    username: &Option<String>,
    realname: &Option<String>,
) {
    // TODO add port
    let stream = TcpStream::connect((server, port)).unwrap();
    let out_stream = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(out_stream);
    write!(writer, "NICK {}\r\n", nick).unwrap();
    let user = username.as_deref().unwrap_or(nick);
    let name = realname.as_deref().unwrap_or(nick);
    write!(writer, "USER {} 0 * :{}\r\n", user, name).unwrap();
    writer.flush().unwrap();

    for line in reader.lines() {
        let l = line.unwrap();
        if (l.contains("PING")) {
            write!(writer, "PONG")
        }
        println!("{}", l);
    }
}
