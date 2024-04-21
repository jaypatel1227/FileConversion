use once_cell::sync::Lazy;
use std::collections::HashMap;
mod image_conversion;

type ConversionFunction = fn(String) -> String; // this is the type signature of a conversion function

// this is the mapping that is used to define the available conversions that are availiable
static AVAILABLE_CONVERSIONS: Lazy<HashMap<String, ConversionFunction>> = Lazy::new(|| {
    let mut map = HashMap::new();
    // Use a call like below to register a new conversion service
    map.insert("docx_to_pdf".to_string(), docx_to_pdf as ConversionFunction);
    map.insert("png_to_webp".to_string(), png_to_webp as ConversionFunction);
    map
});

pub fn call(func_name: String, file_name: String) -> String {
    if let Some(conversion_func) = AVAILABLE_CONVERSIONS.get(&func_name) {
        dbg!(file_name.clone());
        let result = conversion_func(file_name);
        return result;
    }
    return "".to_string();
}

pub fn png_to_webp(file: String) -> String {
    let out_path = file
        .clone()
        .replace("./input", "./output")
        .replace(".png", ".webp")
        .to_string();
    dbg!(out_path.clone());
    return image_conversion::convert_image(&file, out_path);
}

pub fn docx_to_pdf(file: String) -> String {
    return file.replace(".docx", ".pdf").to_string();
}
