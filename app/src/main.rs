mod converters;
mod service;
use actix_files::Files;
use actix_multipart::Multipart;
use actix_web::{get, post, web, App, HttpResponse, HttpResponseBuilder, HttpServer, Responder};

#[get("/available_options")]
async fn get_options() -> impl Responder {
    HttpResponse::Ok()
        .conditionally_compiled_settings()
        .json(service::get_server_config())
}

#[post("/convert_file/{conversion_type}/")]
async fn convert_file(path: web::Path<String>, payload: Multipart) -> impl Responder {
    let resp = service::convert_file_core(path, payload).await;
    HttpResponse::Ok()
        .conditionally_compiled_settings()
        .json(resp.unwrap())
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok()
        .conditionally_compiled_settings()
        .body(req_body)
}

#[get("/stats")]
async fn get_stats(req_body: String) -> impl Responder {
    HttpResponse::Ok()
        .conditionally_compiled_settings()
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

// used to add additional methods for the HttpResponseBuilder type
trait HttpRespUtils {
    fn conditionally_compiled_settings(&mut self) -> &mut Self;
}

impl HttpRespUtils for HttpResponseBuilder {
    // include override for CORS when debugging locally, but do not when running in optimized build
    fn conditionally_compiled_settings(&mut self) -> &mut Self {
        if cfg!(debug_assertions) {
            return self.insert_header(("Access-Control-Allow-Origin", "*"));
        }
        self
    }
}
