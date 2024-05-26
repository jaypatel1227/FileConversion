use image::io::Reader as ImageReader;

pub fn convert_image(from: &String, into: String) -> Result<String, Box<dyn std::error::Error>> {
    let input_image = ImageReader::open(from)
        .expect("Error opening the input file.")
        .decode()?;
    input_image
        .save(into.clone())
        .expect("Error saving the output file.");
    return Ok(into);
}
