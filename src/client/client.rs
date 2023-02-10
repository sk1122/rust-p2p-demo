use std::{net::SocketAddr, str::FromStr, time::SystemTime};

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, body::BoxBody, http::header::{ContentType, self}, ResponseError, http::StatusCode, HttpRequest, HttpMessage, post};
use derive_more::{Display, Error};
use serde_json::Value;

use crate::{helpers::connect::{Config, Method, MethodInnerData}, Client, peers::peers::Peer};

#[macro_export]
macro_rules! extract_enum_value {
  ($value:expr, $pattern:pat => $extracted_value:expr) => {
    match $value {
      $pattern => $extracted_value,
      _ => panic!("Pattern doesn't match!"),
    }
  };
}

#[derive(Debug, Display, Error)]
#[display(fmt = "{}", error_message)]
struct ServerError {
    error_message: String,
    status: StatusCode
}

impl ResponseError for ServerError {}

#[get("/config")]
async fn pol(req_body: HttpRequest) -> impl Responder {
    let req_result = Config {
        version: "1.0".into(),
        peer_address: req_body.app_config().local_addr().to_string()
    };

    // let host_header = req_body.headers().get("host").unwrap().to_str().unwrap();
    // let host_and_port: Vec<&str> = host_header.split(':').collect();
    // let port = host_and_port[1].parse::<u32>().unwrap();
    // println!("The port is {}", port);

    // let client = &mut Client::new(&"".into(), port).await;
    // client.connect_with_new_peer(req_body.app);

    let response = serde_json::to_string(&req_result).unwrap();

    HttpResponse::Ok().body(response)
}

#[get("/get-peers")]
async fn get_peers(req_body: HttpRequest) -> impl Responder {
    let host_header = req_body.headers().get("host").unwrap().to_str().unwrap();
    let host_and_port: Vec<&str> = host_header.split(':').collect();
    let port = host_and_port[1].parse::<u32>().unwrap();
    println!("The port is {}", port);

    let client = Client::new(&"".into(), port).await;

    let response = serde_json::to_string(&client.peers).unwrap();

    HttpResponse::Ok().body(response)
}

#[post("/v1")]
async fn execute_method(data: web::Json<Method>, req: HttpRequest) -> impl Responder {
    println!("request received");
    
    let body = data.into_inner();

    println!("{:?}", body);

    let host_header = req.headers().get("host").unwrap().to_str().unwrap();
    let host_and_port: Vec<&str> = host_header.split(':').collect();
    let port = host_and_port[1].parse::<u32>().unwrap();

    let mut client = Client::new(&"".into(), port).await;

    let data = extract_enum_value!(&body.params[0], MethodInnerData::Object(x) => x);
    let address = extract_enum_value!(data.get("address".into()).unwrap(), Value::String(x) => x);
    let peer = Peer {
        address: address.clone(),
        connected: true,
        last_connected: SystemTime::now()
    };

    println!("{:?}", peer);

    let response = client.start_gossip(&peer).await;

    HttpResponse::Ok().append_header(("content-type", "application/json")).body(serde_json::to_string_pretty(&response).unwrap())
}

pub async fn run_server(port: u32) -> std::io::Result<()> {
    println!("Running server on port {}🎉", port);
    
    HttpServer::new(|| {
        App::new()
            .service(execute_method)
            .service(pol)
            .service(get_peers)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}