use once_cell::sync::Lazy;
use std::collections::HashMap;

use self::image_conversion::{
    generic_to_bmp, generic_to_gif, generic_to_ico, generic_to_jpg, generic_to_png,
};
mod image_conversion;

type ConversionFunction = fn(String) -> Result<String, Box<dyn std::error::Error>>; // this is the type signature of a conversion function

// this is the mapping that is used to define the available conversions that are availiable
static AVAILABLE_CONVERSIONS: Lazy<HashMap<String, ConversionFunction>> = Lazy::new(|| {
    let mut map = HashMap::new();
    // Use a call like below to register a new conversion service
    map.insert("docx_to_pdf".to_string(), docx_to_pdf as ConversionFunction);
    map.insert("webp_to_png".to_string(), generic_img_conversion("png"));
    map.insert("jpg_to_png".to_string(), generic_img_conversion("png"));
    map.insert("gif_to_png".to_string(), generic_img_conversion("png"));
    map.insert("bmp_to_png".to_string(), generic_img_conversion("png"));
    map.insert("ico_to_png".to_string(), generic_img_conversion("png"));
    map.insert("png_to_webp".to_string(), generic_img_conversion("webp"));
    map.insert("jpg_to_webp".to_string(), generic_img_conversion("webp"));
    map.insert("gif_to_webp".to_string(), generic_img_conversion("webp"));
    map.insert("bmp_to_webp".to_string(), generic_img_conversion("webp"));
    map.insert("ico_to_webp".to_string(), generic_img_conversion("webp"));
    map.insert("png_to_jpg".to_string(), generic_img_conversion("jpg"));
    map.insert("webp_to_jpg".to_string(), generic_img_conversion("jpg"));
    map.insert("gif_to_jpg".to_string(), generic_img_conversion("jpg"));
    map.insert("bmp_to_jpg".to_string(), generic_img_conversion("jpg"));
    map.insert("ico_to_jpg".to_string(), generic_img_conversion("jpg"));
    map.insert("png_to_gif".to_string(), generic_img_conversion("gif"));
    map.insert("webp_to_gif".to_string(), generic_img_conversion("gif"));
    map.insert("jpg_to_gif".to_string(), generic_img_conversion("gif"));
    map.insert("bmp_to_gif".to_string(), generic_img_conversion("gif"));
    map.insert("ico_to_gif".to_string(), generic_img_conversion("gif"));
    map.insert("png_to_bmp".to_string(), generic_img_conversion("bmp"));
    map.insert("webp_to_bmp".to_string(), generic_img_conversion("bmp"));
    map.insert("jpg_to_bmp".to_string(), generic_img_conversion("bmp"));
    map.insert("gif_to_bmp".to_string(), generic_img_conversion("bmp"));
    map.insert("ico_to_bmp".to_string(), generic_img_conversion("bmp"));
    map.insert("png_to_ico".to_string(), generic_img_conversion("ico"));
    map.insert("webp_to_ico".to_string(), generic_img_conversion("ico"));
    map.insert("jpg_to_ico".to_string(), generic_img_conversion("ico"));
    map.insert("gif_to_ico".to_string(), generic_img_conversion("ico"));
    map.insert("bmp_to_ico".to_string(), generic_img_conversion("ico"));
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

pub fn generic_img_conversion(out_extension: &str) -> ConversionFunction {
    match out_extension {
        "png" => generic_to_png,
        "webp" => to_webp, // currently using this to_webp function rather than the generic that
        // uses image_convert since there are some issues with that supporting webp
        "jpg" => generic_to_jpg,
        "gif" => generic_to_gif,
        "bmp" => generic_to_bmp,
        "ico" => generic_to_ico,
        _ => default_img_conversion,
    }
}

// this should work for arbitrary inputs so that's why it is the default
pub fn default_img_conversion(file: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(image_conversion::convert_image(&file, "webp")?)
}

pub fn to_webp(file: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(image_conversion::convert_image(&file, "webp")?)
}

pub fn docx_to_pdf(file: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(file.replace(".docx", ".pdf").to_string())
}
