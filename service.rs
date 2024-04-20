use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ServerConfig {
    service_name: String,
    available_name: Vec<AvialableService>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AvialableService {
    name: String,
    service_func_name: String,
}

fn get_server_config() -> ServerConfig {
    let SERVER_CONFIG_NAME = Path::new("../../server.config");
    let file =
        File::open(SERVER_CONFIG_NAME).expect("No server.config not found at the specified path.");

    let json: ServerConfig =
        serde_json::from_reader(file).expect("The Server's JSON config is malformed");
    return json;
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 5001))?
    .run()
    .await
}
