use std::{time::SystemTime, io::Result, collections::HashMap, error::Error, hash::Hash};
use serde::{Serialize, Deserialize};
use serde_json::{Map, Value};
use crate::peers::peers::Peer;
use reqwest::Client;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub peer_address: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MethodInnerData {
    Text(String),
    TextArray(Vec<String>),
    NumberArray(Vec<u64>),
    Boolean(bool),
    Object(HashMap<String, Value>)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Method {
    pub method: String,
    pub params: Vec<MethodInnerData>
}

pub async fn connect_with_trusted_client(addr: &String) -> Vec<Peer> {
    let client = Client::builder().build().unwrap();
    let mut mapp = HashMap::new();

    mapp.insert("id", "1.0");

    // let res = client.post(addr).json(&mapp).send().await.unwrap();
    
    // let peer = Peer {
    //     address: "".into(),
    //     connected: true,
    //     last_connected: SystemTime::now()
    // };

    let res = client.get(format!("{}/get-peers", addr)).send().await.unwrap();

    let peers = res.json::<Vec<Peer>>().await.unwrap();

    return peers;
}

pub async fn connect_with_peer(peer_address: &String) -> Result<bool> {
    let client = Client::builder().build().unwrap();
    // let mut mapp = HashMap::new();

    println!("{}", peer_address);

    let res = client.get(format!("{}/config", peer_address)).send().await.unwrap();

    let body = res.json::<Config>().await.unwrap();

    if body.peer_address.eq(peer_address)  {
        return Ok(true);
    }

    if body.version.eq("1.0".into()) {
        return Ok(true)
    }
    
    Ok(false)
}

pub async fn send_body_to_peer(peer: &Peer, data: &Method) -> bool {
    let body_string = serde_json::to_string_pretty(data).unwrap();

    println!("{}", body_string);
    println!("{}", format!("{}/v1", peer.address));
    
    let client = Client::new();

    let res = client.post(format!("{}/v1", peer.address)).body(body_string).header("content-type", "application/json").send().await.unwrap();

    println!("{:?}", res.status());
    // let body = res.json::<Vec<Peer>>().await.unwrap();
    println!("{:?}", res.text().await.unwrap());

    // let connected = body.get("connected".into()).unwrap();

    return true
}