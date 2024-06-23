use image::io::Reader as ImageReader;
use image_convert::{ImageResource, PNGConfig};
use std::path::PathBuf;

const ERR_INPUT_OPEN: &str = "Error opening the input file.";
const ERR_OUTPUT_OPEN: &str = "Error saving the output file.";

fn create_output_path(file: String, current_extension: String, desired_extenstion: &str) -> String {
    file.clone()
        .replace("./input", "./output")
        .replace(&current_extension, desired_extenstion)
        .to_string()
}

fn identify_extension(path: String) -> String {
    let path_buf = PathBuf::from(path);
    path_buf.extension().unwrap().to_str().unwrap().to_string()
}

// pub fn generic_to_webp(input_path: String) -> Result<String, Box<dyn std::error::Error>> {
//     let input_extension = identify_extension(input_path.clone());
//     let out_path = create_output_path(input_path.clone(), input_extension.to_string(), ".webp");
//     let input = ImageResource::from_path(input_path);
//     let mut output = ImageResource::from_path(out_path.clone());
//     image_convert::to_webp(&mut output, &input, &WEBPConfig::new())?;
//     Ok(out_path)
// }

pub fn generic_to_png(input_path: String) -> Result<String, Box<dyn std::error::Error>> {
    let input_extension = identify_extension(input_path.clone());
    let out_path = create_output_path(input_path.clone(), input_extension.to_string(), ".png");
    let input = ImageResource::from_path(input_path);
    let mut output = ImageResource::from_path(out_path.clone());
    image_convert::to_png(&mut output, &input, &PNGConfig::new())?;
    Ok(out_path)
}

pub fn convert_image(from: &String, fmt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let out_path = create_output_path(from.clone(), identify_extension(from.clone()), fmt);
    let input_image = ImageReader::open(from).expect(ERR_INPUT_OPEN).decode()?;
    input_image.save(out_path.clone()).expect(ERR_OUTPUT_OPEN);
    Ok(out_path)
}
