use image_convert::{ImageResource, PNGConfig};
use once_cell::sync::Lazy;
use std::collections::HashMap;
mod image_conversion;

type ConversionFunction = fn(String) -> Result<String, Box<dyn std::error::Error>>; // this is the type signature of a conversion function

// this is the mapping that is used to define the available conversions that are availiable
static AVAILABLE_CONVERSIONS: Lazy<HashMap<String, ConversionFunction>> = Lazy::new(|| {
    let mut map = HashMap::new();
    // Use a call like below to register a new conversion service
    map.insert("docx_to_pdf".to_string(), docx_to_pdf as ConversionFunction);
    map.insert("png_to_webp".to_string(), png_to_webp as ConversionFunction);
    map.insert("webp_to_png".to_string(), webp_to_png as ConversionFunction);
    map.insert("bmp_to_png".to_string(), bmp_to_png as ConversionFunction);
    map.insert("jpg_to_png".to_string(), jpg_to_png as ConversionFunction);
    map.insert("gif_to_png".to_string(), gif_to_png as ConversionFunction);
    map.insert("ico_to_png".to_string(), ico_to_png as ConversionFunction);
    map
});

pub fn call(func_name: String, file_name: String) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(conversion_func) = AVAILABLE_CONVERSIONS.get(&func_name) {
        dbg!(file_name.clone());
        let result = conversion_func(file_name);
        return result;
    }
    return Err("Conversion function not defined.".into());
}

fn create_output_path(file: String, current_extension: &str, desired_extenstion: &str) -> String {
    file.clone()
        .replace("./input", "./output")
        .replace(current_extension, desired_extenstion)
        .to_string()
}

fn generic_to_png(
    input_path: String,
    input_extension: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let out_path = create_output_path(input_path.clone(), input_extension, ".png");
    let input = ImageResource::from_path(input_path);
    let mut output = ImageResource::from_path(out_path.clone());
    image_convert::to_png(&mut output, &input, &PNGConfig::new())?;
    Ok(out_path)
}

pub fn png_to_webp(file: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut out_path = create_output_path(file.clone(), ".png", ".webp");
    out_path = image_conversion::convert_image(&file, out_path)?;
    return Ok(out_path);
}

pub fn webp_to_png(file: String) -> Result<String, Box<dyn std::error::Error>> {
    generic_to_png(file, ".webp")
}

pub fn bmp_to_png(file: String) -> Result<String, Box<dyn std::error::Error>> {
    generic_to_png(file, ".bmp")
}

pub fn jpg_to_png(file: String) -> Result<String, Box<dyn std::error::Error>> {
    generic_to_png(file, ".jpg")
}

pub fn gif_to_png(file: String) -> Result<String, Box<dyn std::error::Error>> {
    generic_to_png(file, ".gif")
}

pub fn ico_to_png(file: String) -> Result<String, Box<dyn std::error::Error>> {
    generic_to_png(file, ".ico")
}

pub fn docx_to_pdf(file: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(file.replace(".docx", ".pdf").to_string())
}
