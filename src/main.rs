use std::net::SocketAddr;

use clap::Parser;
use client::Client;
use tokio::net::TcpStream;

mod client;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    ip: String,
    #[arg(short, long, default_value_t = 1)]
    count: u32,
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let args = Args::parse();
    let address = args.ip.parse::<SocketAddr>().unwrap();
    log::info!("{} Bots will Join {}", args.count, address);
    let mut join_handles = Vec::with_capacity(args.count as usize);

    for i in 0..args.count {
        let stream = TcpStream::connect(address)
            .await
            .expect("Failed to connect to Ip");
        let client = Client::new(stream);
        client.join_server(address, format!("BOT_{}", i)).await;
        let join_handle = tokio::spawn(async move {
            loop {
                if !client.poll().await {
                    break;
                }
                client.process_packets().await;
            }
        });
        join_handles.push(join_handle);
        log::info!("{}/{} Bots Joined", i + 1, args.count);
    }
    // Graceful shutdown on Ctrl+C
    tokio::signal::ctrl_c().await.unwrap();
    log::info!("Shutting down...");

    // Wait for all bot tasks to exit
    for handle in join_handles {
        handle.abort();
    }
}
