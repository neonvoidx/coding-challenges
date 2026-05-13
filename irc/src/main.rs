use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    // Server to connect to
    server: String,
    // (optional) nickname
    nick: Option<String>,
    #[arg(short, long)]
    port: Option<u32>,
    #[arg(short, long)]
    channel: Option<String>,
    #[arg(short, long)]
    username: Option<String>,
    #[arg(short, long)]
    realname: Option<String>,
}

const DEFAULT_PORT: u32 = 6667;

fn main() {
    let cli = Cli::parse();
}

fn connect_to_server(server: &str, port: Option<u32>) {}
