use std::{net::SocketAddr, str::FromStr};

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, body::BoxBody, http::header::ContentType, ResponseError, http::StatusCode, HttpRequest, HttpMessage};
use derive_more::{Display, Error};

use crate::{helpers::connect::Config, Client};

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

    let response = serde_json::to_string(&req_result).unwrap();

    HttpResponse::Ok().body(response)
}

#[get("get-peers")]
async fn get_peers(req_body: HttpRequest) -> impl Responder {
    let host_header = req_body.headers().get("host").unwrap().to_str().unwrap();
    let host_and_port: Vec<&str> = host_header.split(':').collect();
    let port = host_and_port[1].parse::<u32>().unwrap();
    println!("The port is {}", port);

    let client = Client::new(&"".into(), port).await;

    let response = serde_json::to_string(&client.peers).unwrap();

    HttpResponse::Ok().body(response)
}

pub async fn run_server(port: u32) -> std::io::Result<()> {
    println!("Running server on port {}🎉", port);
    
    HttpServer::new(|| {
        App::new()
            .service(pol)
            .service(get_peers)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}