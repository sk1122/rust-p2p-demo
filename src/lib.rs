// 1. start a client with a trusted client of the network, if don't have any previous trusted client, current client will be the only node

// 2. get list of all available clients in the network and start connecting with them

// 3. connect with each peer and request configuration from them

// 4. verify it, if it is correct, add that peer to the local peer list

// 5. every client gossips with each other every 10mins and updates their local peer list

// 6. gossiping means that it will select few random nodes from the network, which it hadn't selected in previous few rounds

use std::time::SystemTime;

use helpers::connect::{connect_with_trusted_client, connect_with_peer};
use peers::peers::Peer;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    pub trusted_client: String,
    pub address: String,
    pub last_synced: SystemTime,
    pub peers: Vec<Peer>
}

impl Client {
    pub async fn new(trusted_client: &String, port: u32) -> Self {
        let text_result = std::fs::read_to_string(format!("./config-{}.json", port));

        match text_result {
            Ok(text) => {
                if text.len() > 0 {
                    let config = serde_json::from_str::<Client>(&text).unwrap();
                    
                    if (config.trusted_client.eq(trusted_client) && config.last_synced.elapsed().unwrap().as_secs() <= 60) || (trusted_client.eq("".into())) || (port == 0) {
                        return config
                    }
                }
            },
            Err(_) => println!("Not found a cached file")
        }

        
        let peers = connect_with_trusted_client(&trusted_client).await;

        let mut connected_peers = vec![];

        for peer in &peers {
            println!("{:?}", peer);

            let connected = connect_with_peer(&peer.address).await.unwrap();

            if connected {
                connected_peers.push(peer.clone())
            }
        }

        connected_peers.push(Peer {
            address: format!("http://0.0.0.0:{}", port),
            connected: true,
            last_connected: SystemTime::now()
        });

        let client = Client {
            trusted_client: trusted_client.to_string(),
            address: "".into(),
            last_synced: SystemTime::now(),
            peers: connected_peers
        };

        std::fs::write(format!("./config-{}.json", port), serde_json::to_string_pretty(&client).unwrap()).unwrap();

        client
    }

    pub fn new_without_trusted_client(port: u32) -> Self {
        let client = Self {
            trusted_client: "".into(),
            address: format!("http://0.0.0.0:{}", port),
            last_synced: SystemTime::now(),
            peers: vec![Peer {
                address: format!("http://0.0.0.0:{}", port),
                connected: true,
                last_connected: SystemTime::now()
            }]
        };
    
        std::fs::write(format!("./config-{}.json", port), serde_json::to_string_pretty(&client).unwrap()).unwrap();

        client
    }
}

pub mod helpers;
pub mod peers;
pub mod client;