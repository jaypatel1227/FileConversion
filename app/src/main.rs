use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
mod service;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
struct ServerConfig {
    service_name: String,
    available_services: Vec<AvialableService>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AvialableService {
    name: String,
    service_func_name: String,
}

fn get_server_config() -> ServerConfig {
    let server_config = Path::new("../server.config");
    let file = File::open(server_config)
        .expect("No server.config not found at the top level of the project");

    let json: ServerConfig =
        serde_json::from_reader(file).expect("The Server's JSON config is malformed");

    return json;
}

#[get("/available_options")]
async fn get_options() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*")) // to stop CORS error when running the
        // site and service from the same domain (such as localhost)
        .json(get_server_config())
    // Ok(web::Json(get_server_config().into()));
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
            .service(get_options)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 5001))?
    .run()
    .await
}
