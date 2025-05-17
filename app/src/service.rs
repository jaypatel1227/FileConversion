use crate::converters;
use actix_multipart::Multipart;
use actix_web::{web, Error, Result};
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
pub struct ConversionResponse {
    file_name: String,
    success: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerStats {
    inp_file_count: u32,
    out_file_count: u32,
    tot_file_size: u64,
}

pub fn get_server_config() -> ServerConfig {
    let server_config = Path::new("./server_config.json");
    let file = File::open(server_config)
        .expect("No server.config not found at the top level of the project");

    let json: ServerConfig =
        serde_json::from_reader(file).expect("The Server's JSON config is malformed");

    return json;
}

pub async fn service_stats(_req_body: String) -> ServerStats {
    ServerStats {
        tot_file_size: 100,
        inp_file_count: 10,
        out_file_count: 10,
    }
}

pub async fn convert_file_core(
    path: web::Path<String>,
    mut payload: Multipart,
) -> Result<ConversionResponse, Error> {
    // do onetime file creation
    let mut done_initialization: bool = false;
    let mut file: Option<std::fs::File> = None;
    let mut file_name: String = "".to_string();

    // stream pieces of the file that the user is uploading
    while let Ok(Some(mut field)) = payload.try_next().await {
        // make sure it is actally a file!
        let field_type = field.content_disposition().get_name().unwrap();
        if field_type != "file" {
            continue;
        }

        // do the one time initialization
        if !done_initialization {
            file_name =
                clean_file_name(field.content_disposition().get_filename().unwrap()).to_string();
            file = Some(initialize_input(file_name.clone()).await?);
            done_initialization = true;
        }

        // grab the next chunk of data
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            if file.is_none() {
                // shouldn't be possible but in case initialization of file failed
                continue;
            } else {
                // filesystem operations are blocking, we have to use thread pool
                file = Some(
                    web::block(move || {
                        file.as_mut()
                            .unwrap()
                            //.expect("Error reading the file.")
                            .write_all(&data)
                            .map(|_| file.unwrap())
                    })
                    .await
                    .unwrap()
                    .expect("Error writing the data to server."),
                );
            }
        }
    }

    // once we have to file, do the conversion!
    Ok(do_conversion(path, file_name)?)
}

pub fn clean_file_name(unclean_file_name: &str) -> String {
    let time_stamp = format!(
        "{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros()
    );
    format!(
        "{}-{}",
        time_stamp,
        sanitize_filename::sanitize(unclean_file_name)
    )
}

pub async fn initialize_input(file_name: String) -> Result<File, Box<dyn std::error::Error>> {
    fs::create_dir_all("./input/")?;
    let filepath = format!("{}{}", "./input/", &file_name);
    // File::create is blocking operation, use thread pool
    let f = web::block(|| std::fs::File::create(filepath))
        .await
        .unwrap()?;
    Ok(f)
}

pub fn do_conversion(
    path: web::Path<String>,
    file_name: String,
) -> Result<ConversionResponse, Box<dyn std::error::Error>> {
    let conversion_type = path.into_inner();
    let matching_service = get_server_config()
        .available_services
        .into_iter()
        .find(|s| s.name == conversion_type);

    match matching_service {
        None => {
            return Ok(ConversionResponse {
                file_name: "".to_string(),
                success: false,
            })
        }
        Some(service_info) => {
            fs::create_dir_all("./output/")?;
            let converted_file = converters::call(
                service_info.service_func_name,
                format!("{}{}", "./input/", file_name),
            )
            .unwrap();

            let resp = ConversionResponse {
                file_name: converted_file.replace("./output", "output"), // convert from relative
                // path to route to fetch
                success: true,
            };

            return Ok(resp);
        }
    }
}
