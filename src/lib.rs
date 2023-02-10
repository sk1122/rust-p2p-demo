// 1. start a client with a trusted client of the network, if don't have any previous trusted client, current client will be the only node

// 2. get list of all available clients in the network and start connecting with them

// 3. connect with each peer and request configuration from them

// 4. verify it, if it is correct, add that peer to the local peer list

// 5. every client gossips with each other every 10mins and updates their local peer list

// 6. gossiping means that it will select few random nodes from the network, which it hadn't selected in previous few rounds

use std::{time::SystemTime, collections::HashMap};
use rand::Rng;
use helpers::connect::{connect_with_trusted_client, connect_with_peer, send_body_to_peer, Method, MethodInnerData};
use peers::peers::Peer;
use serde::{Serialize, Deserialize};
use serde_json::Value;

const FANOUT_VALUE: u32 = 4;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    pub trusted_client: String,
    pub port: u32,
    pub address: String,
    pub last_synced: SystemTime,
    pub peers: Vec<Peer>,
    pub peer_addresses: Vec<String>
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

        // connected_peers.push(Peer {
        //     address: format!("http://0.0.0.0:{}", port),
        //     connected: true,
        //     last_connected: SystemTime::now()
        // });

        let client = &mut Client {
            port,
            trusted_client: trusted_client.to_string(),
            address: "".into(),
            last_synced: SystemTime::now(),
            peers: connected_peers.clone(),
            peer_addresses: connected_peers.iter().map(|x| x.address.clone()).collect()
        };

        let con_peers = &mut client.start_gossip(&Peer {
            address: format!("http://0.0.0.0:{}", port),
            connected: true,
            last_connected: SystemTime::now()
        }).await;

        client.peers.append(con_peers);

        std::fs::write(format!("./config-{}.json", port), serde_json::to_string_pretty(&client).unwrap()).unwrap();

        client.clone()
    }

    pub fn new_without_trusted_client(port: u32) -> Self {
        let client = Self {
            port,
            trusted_client: "".into(),
            address: format!("http://0.0.0.0:{}", port),
            last_synced: SystemTime::now(),
            peers: vec![],
            peer_addresses: vec![]
        };

        client.update_db(&port);
    
        client
    }

    pub async fn connect_with_new_peer(&mut self, peer_addr: &String) -> bool {
        let connected = connect_with_peer(&peer_addr).await.unwrap();

        self.peers.push(Peer {
            address: peer_addr.to_string(),
            connected: true,
            last_connected: SystemTime::now()
        });

        connected
    }

    pub async fn start_gossip(&mut self, peer: &Peer) -> Vec<Peer> {
        if self.address == peer.address {
            return vec![];
        }
        
        if self.peer_addresses.contains(&peer.address) {
            return vec![];
        }
        
        let mut rng = rand::thread_rng();

        let mut peers: Vec<Peer> = vec![];
        
        println!("here 1");
        let connected = connect_with_peer(&peer.address).await.unwrap();
        if connected {
            self.peers.push(peer.clone());
            self.peer_addresses.push(peer.address.clone());
            self.update_db(&self.port.clone());
            println!("here 2");
        }

        for _ in 0..4 {
            let random_number = rng.gen_range(0..self.peers.len());

            peers.push(self.peers[random_number].clone());
        }

        let mut connected_peers: Vec<Peer> = vec![];

        for single_peer in peers {
            let mut body: HashMap<String, Value> = HashMap::new();

            body.insert("address".into(), Value::String(peer.address.clone()));

            let sent = send_body_to_peer(&single_peer, &Method {
                method: "introduce_peer".into(),
                params: vec![MethodInnerData::Object(body)]
            }).await;

            if sent {
                connected_peers.push(single_peer);
            }
        }

        connected_peers
    }

    pub fn update_db(&self, port: &u32) {
        std::fs::write(format!("./config-{}.json", port), serde_json::to_string_pretty(&self).unwrap()).unwrap();
    }
}

pub mod helpers;
pub mod peers;
pub mod client;