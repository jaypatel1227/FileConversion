use crate::converters;
use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse, Result};
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    service_name: String,
    available_services: Vec<AvialableService>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AvialableService {
    name: String,
    service_func_name: String,
    description: String,
}

#[derive(Debug, Serialize)]
struct ConversionResponse {
    file_name: String,
    success: bool,
}

pub fn get_server_config() -> ServerConfig {
    let server_config = Path::new("./server_config.json");
    let file = File::open(server_config)
        .expect("No server.config not found at the top level of the project");

    let json: ServerConfig =
        serde_json::from_reader(file).expect("The Server's JSON config is malformed");

    return json;
}

pub async fn convert_file_core(
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
        // File::create is blocking operation, use thread pool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();

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

    match matching_service {
        None => return Ok(HttpResponse::BadRequest().body("Invalid Conversion Request!")),
        Some(service_info) => {
            fs::create_dir_all("./output/")?;
            let converted_file = converters::call(
                service_info.service_func_name,
                format!("{}{}", "./input/", filename),
            )
            .unwrap();

            let resp = ConversionResponse {
                file_name: converted_file.replace("./output", "output"), // convert from relative
                // path to route to fetch
                success: true,
            };

            return Ok(HttpResponse::Ok()
                .insert_header(("Access-Control-Allow-Origin", "*"))
                .json(resp));
        }
    }
}
