mod service;
use actix_files::Files;
use actix_multipart::Multipart;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder, Result};
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize, Serialize)]
struct ServerConfig {
    service_name: String,
    available_services: Vec<AvialableService>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AvialableService {
    name: String,
    service_func_name: String,
    description: String,
}

#[derive(Debug, Serialize)]
struct ConversionResponse {
    file_name: String,
    success: bool,
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
}

#[post("/convert_file/{conversion_type}/")]
async fn convert_file(
    path: web::Path<String>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    let mut filename = "".to_string();
    let time_stamp = format!(
        "{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros()
    );
    dbg!(time_stamp.clone());
    dbg!(path.clone());

    while let Ok(Some(mut field)) = payload.try_next().await {
        let field_type = field.content_disposition().get_name().unwrap();
        if field_type != "file" {
            continue;
        }
        filename = format!(
            "{}-{}",
            time_stamp,
            sanitize_filename::sanitize(field.content_disposition().get_filename().unwrap())
        );

        fs::create_dir_all("./input/")?;
        let filepath = format!("{}{}", "./input/", sanitize_filename::sanitize(&filename));
        dbg!(filepath.clone());
        // File::create is blocking operation, use thread pool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();

        dbg!(field_type);
        dbg!(filename.clone());

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use thread pool
            f = web::block(move || {
                f.as_mut()
                    .expect("Error reading the file.")
                    .write_all(&data)
                    .map(|_| f)
            })
            .await?
            .expect("Error when writing the data.");
        }
    }

    let conversion_type = path.into_inner();
    let matching_service = get_server_config()
        .available_services
        .into_iter()
        .find(|s| s.name == conversion_type);

    dbg!(conversion_type.clone());

    match matching_service {
        None => return Ok(HttpResponse::BadRequest().body("Invalid Conversion Request!")),
        Some(service_info) => {
            dbg!(filename.clone());
            fs::create_dir_all("./output/")?;
            let converted_file = service::call(
                service_info.service_func_name,
                format!("{}{}", "./input/", filename),
            );

            // dbg!(service_info.service_func_name.clone());
            dbg!(service_info.name.clone());
            dbg!(converted_file.clone());

            let resp = ConversionResponse {
                file_name: converted_file,
                success: true,
            };

            return Ok(HttpResponse::Ok()
                .insert_header(("Access-Control-Allow-Origin", "*"))
                .json(resp));
        }
    }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .body(req_body)
}

// async fn index(req: HttpRequest) -> actix_web::Result<NamedFile> {
//     let path: PathBuf = req.match_info().query("filename").parse().unwrap();
//     Ok(NamedFile::open(path)?)
// }

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_options)
            .service(convert_file)
            .service(Files::new("/output", "./output").prefer_utf8(true)) // make the files under output accessible
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
        // .route("/{filename:.*}", web::get().to(index))
    })
    .bind(("127.0.0.1", 5001))?
    .run()
    .await
}
