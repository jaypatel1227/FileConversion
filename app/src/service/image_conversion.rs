use image::io::Reader as ImageReader;

pub fn convert_image(from: &String, into: String) -> String {
    // Use the open function to load an image from a Path.
    // ```open``` returns a dynamic image.
    dbg!(from);
    dbg!(into.clone());
    let input_image = ImageReader::open(from)
        .expect("Error opening the input file.")
        .decode()
        .unwrap();
    // Write the contents of this image using extension guessing.
    input_image
        .save(into.clone())
        .expect("Error saving the output file.");
    return into;
}
