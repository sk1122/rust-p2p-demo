use std::time::SystemTime;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Peer {
    pub address: String,
    pub connected: bool,
    pub last_connected: SystemTime
}

impl Peer {
    pub fn new(port: u32) -> Self {
        Peer {
            address: format!("0.0.0.0:{}", port),
            connected: true,
            last_connected: SystemTime::now()
        }
    }
}

#[derive(Clone)]
pub struct ConnectedPeers {
    pub connected_peers: Vec<Peer>
}

impl ConnectedPeers {
    pub fn new() -> Self {
        ConnectedPeers { connected_peers: vec![] }
    }

    pub fn add_peer(&mut self, peer: &Peer) {
        self.connected_peers.push(peer.clone());
    }

    pub fn get_peers(self) -> Vec<Peer> {
        self.connected_peers
    }
}