use std::env;

use rust_p2p_2::{client::client::run_server, Client};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 3 {
        if args[2].eq("-p") {
            Client::new(&args[3], args[1].parse::<u32>().unwrap()).await;
        }
    } else {
        Client::new_without_trusted_client(args[1].parse::<u32>().unwrap());
    }

    run_server(args[1].parse::<u32>().unwrap()).await.unwrap();
}