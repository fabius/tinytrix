use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct ClientResponse {
    #[serde(rename = "m.homeserver")]
    homeserver: Homeserver,
}

#[derive(Serialize, Deserialize, Debug)]
struct Homeserver {
    base_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerResponse {
    #[serde(rename = "m.server")]
    server: String,
}

const HOSTNAME: &str = "MX_HOSTNAME";

#[get("/.well-known/matrix/client")]
async fn federated_client() -> impl Responder {
    let resp = ClientResponse {
        homeserver: Homeserver {
            base_url: format!("https://{}", env::var(HOSTNAME).unwrap()),
        },
    };
    println!("/client responding: {:?}", resp);
    return HttpResponse::Ok().json(resp);
}

#[get("/.well-known/matrix/server")]
async fn federated_server() -> impl Responder {
    let resp = ServerResponse {
        server: format!("{}:443", env::var(HOSTNAME).unwrap()),
    };
    println!("/server responding: {:?}", resp);
    return HttpResponse::Ok().json(resp);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match env::var(HOSTNAME) {
        Err(_) => {
            panic!("{} environment variable not set", HOSTNAME);
        }
        Ok(host) => {
            println!("Hostname: {}", host);
            HttpServer::new(|| {
                App::new()
                    .service(federated_server)
                    .service(federated_client)
            })
            .bind("0.0.0.0:8080")?
            .run()
            .await
        }
    }
}
