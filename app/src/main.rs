mod converters;
mod service;
use actix_files::Files;
use actix_multipart::Multipart;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/available_options")]
async fn get_options() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*")) // to stop CORS error when running the
        // site and service from the same domain (such as localhost)
        .json(service::get_server_config())
}

#[post("/convert_file/{conversion_type}/")]
async fn convert_file(path: web::Path<String>, payload: Multipart) -> impl Responder {
    let resp = service::convert_file_core(path, payload).await;
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .json(resp.unwrap())
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .body(req_body)
}

#[get("/stats")]
async fn get_stats(req_body: String) -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .json(service::service_stats(req_body).await)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_options)
            .service(convert_file)
            .service(Files::new("/output", "./output").prefer_utf8(true)) // make the files under output accessible
            .service(echo)
        // .route("/{filename:.*}", web::get().to(index))
    })
    .bind(("0.0.0.0", 5001))?
    .run()
    .await
}
